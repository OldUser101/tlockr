// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
        render.cpp:
                This file contains the QML rendering logic.
*/

#include "render.hpp"
#include "event_handler.hpp"
#include "interface.hpp"
#include "keyboard_repeat.hpp"
#include "logging.hpp"

static const char *FILENAME = "tlockr_qt/render.cpp";

#ifdef __cplusplus
extern "C" {
#endif

int writeEvent(int fd, EventType event_type, EventParam param_1,
               EventParam param_2) {
    Event ev = {
        event_type,
        param_1,
        param_2,
    };

    ssize_t res = write(fd, &ev, sizeof(Event));
    if (res != sizeof(Event)) {
        error_log(
            FILENAME,
            format_log("Failed to write event: ", strerror(errno)).c_str());
        return -1;
    }

    return 0;
}

QmlRenderer *initialize_renderer(int width, int height, const char *qmlPath,
                                 ApplicationState *appState) {
    QmlRenderer *renderer = new QmlRenderer();
    renderer->fbSize = QSize(width, height);
    renderer->qmlPath = qmlPath;
    renderer->appState = appState;

    renderer->keyboardRepeatEngine = new KeyboardRepeatEngine(renderer);
    renderer->eventHandler =
        new EventHandler(renderer, renderer->keyboardRepeatEngine);

    return renderer;
}

void set_deinitialize(QmlRenderer *renderer) {
    {
        std::lock_guard<std::mutex> lock(renderer->initMutex);
        renderer->initialized = false;
    }
    renderer->initCondition.notify_one();
}

void set_initialize(QmlRenderer *renderer) {
    {
        std::lock_guard<std::mutex> lock(renderer->initMutex);
        renderer->initialized = true;
    }
    renderer->initCondition.notify_one();
}

void setup_event_socket(QmlRenderer *renderer) {
    // Set non-blocking I/O on the read file descriptor, required by
    // QSocketNotifier
    int fd = renderer->appState->rendererReadFd;
    fcntl(fd, F_SETFL, O_NONBLOCK);

    QSocketNotifier *notifier = new QSocketNotifier(fd, QSocketNotifier::Read);

    notifier->setEnabled(true);

    QObject::connect(notifier, &QSocketNotifier::activated, [renderer](int) {
        renderer->eventHandler->handleReceivedEvent();
    });

    renderer->eventSocketNotifier = notifier;
}

void send_frame_rendered_event(QmlRenderer *renderer, void *buf) {
    writeEvent(renderer->appState->rendererWriteFd, EventType::Renderer,
               reinterpret_cast<EventParam>(buf), 0);
}

void setup_renderer(QmlRenderer *renderer) {
    int argc = 0;
    char **argv = nullptr;

    // Install custom message handler
    qInstallMessageHandler(qtMessageHandler);

    renderer->app = new QGuiApplication(argc, argv);

    renderer->context = new QOpenGLContext();
    renderer->surfaceFormat = new QSurfaceFormat();

    renderer->surfaceFormat->setDepthBufferSize(24);
    renderer->surfaceFormat->setStencilBufferSize(8);
    renderer->surfaceFormat->setVersion(3, 2);
    renderer->surfaceFormat->setProfile(QSurfaceFormat::CoreProfile);
    renderer->context->setFormat(*renderer->surfaceFormat);

    if (!renderer->context->create()) {
        error_log(FILENAME, "Failed to create offscreen surface");
        set_deinitialize(renderer);
        return;
    }

    renderer->surface = new QOffscreenSurface();
    renderer->surface->setFormat(renderer->context->format());
    renderer->surface->create();

    if (!renderer->surface->isValid()) {
        error_log(FILENAME, "Failed to create offscreen surface");
        set_deinitialize(renderer);
        return;
    }

    if (!renderer->context->makeCurrent(renderer->surface)) {
        error_log(FILENAME, "Failed to make OpenGL context current");
        set_deinitialize(renderer);
        return;
    }

    renderer->renderControl = new QQuickRenderControl();
    renderer->window = new QQuickWindow(renderer->renderControl);
    renderer->window->resize(renderer->fbSize);

    if (!renderer->renderControl->initialize()) {
        error_log(FILENAME, "Failed to initialize render control");
        set_deinitialize(renderer);
        return;
    }

    renderer->fbFormat = new QOpenGLFramebufferObjectFormat();
    renderer->fbFormat->setAttachment(
        QOpenGLFramebufferObject::CombinedDepthStencil);
    renderer->fb =
        new QOpenGLFramebufferObject(renderer->fbSize, *renderer->fbFormat);

    auto renderTarget = QQuickRenderTarget::fromOpenGLTexture(
        renderer->fb->texture(), renderer->fb->size());
    renderer->window->setRenderTarget(renderTarget);

    renderer->engine = new QQmlEngine();

    renderer->interface = new Interface(renderer);

    // Expose the `tlockr` interface to the QML
    renderer->engine->rootContext()->setContextProperty("tlockr",
                                                        renderer->interface);

    renderer->component = new QQmlComponent(renderer->engine);
}

void setup_renderer_signals(QmlRenderer *renderer) {
    QObject::connect(
        renderer->component, &QQmlComponent::statusChanged, [renderer]() {
            if (renderer->component->status() == QQmlComponent::Ready) {
                QObject *rootObject = renderer->component->create();
                if (!rootObject) {
                    error_log(FILENAME, "Failed to create QML root object");
                    return;
                }

                QQuickItem *rootItem = qobject_cast<QQuickItem *>(rootObject);
                if (!rootItem) {
                    error_log(FILENAME, "Root object is not a QQuickItem");
                    delete rootObject;
                    return;
                }

                rootItem->setParentItem(renderer->window->contentItem());
                rootItem->setWidth(renderer->fbSize.width());
                rootItem->setHeight(renderer->fbSize.height());

                renderer->rootItem = rootItem;
                renderer->running = true;
            } else if (renderer->component->status() == QQmlComponent::Error) {
                error_log(FILENAME, "QML component has errors:");
                const auto errors = renderer->component->errors();
                for (const auto &error : errors) {
                    error_log(renderer->appState->qmlPath,
                              format_log("\t", error.toString().toStdString())
                                  .c_str());
                }
            }
        });

    QObject::connect(
        renderer->renderControl, &QQuickRenderControl::renderRequested,
        [renderer]() {
            if (!renderer->running || !renderer->fb->isValid() ||
                renderer->shouldStop)
                return;

            if (!renderer->context->makeCurrent(renderer->surface)) {
                error_log(FILENAME, "Failed to make OpenGL context current");
                return;
            }

            renderer->renderControl->polishItems();
            renderer->renderControl->beginFrame();
            renderer->renderControl->sync();
            renderer->renderControl->render();
            renderer->renderControl->endFrame();

            if (renderer->getBufferCallback) {
                void *buffer = renderer->getBufferCallback(renderer->userData);
                if (buffer) {
                    render(*renderer->fb, buffer);
                    send_frame_rendered_event(renderer, buffer);
                }
            }
        });

    QObject::connect(renderer->renderControl,
                     &QQuickRenderControl::sceneChanged, [renderer]() {
                         QMetaObject::invokeMethod(
                             renderer->window,
                             [renderer]() { renderer->window->update(); },
                             Qt::QueuedConnection);
                     });

    QObject::connect(renderer->app, &QGuiApplication::aboutToQuit,
                     [renderer]() {
                         renderer->running = false;
                         renderer->renderControl->disconnect();
                     });
}

void qml_renderer_thread(QmlRenderer *renderer) {
    QGuiApplication::setAttribute(Qt::AA_UseOpenGLES, false);

    setup_renderer(renderer);
    setup_event_socket(renderer);
    setup_renderer_signals(renderer);
    set_initialize(renderer);

    renderer->threadRunning = true;
    while (!renderer->shouldStop && renderer->app) {
        renderer->app->processEvents(QEventLoop::AllEvents, 16);
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }

    renderer->threadRunning = false;
}

int start_renderer(QmlRenderer *renderer) {
    if (!renderer) {
        error_log(FILENAME, "Invalid renderer");
        return -1;
    }

    renderer->renderThread = std::thread(qml_renderer_thread, renderer);

    std::unique_lock<std::mutex> lock(renderer->initMutex);
    renderer->initCondition.wait(
        lock, [renderer] { return renderer->initialized.load(); });

    if (!renderer->initialized) {
        error_log(FILENAME, "Failed to initialize Qt");
        return -1;
    }

    QMetaObject::invokeMethod(
        renderer->component,
        [renderer]() {
            info_log(FILENAME, "Loading QML component...");
            renderer->component->loadUrl(
                QUrl::fromLocalFile(renderer->qmlPath));
        },
        Qt::QueuedConnection);

    return 0;
}

void set_callbacks(QmlRenderer *renderer, RsGetBufferCallback getBuffer,
                   void *userData) {
    if (!renderer) {
        return;
    }

    renderer->getBufferCallback = getBuffer;
    renderer->userData = userData;
}

void cleanup_renderer(QmlRenderer *renderer) {
    if (!renderer) {
        return;
    }

    renderer->shouldStop = true;
    renderer->running = false;

    if (renderer->renderThread.joinable()) {
        renderer->renderThread.join();
    }

    delete renderer;

    info_log("tlockr_qt/render.cpp", "Renderer thread exited");
}

int render(const QOpenGLFramebufferObject &fbo, void *buffer) {
    int width = fbo.width();
    int height = fbo.height();

    glBindFramebuffer(GL_FRAMEBUFFER, fbo.handle());

    GLenum status = glCheckFramebufferStatus(GL_FRAMEBUFFER);
    if (status != GL_FRAMEBUFFER_COMPLETE) {
        error_log(FILENAME, format_log("Original framebuffer incomplete: 0x",
                                       std::hex, status)
                                .c_str());
        return 1;
    }

    unsigned char *outputBuffer = static_cast<unsigned char *>(buffer);
    int rowSize = width * 4;

    // Read the framebuffer, starting at the bottom
    for (int y = 0; y < height; y++) {
        glReadPixels(0, height - 1 - y, width, 1, GL_BGRA, GL_UNSIGNED_BYTE,
                     outputBuffer + y * rowSize);
    }

    GLenum error = glGetError();
    if (error != GL_NO_ERROR) {
        error_log(FILENAME, format_log("glReadPixels failed with error: 0x",
                                       std::hex, error)
                                .c_str());
        return 1;
    }

    glBindFramebuffer(GL_FRAMEBUFFER, 0);

    return 0;
}

#ifdef __cplusplus
}
#endif

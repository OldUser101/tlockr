// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include <QGuiApplication>
#include <QOffscreenSurface>
#include <QOpenGLContext>
#include <QOpenGLFramebufferObject>
#include <QQmlComponent>
#include <QQmlEngine>
#include <QQuickItem>
#include <QQuickRenderControl>
#include <QQuickRenderTarget>
#include <QQuickWindow>
#include <QSurfaceFormat>
#include <QDebug>
#include <QVariant>
#include <QTimer>

#include <GLES2/gl2.h>
#include <GLES2/gl2ext.h>

#include <iostream>
#include <unistd.h>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <atomic>
#include <chrono>

#ifdef __cplusplus
extern "C"
{
#endif

	int render(const QOpenGLFramebufferObject &fbo, void *buffer);

	typedef void *(*RsGetBufferCallback)(void *user_data);
	typedef void (*RsFrameReadyCallback)(void *user_data, void *buffer);

	struct QmlRenderer
	{
		QGuiApplication *app;
		QSize fbSize;
		QOpenGLContext *context;
		QSurfaceFormat *surfaceFormat;
		QOffscreenSurface *surface;
		QQuickRenderControl *renderControl;
		QQuickWindow *window;
		QOpenGLFramebufferObjectFormat *fbFormat;
		QOpenGLFramebufferObject *fb;
		QQmlEngine *engine;
		QQmlComponent *component;
		QTimer *renderTimer;

		const char *qmlPath;
		bool running = false;

		RsGetBufferCallback getBufferCallback = nullptr;
		RsFrameReadyCallback frameReadyCallback = nullptr;
		void *userData = nullptr;

		std::thread renderThread;
		std::atomic<bool> threadRunning{false};
		std::atomic<bool> shouldStop{false};
		std::mutex initMutex;
		std::condition_variable initCondition;
		std::atomic<bool> initialized{false};
	};

	QmlRenderer *initialize_renderer(int width, int height, const char *qmlPath)
	{
		QmlRenderer *renderer = new QmlRenderer();
		renderer->fbSize = QSize(width, height);
		renderer->qmlPath = qmlPath;

		return renderer;
	}

	void set_deinitialize(QmlRenderer *renderer)
	{
		{
			std::lock_guard<std::mutex> lock(renderer->initMutex);
			renderer->initialized = false;
		}
		renderer->initCondition.notify_one();
	}

	void set_initialize(QmlRenderer *renderer)
	{
		{
			std::lock_guard<std::mutex> lock(renderer->initMutex);
			renderer->initialized = true;
		}
		renderer->initCondition.notify_one();
	}

	void setup_renderer(QmlRenderer *renderer)
	{
		int argc = 0;
		char **argv = nullptr;
		renderer->app = new QGuiApplication(argc, argv);

		renderer->context = new QOpenGLContext();
		renderer->surfaceFormat = new QSurfaceFormat();

		renderer->surfaceFormat->setDepthBufferSize(24);
		renderer->surfaceFormat->setStencilBufferSize(8);
		renderer->surfaceFormat->setVersion(3, 2);
		renderer->surfaceFormat->setProfile(QSurfaceFormat::CoreProfile);
		renderer->context->setFormat(*renderer->surfaceFormat);

		if (!renderer->context->create())
		{
			std::cerr << "Failed to create OpenGL context\n";
			set_deinitialize(renderer);
			return;
		}

		renderer->surface = new QOffscreenSurface();
		renderer->surface->setFormat(renderer->context->format());
		renderer->surface->create();

		if (!renderer->surface->isValid())
		{
			std::cerr << "Failed to create offscreen surface\n";
			set_deinitialize(renderer);
			return;
		}

		if (!renderer->context->makeCurrent(renderer->surface))
		{
			std::cerr << "Failed to make OpenGL context current\n";
			set_deinitialize(renderer);
			return;
		}

		renderer->renderControl = new QQuickRenderControl();
		renderer->window = new QQuickWindow(renderer->renderControl);
		renderer->window->resize(renderer->fbSize);

		if (!renderer->renderControl->initialize())
		{
			std::cerr << "Failed to initialize render control\n";
			set_deinitialize(renderer);
			return;
		}

		renderer->fbFormat = new QOpenGLFramebufferObjectFormat();
		renderer->fbFormat->setAttachment(QOpenGLFramebufferObject::CombinedDepthStencil);
		renderer->fb = new QOpenGLFramebufferObject(renderer->fbSize, *renderer->fbFormat);

		auto renderTarget = QQuickRenderTarget::fromOpenGLTexture(renderer->fb->texture(), renderer->fb->size());
		renderer->window->setRenderTarget(renderTarget);

		renderer->engine = new QQmlEngine();
		renderer->component = new QQmlComponent(renderer->engine);
		renderer->renderTimer = new QTimer();
	}

	void setup_renderer_signals(QmlRenderer *renderer)
	{
		QObject::connect(renderer->component, &QQmlComponent::statusChanged, [renderer]()
						 {
			if (renderer->component->status() == QQmlComponent::Ready) {
				QObject *rootObject = renderer->component->create();
				if (!rootObject) {
					std::cerr << "Failed to create QML root object\n";
					return;
				}

				QQuickItem *rootItem = qobject_cast<QQuickItem *>(rootObject);
				if (!rootItem) {
					std::cerr << "Root object is not a QQuickItem\n";
					delete rootObject;
					return;
				}

				rootItem->setParentItem(renderer->window->contentItem());
				rootItem->setWidth(renderer->fbSize.width());
				rootItem->setHeight(renderer->fbSize.height());

				renderer->running = true;
				renderer->renderTimer->start(32);	// 32 ms, 30 Hz
			} else if (renderer->component->status() == QQmlComponent::Error) {
				std::cerr << "QML Component has errors:" << std::endl;
				const auto errors = renderer->component->errors();
				for (const auto &error : errors)
					std::cerr << "  " << error.toString().toStdString() << std::endl;
			} });

		QObject::connect(renderer->renderTimer, &QTimer::timeout, [renderer]()
						 {
			if (!renderer->running || !renderer->fb->isValid() || renderer->shouldStop)
				return;
				
			if (!renderer->context->makeCurrent(renderer->surface)) {
				std::cerr << "Failed to make OpenGL context current\n";
				return;
			}       

			renderer->renderControl->polishItems();
			renderer->renderControl->beginFrame();
			renderer->renderControl->sync();
			renderer->renderControl->render();
			renderer->renderControl->endFrame();
			
			if (renderer->getBufferCallback && renderer->frameReadyCallback) {
				void* buffer = renderer->getBufferCallback(renderer->userData);
				if (buffer) {
					render(*renderer->fb, buffer);
					renderer->frameReadyCallback(renderer->userData, buffer);
				}
			} });

		QObject::connect(renderer->app, &QGuiApplication::aboutToQuit, [renderer]()
						 {
			renderer->running = false;
			renderer->renderTimer->stop();
			renderer->renderControl->disconnect(); });
	}

	void qml_renderer_thread(QmlRenderer *renderer)
	{
		QGuiApplication::setAttribute(Qt::AA_UseOpenGLES, false);

		setup_renderer(renderer);
		setup_renderer_signals(renderer);
		set_initialize(renderer);

		renderer->threadRunning = true;
		while (!renderer->shouldStop && renderer->app)
		{
			renderer->app->processEvents(QEventLoop::AllEvents, 16);
			std::this_thread::sleep_for(std::chrono::milliseconds(1));
		}

		renderer->threadRunning = false;
	}

	int start_renderer(QmlRenderer *renderer)
	{
		if (!renderer)
		{
			std::cerr << "Invalid renderer\n";
			return -1;
		}

		renderer->renderThread = std::thread(qml_renderer_thread, renderer);

		std::unique_lock<std::mutex> lock(renderer->initMutex);
		renderer->initCondition.wait(lock, [renderer]
									 { return renderer->initialized.load(); });

		if (!renderer->initialized)
		{
			std::cerr << "Failed to initialize Qt\n";
			return -1;
		}

		QMetaObject::invokeMethod(renderer->component, [renderer]()
								  {
			std::cout << "Loading QML component..." << std::endl;
			renderer->component->loadUrl(QUrl::fromLocalFile(renderer->qmlPath)); }, Qt::QueuedConnection);

		return 0;
	}

	void set_callbacks(QmlRenderer *renderer, RsGetBufferCallback getBuffer, RsFrameReadyCallback frameReady, void *userData)
	{
		if (!renderer)
			return;

		renderer->getBufferCallback = getBuffer;
		renderer->frameReadyCallback = frameReady;
		renderer->userData = userData;
	}

	void cleanup_renderer(QmlRenderer *renderer)
	{
		if (!renderer)
			return;

		renderer->shouldStop = true;
		renderer->running = false;

		if (renderer->renderThread.joinable())
		{
			renderer->renderThread.join();
		}

		delete renderer;
	}

	int render(const QOpenGLFramebufferObject &fbo, void *buffer)
	{
		int width = fbo.width();
		int height = fbo.height();

		glBindFramebuffer(GL_FRAMEBUFFER, fbo.handle());

		GLenum status = glCheckFramebufferStatus(GL_FRAMEBUFFER);
		if (status != GL_FRAMEBUFFER_COMPLETE)
		{
			std::cerr << "Original framebuffer incomplete: 0x" << std::hex << status << "\n";
			return 1;
		}

		unsigned char *outputBuffer = static_cast<unsigned char *>(buffer);
		int rowSize = width * 4;

		// Read the framebuffer, starting at the bottom
		for (int y = 0; y < height; y++)
		{
			glReadPixels(0, height - 1 - y, width, 1, GL_BGRA, GL_UNSIGNED_BYTE, outputBuffer + y * rowSize);
		}

		GLenum error = glGetError();
		if (error != GL_NO_ERROR)
		{
			std::cerr << "glReadPixels failed with error: 0x" << std::hex << error << std::endl;
			return 1;
		}

		glBindFramebuffer(GL_FRAMEBUFFER, 0);

		return 0;
	}

#ifdef __cplusplus
}
#endif

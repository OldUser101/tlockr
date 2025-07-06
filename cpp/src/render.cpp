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

#ifdef __cplusplus
extern "C"
{
#endif

	int render(const QOpenGLFramebufferObject &fbo, void *buffer)
	{
		static bool imageSaved = false;
		if (imageSaved)
		{
			return 0;
		}

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

		imageSaved = true;
		return 0;
	}

	int render_single_frame(const char *qml_path, int width, int height, void *buffer)
	{
		QGuiApplication::setAttribute(Qt::AA_UseOpenGLES, false);

		int argc = 0;
		char **argv = nullptr;
		QGuiApplication app(argc, argv);

		QSize fb_size{width, height};

		QOpenGLContext glContext;
		QSurfaceFormat format;
		format.setDepthBufferSize(24);
		format.setStencilBufferSize(8);
		format.setVersion(3, 2);
		format.setProfile(QSurfaceFormat::CoreProfile);
		glContext.setFormat(format);
		if (!glContext.create())
		{
			std::cerr << "Failed to create OpenGL context\n";
			return -1;
		}

		QOffscreenSurface offscreenSurface;
		offscreenSurface.setFormat(glContext.format());
		offscreenSurface.create();
		if (!offscreenSurface.isValid())
		{
			std::cerr << "Failed to create offscreen surface\n";
			return -1;
		}

		if (!glContext.makeCurrent(&offscreenSurface))
		{
			std::cerr << "Failed to make OpenGL context current\n";
			return -1;
		}

		QQuickRenderControl renderControl;

		QQuickWindow window(&renderControl);
		window.resize(fb_size);

		if (!renderControl.initialize())
		{
			std::cerr << "Failed to initialize QQuickRenderControl\n";
			return -1;
		}

		QOpenGLFramebufferObjectFormat fboFormat;
		fboFormat.setAttachment(QOpenGLFramebufferObject::CombinedDepthStencil);
		QOpenGLFramebufferObject fb(fb_size, fboFormat);

		auto renderTarget = QQuickRenderTarget::fromOpenGLTexture(fb.texture(), fb.size());
		window.setRenderTarget(renderTarget);

		QQmlEngine engine;
		QQmlComponent component(&engine);

		QTimer renderTimer;
		bool running = true;
		bool hasRendered = false;

		QObject::connect(&renderTimer, &QTimer::timeout, [&]()
						 {
			if (!running || !fb.isValid())
				return;
				
			if (!glContext.makeCurrent(&offscreenSurface)) {
				std::cerr << "Failed to make OpenGL context current\n";
				return;
			}       

			renderControl.polishItems();
			renderControl.beginFrame();
			renderControl.sync();
			renderControl.render();
			renderControl.endFrame();
			
			render(fb, buffer);
			
			if (!hasRendered) {
				hasRendered = true;
				renderTimer.stop();
				QTimer::singleShot(100, &app, &QGuiApplication::quit);
        } });

		QObject::connect(&component, &QQmlComponent::statusChanged, [&]()
						 {
			std::cout << "QML Component status changed: " << component.status() << std::endl;
			if (component.status() == QQmlComponent::Ready) {
				std::cout << "QML Component is ready, creating object..." << std::endl;
				QObject *rootObject = component.create();
				if (!rootObject) {
					std::cerr << "Failed to create root QML object\n";
					return;
				}

				std::cout << "Root object created successfully" << std::endl;
				QQuickItem *rootItem = qobject_cast<QQuickItem *>(rootObject);
				if (!rootItem) {
					std::cerr << "Root object is not a QQuickItem\n";
					delete rootObject;
					return;
				}

				std::cout << "Setting up root item..." << std::endl;
				rootItem->setParentItem(window.contentItem());
				rootItem->setWidth(fb_size.width());
				rootItem->setHeight(fb_size.height());
				
				std::cout << "Starting render timer..." << std::endl;
				renderTimer.start(16);
			} else if (component.status() == QQmlComponent::Error) {
				std::cerr << "QML Component has errors:" << std::endl;
				const auto errors = component.errors();
				for (const auto &error : errors)
					std::cerr << "  " << error.toString().toStdString() << std::endl;
			} });

		QObject::connect(&app, &QGuiApplication::aboutToQuit, [&]()
						 {
							 running = false;
							 renderTimer.stop();
							 renderControl.disconnect(); });

		std::cout << "Loading QML component..." << std::endl;
		component.loadUrl(QUrl::fromLocalFile(qml_path));

		return app.exec();
	}

#ifdef __cplusplus
}
#endif

// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef RENDER_HPP
#define RENDER_HPP

#include <QDebug>
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
#include <QSocketNotifier>
#include <QSurfaceFormat>
#include <QTimer>
#include <QVariant>

#include <GLES2/gl2.h>
#include <GLES2/gl2ext.h>

#include <atomic>
#include <chrono>
#include <condition_variable>
#include <errno.h>
#include <fcntl.h>
#include <iostream>
#include <mutex>
#include <thread>
#include <unistd.h>

#include "event.hpp"
#include "event_handler.hpp"

#ifdef __cplusplus
extern "C" {
#endif

typedef void *(*RsGetBufferCallback)(void *user_data);

struct ApplicationState {
    const char *qmlPath;
    int state;
    void *renderer;
    int rendererWriteFd;
    int rendererReadFd;
};

struct QmlRenderer {
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
    QSocketNotifier *eventSocketNotifier;

    const char *qmlPath;
    bool running = false;

    RsGetBufferCallback getBufferCallback = nullptr;
    void *userData = nullptr;

    std::thread renderThread;
    std::atomic<bool> threadRunning{false};
    std::atomic<bool> shouldStop{false};
    std::mutex initMutex;
    std::condition_variable initCondition;
    std::atomic<bool> initialized{false};

    EventHandler *eventHandler;
    ApplicationState *appState;
};

typedef void *(*RsGetBufferCallback)(void *user_data);

QmlRenderer *initialize_renderer(int width, int height, const char *qmlPath,
                                 ApplicationState *appState);
int start_renderer(QmlRenderer *renderer);
void set_callbacks(QmlRenderer *renderer, RsGetBufferCallback getBuffer,
                   void *userData);
int render(const QOpenGLFramebufferObject &fbo, void *buffer);
void cleanup_renderer(QmlRenderer *renderer);

#ifdef __cplusplus
}
#endif

#endif

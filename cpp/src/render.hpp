// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef RENDER_HPP
#define RENDER_HPP

#ifdef __cplusplus
extern "C"
{
#endif

    struct QmlRenderer;

    typedef void *(*RsGetBufferCallback)(void *user_data);

    QmlRenderer *initialize_renderer(int width, int height, const char *qmlPath);
    int start_renderer(QmlRenderer *renderer);
    void set_callbacks(QmlRenderer *renderer, RsGetBufferCallback getBuffer, RsFrameReadyCallback frameReady, void *userData);
    void cleanup_renderer(QmlRenderer *renderer);

#ifdef __cplusplus
}
#endif

#endif

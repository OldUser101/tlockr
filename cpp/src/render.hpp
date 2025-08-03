// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef RENDER_HPP
#define RENDER_HPP

#ifdef __cplusplus
extern "C" {
#endif

struct QmlRenderer;
struct ApplicationState;

typedef void *(*RsGetBufferCallback)(void *user_data);

QmlRenderer *initialize_renderer(int width, int height, const char *qmlPath,
                                 ApplicationState *appState);
int start_renderer(QmlRenderer *renderer);
void set_callbacks(QmlRenderer *renderer, RsGetBufferCallback getBuffer,
                   void *userData);
void cleanup_renderer(QmlRenderer *renderer);

#ifdef __cplusplus
}
#endif

#endif

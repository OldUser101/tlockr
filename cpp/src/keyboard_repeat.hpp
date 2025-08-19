// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef KEYBOARD_REPEAT_HPP
#define KEYBOARD_REPEAT_HPP

#include <QTimer>
#include <cstdint>
#include <functional>

struct QmlRenderer;
struct KeyPressEvent;

struct RepeatInfo {
    int32_t rate;
    int32_t delay;
};

class KeyboardRepeatEngine {
private:
    QmlRenderer *m_renderer;
    RepeatInfo *m_repeatInfo = nullptr;
    QTimer *m_timer = nullptr;
    QTimer *m_delayTimer = nullptr;

    bool m_running = false;

    std::function<void(KeyPressEvent *)> m_callback;

    KeyPressEvent *m_lastEvent = nullptr;

    void tryStart();
    void timeout();

public:
    explicit KeyboardRepeatEngine(QmlRenderer *renderer);
    ~KeyboardRepeatEngine();

    void setRepeatInfo(int32_t rate, int32_t delay);
    void setCallback(std::function<void(KeyPressEvent *)> callback);

    void set(KeyPressEvent *event);
    bool state();
    void reset();
};

#endif

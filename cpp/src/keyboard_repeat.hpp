// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef KEYBOARD_REPEAT_HPP
#define KEYBOARD_REPEAT_HPP

#include <cstdint>

struct QmlRenderer;

struct RepeatInfo {
    int32_t rate;
    int32_t delay;
};

class KeyboardRepeatEngine {
private:
    QmlRenderer *m_renderer;
    RepeatInfo *m_repeatInfo = nullptr;

public:
    explicit KeyboardRepeatEngine(QmlRenderer *renderer);
    ~KeyboardRepeatEngine();

    void setRepeatInfo(int32_t rate, int32_t delay);
};

#endif

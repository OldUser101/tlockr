// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "keyboard_repeat.hpp"
#include "logging.hpp"
#include "render.hpp"

static const char *FILENAME = "tlockr_qt/keyboard_repeat.cpp";

KeyboardRepeatEngine::KeyboardRepeatEngine(QmlRenderer *renderer)
    : m_renderer(renderer) {}

KeyboardRepeatEngine::~KeyboardRepeatEngine() {
    if (m_repeatInfo) {
        delete m_repeatInfo;
        m_repeatInfo = nullptr;
    }
}

void KeyboardRepeatEngine::setRepeatInfo(int32_t rate, int32_t delay) {
    if (m_repeatInfo == nullptr) {
        m_repeatInfo = new RepeatInfo;
    }

    m_repeatInfo->rate = rate;
    m_repeatInfo->delay = delay;

    debug_log(FILENAME, "Updated repeat info");
}

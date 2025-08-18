// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "keyboard_repeat.hpp"
#include "logging.hpp"
#include "render.hpp"
#include <QObject>

static const char *FILENAME = "tlockr_qt/keyboard_repeat.cpp";

KeyboardRepeatEngine::KeyboardRepeatEngine(QmlRenderer *renderer)
    : m_renderer(renderer) {
    m_timer = new QTimer();

    QObject::connect(m_timer, &QTimer::timeout, [this] { timeout(); });
}

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

    m_timer->setInterval(rate);

    debug_log(FILENAME, "Updated repeat info");
}

void KeyboardRepeatEngine::tryStart() {
    if (m_running) {
        timeout();
        m_timer->start();
    }
}

void KeyboardRepeatEngine::timeout() {
    if (m_running) {
        debug_log(FILENAME, "Keyboard repeat");
    } else {
        m_timer->stop();
    }
}

void KeyboardRepeatEngine::set() {
    if (m_repeatInfo == nullptr) {
        return;
    }

    m_running = true;
    m_timer->singleShot(m_repeatInfo->delay, [this] { tryStart(); });
}

bool KeyboardRepeatEngine::state() { return m_running; }

void KeyboardRepeatEngine::reset() {
    m_running = false;
    m_timer->stop();
}

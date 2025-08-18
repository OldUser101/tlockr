// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef KEYBOARD_HPP
#define KEYBOARD_HPP

#include <QKeyEvent>
#include <QString>
#include <cstdint>
#include <xkbcommon/xkbcommon.h>

enum class KeyState : uint32_t {
    Released = 0,
    Pressed = 1,
    Repeated = 2,
};

struct QmlRenderer;

struct RepeatInfo {
    int32_t rate;
    int32_t delay;
};

class KeyboardHandler {
private:
    QmlRenderer *m_renderer;
    struct xkb_context *m_xkbContext;
    struct xkb_keymap *m_xkbKeymap;
    struct xkb_state *m_xkbState;
    struct RepeatInfo m_repeatInfo;

public:
    explicit KeyboardHandler(QmlRenderer *renderer);
    ~KeyboardHandler();

    void setupXkbContext();
    void handleKeymapEvent(int fd, uint32_t size);
    void handleModifiersEvent(uint32_t mods_depressed, uint32_t mods_latched,
                              uint32_t mods_locked, uint32_t group);
    void handleKeyEvent(uint32_t key_code, KeyState state);
    void handleRepeatInfoEvent(int32_t rate, int32_t delay);

    Qt::Key xkbKeysymToQtKey(xkb_keysym_t keysym);
    Qt::KeyboardModifiers xkbStateToQtModifiers();

    void sendKeyEvent(QEvent::Type eventType, Qt::Key key,
                      Qt::KeyboardModifiers modifiers, const QString &text);
};

#endif

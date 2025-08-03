// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef KEYBOARD_HPP
#define KEYBOARD_HPP

#include <QKeyEvent>
#include <QString>
#include <cstdint>
#include <xkbcommon/xkbcommon.h>

struct QmlRenderer;

class KeyboardHandler {
private:
    QmlRenderer *m_renderer;
    struct xkb_context *m_xkbContext;
    struct xkb_keymap *m_xkbKeymap;
    struct xkb_state *m_xkbState;

public:
    explicit KeyboardHandler(QmlRenderer *renderer);
    ~KeyboardHandler();

    void setupXkbContext();
    void handleKeymapEvent(int fd, uint32_t size);
};

#endif
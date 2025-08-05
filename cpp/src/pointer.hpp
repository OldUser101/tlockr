// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef POINTER_HPP
#define POINTER_HPP

#include <QMouseEvent>
#include <cstdint>

enum class ButtonState : uint32_t {
    Released = 0,
    Pressed = 1,
};

struct QmlRenderer;
class KeyboardHandler;

class PointerHandler {
private:
    QmlRenderer *m_renderer;
    KeyboardHandler *m_keyboardHandler;
    Qt::MouseButtons m_buttonState;
    QPointF m_globalPos;

public:
    explicit PointerHandler(QmlRenderer *renderer,
                            KeyboardHandler *keyboardHandler);
    ~PointerHandler();

    void handleMotionEvent(double surface_x, double surface_y);
    void handleButtonEvent(uint32_t button, ButtonState state);

    void sendMouseEvent(QEvent::Type eventType, QPointF globalPos,
                        Qt::MouseButton button, Qt::MouseButtons buttons);

    Qt::MouseButton waylandButtonToQtButton(uint32_t button);
};

/*
    The following codes are defined in `linux/input-event-codes.h` for mouse
    button mapping
*/

#define BTN_LEFT 0x110
#define BTN_RIGHT 0x111
#define BTN_MIDDLE 0x112
#define BTN_FORWARD 0x115
#define BTN_BACK 0x116
#define BTN_TASK 0x117

#endif
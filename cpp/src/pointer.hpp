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

public:
    explicit PointerHandler(QmlRenderer *renderer,
                            KeyboardHandler *keyboardHandler);
    ~PointerHandler();

    void handleMotionEvent(double surface_x, double surface_y);
    void handleButtonEvent(uint32_t button, ButtonState state);

    void sendMouseEvent(QEvent::Type eventType, QPointF globalPos,
                        Qt::MouseButton button, Qt::MouseButtons buttons);
};

#endif
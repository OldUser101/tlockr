// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "pointer.hpp"
#include "keyboard.hpp"
#include "logging.hpp"
#include "render.hpp"
#include <QCoreApplication>
#include <QGuiApplication>
#include <QQuickItem>
#include <QWindow>
#include <iostream>

static const char *FILENAME = "tlockr_qt/pointer.cpp";

PointerHandler::PointerHandler(QmlRenderer *renderer,
                               KeyboardHandler *keyboardHandler)
    : m_renderer(renderer), m_keyboardHandler(keyboardHandler) {}

PointerHandler::~PointerHandler() = default;

void PointerHandler::handleMotionEvent(double surface_x, double surface_y) {
    QPointF globalPos(surface_x, surface_y);

    m_globalPos = globalPos;

    sendMouseEvent(QEvent::MouseMove, globalPos, Qt::NoButton, m_buttonState);
}

void PointerHandler::handleButtonEvent(uint32_t button, ButtonState state) {
    Qt::MouseButton mouse_button = waylandButtonToQtButton(button);

    if (state == ButtonState::Pressed) {
        m_buttonState |= mouse_button;
        sendMouseEvent(QEvent::MouseButtonPress, m_globalPos, mouse_button,
                       m_buttonState);
    } else {
        m_buttonState &= ~mouse_button;
        sendMouseEvent(QEvent::MouseButtonRelease, m_globalPos, mouse_button,
                       m_buttonState);
    }
}

void PointerHandler::sendMouseEvent(QEvent::Type eventType, QPointF globalPos,
                                    Qt::MouseButton button,
                                    Qt::MouseButtons buttons) {
    if (!m_renderer->window) {
        error_log(FILENAME, "No renderer window available");
        return;
    }

    QPointF windowPos = globalPos;

    Qt::KeyboardModifiers keyboardModifiers =
        m_keyboardHandler->xkbStateToQtModifiers();

    QMouseEvent *event = new QMouseEvent(eventType, globalPos, globalPos,
                                         button, buttons, keyboardModifiers);

    QCoreApplication::postEvent(m_renderer->window, event);
}
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "pointer.hpp"
#include "keyboard.hpp"
#include "render.hpp"
#include <QCoreApplication>
#include <QGuiApplication>
#include <QQuickItem>
#include <QWindow>
#include <iostream>

PointerHandler::PointerHandler(QmlRenderer *renderer,
                               KeyboardHandler *keyboardHandler)
    : m_renderer(renderer), m_keyboardHandler(keyboardHandler) {}

PointerHandler::~PointerHandler() = default;

void PointerHandler::handleMotionEvent(double surface_x, double surface_y) {
    QPointF globalPos(surface_x, surface_y);

    sendMouseEvent(QEvent::MouseMove, globalPos, Qt::NoButton, Qt::NoButton);
}

void PointerHandler::sendMouseEvent(QEvent::Type eventType, QPointF globalPos,
                                    Qt::MouseButton button,
                                    Qt::MouseButtons buttons) {
    if (!m_renderer->window) {
        std::cerr << "No renderer window available\n";
        return;
    }

    QPointF windowPos = globalPos;

    Qt::KeyboardModifiers keyboardModifiers =
        m_keyboardHandler->xkbStateToQtModifiers();

    QMouseEvent *event = new QMouseEvent(eventType, globalPos, globalPos,
                                         button, buttons, keyboardModifiers);

    QCoreApplication::postEvent(m_renderer->window, event);
}
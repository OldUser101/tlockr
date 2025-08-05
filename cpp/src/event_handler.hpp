// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef EVENT_HANDLER_HPP
#define EVENT_HANDLER_HPP

#include "event.hpp"
#include <memory>

struct QmlRenderer;
class KeyboardHandler;
class PointerHandler;

class EventHandler {
private:
    QmlRenderer *m_renderer;
    KeyboardHandler *m_keyboardHandler;
    PointerHandler *m_pointerHandler;

public:
    explicit EventHandler(QmlRenderer *renderer);
    ~EventHandler();

    int processEvent(EventType event_type, EventParam param_1,
                     EventParam param_2);
    void handleReceivedEvent();
};

#endif
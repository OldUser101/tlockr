// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "event_handler.hpp"
#include "keyboard.hpp"
#include "logging.hpp"
#include "pointer.hpp"
#include "render.hpp"
#include <errno.h>
#include <iostream>
#include <unistd.h>

static const char *FILENAME = "tlockr_qt/event_handler.cpp";

int readEvent(int fd, Event *event) {
    ssize_t res = read(fd, event, sizeof(Event));
    if (res == -1) {
        if (errno == EAGAIN || errno == EWOULDBLOCK) {
            return -1;
        } else {
            error_log(
                FILENAME,
                format_log("Failed to read event: ", strerror(errno)).c_str());
            return -1;
        }
    } else if (res != sizeof(Event)) {
        error_log(FILENAME, format_log("Partial read: expected ", sizeof(Event),
                                       " bytes, got ", res)
                                .c_str());
        return -1;
    }

    return 0;
}

EventHandler::EventHandler(QmlRenderer *renderer) : m_renderer(renderer) {
    m_keyboardHandler = new KeyboardHandler(renderer);
    m_pointerHandler = new PointerHandler(renderer, m_keyboardHandler);
}

EventHandler::~EventHandler() = default;

int EventHandler::processEvent(EventType event_type, EventParam param_1,
                               EventParam param_2) {
    switch (event_type) {
        case EventType::KeyboardKeymap: {
            m_keyboardHandler->handleKeymapEvent(param_1, param_2);
            break;
        }
        case EventType::KeyboardModifiers: {
            // Modifiers bit packed:
            // param_1: 31 [mods_depressed] [mods_latched] 0
            // param_2: 31 [  mods_locked ] [    group   ] 0
            m_keyboardHandler->handleModifiersEvent(
                param_1 >> 32, param_1 & 0xFFFF, param_2 >> 32,
                param_2 & 0xFFFF);
            break;
        }
        case EventType::KeyboardKey: {
            m_keyboardHandler->handleKeyEvent(param_1,
                                              static_cast<KeyState>(param_2));
            break;
        }
        case EventType::PointerMotion: {
            double surface_x = *reinterpret_cast<const double *>(&param_1);
            double surface_y = *reinterpret_cast<const double *>(&param_2);
            m_pointerHandler->handleMotionEvent(surface_x, surface_y);
            break;
        }
        case EventType::PointerButton: {
            m_pointerHandler->handleButtonEvent(
                param_1, static_cast<ButtonState>(param_2));
            break;
        }
    }

    debug_log(FILENAME,
              format_log("Event Type: ", static_cast<uint64_t>(event_type),
                         "; Param 1: ", param_1, "; Param 2: ", param_2)
                  .c_str());
    return 0;
}

void EventHandler::handleReceivedEvent() {
    Event ev = {};
    int result = readEvent(m_renderer->appState->rendererReadFd, &ev);

    if (result == 0) {
        processEvent(ev.event_type, ev.param_1, ev.param_2);
    }
}
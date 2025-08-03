// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "event_handler.hpp"
#include "render.hpp"
#include <errno.h>
#include <iostream>
#include <unistd.h>

int readEvent(int fd, Event *event) {
    ssize_t res = read(fd, event, sizeof(Event));
    if (res == -1) {
        if (errno == EAGAIN || errno == EWOULDBLOCK) {
            return -1;
        } else {
            std::cerr << "Failed to read event: " << strerror(errno) << "\n";
            return -1;
        }
    } else if (res != sizeof(Event)) {
        std::cerr << "Partial read: expected " << sizeof(Event)
                  << " bytes, got " << res << "\n";
        return -1;
    }

    return 0;
}

EventHandler::EventHandler(QmlRenderer *renderer) : m_renderer(renderer) {
    this->m_keyboardHandler = new KeyboardHandler(renderer);
}

EventHandler::~EventHandler() = default;

int EventHandler::processEvent(EventType event_type, EventParam param_1,
                               EventParam param_2) {
    switch (event_type) {
        case EventType::KeyboardKeymap: {
            this->m_keyboardHandler->handleKeymapEvent(param_1, param_2);
        }
    }
    std::cout << "Event Type: " << static_cast<uint64_t>(event_type)
              << "; Param 1: " << param_1 << "; Param 2: " << param_2 << "\n";
    std::cout.flush();
    return 0;
}

void EventHandler::handleReceivedEvent() {
    Event ev = {};
    int result = readEvent(m_renderer->appState->rendererReadFd, &ev);

    if (result == 0) {
        processEvent(ev.event_type, ev.param_1, ev.param_2);
    }
}
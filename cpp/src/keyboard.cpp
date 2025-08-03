// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "keyboard.hpp"
#include <QCoreApplication>
#include <QGuiApplication>
#include <iostream>
#include <sys/mman.h>
#include <unistd.h>

KeyboardHandler::KeyboardHandler(QmlRenderer *renderer)
    : m_renderer(renderer), m_xkbContext(nullptr), m_xkbKeymap(nullptr),
      m_xkbState(nullptr) {}

KeyboardHandler::~KeyboardHandler() {
    if (this->m_xkbContext) {
        xkb_context_unref(this->m_xkbContext);
    }
}

void KeyboardHandler::setupXkbContext() {
    this->m_xkbContext = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
    if (!this->m_xkbContext) {
        std::cerr << "Failed to create XKB context\n";
    }

    std::cout << "Created new XKB context\n";
}

void KeyboardHandler::handleKeymapEvent(int fd, uint32_t size) {
    if (!m_xkbContext) {
        setupXkbContext();
    }

    char *keymap_str =
        static_cast<char *>(mmap(nullptr, size, PROT_READ, MAP_PRIVATE, fd, 0));
    if (keymap_str == MAP_FAILED) {
        std::cerr << "Failed to mmap keymap\n";
        close(fd);
        return;
    }

    struct xkb_keymap *keymap = xkb_keymap_new_from_string(
        m_xkbContext, keymap_str, XKB_KEYMAP_FORMAT_TEXT_V1,
        XKB_KEYMAP_COMPILE_NO_FLAGS);

    munmap(keymap_str, size);
    close(fd);

    if (!keymap) {
        std::cerr << "Failed to create XKB keymap\n";
        return;
    }

    if (m_xkbState) {
        xkb_state_unref(m_xkbState);
    }
    if (m_xkbKeymap) {
        xkb_keymap_unref(m_xkbKeymap);
    }

    m_xkbKeymap = keymap;
    m_xkbState = xkb_state_new(keymap);

    if (!m_xkbState) {
        std::cerr << "Failed to create XKB state\n";
        return;
    }

    std::cout << "Loaded new XKB keymap\n";
}

void KeyboardHandler::handleModifiersEvent(uint32_t mods_depressed,
                                           uint32_t mods_latched,
                                           uint32_t mods_locked,
                                           uint32_t group) {
    if (this->m_xkbState) {
        xkb_state_update_mask(this->m_xkbState, mods_depressed, mods_latched,
                              mods_locked, 0, 0, group);
        std::cout << "Updated modifiers\n";
    }
}
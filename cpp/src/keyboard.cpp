// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "keyboard.hpp"
#include "render.hpp"
#include <QCoreApplication>
#include <QGuiApplication>
#include <iostream>
#include <sys/mman.h>
#include <unistd.h>

KeyboardHandler::KeyboardHandler(QmlRenderer *renderer)
    : m_renderer(renderer), m_xkbContext(nullptr), m_xkbKeymap(nullptr),
      m_xkbState(nullptr) {}

KeyboardHandler::~KeyboardHandler() {
    if (m_xkbContext) {
        xkb_context_unref(m_xkbContext);
    }
}

void KeyboardHandler::setupXkbContext() {
    m_xkbContext = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
    if (!m_xkbContext) {
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
    if (m_xkbState) {
        xkb_state_update_mask(m_xkbState, mods_depressed, mods_latched,
                              mods_locked, 0, 0, group);
        std::cout << "Updated modifiers\n";
    }
}

void KeyboardHandler::handleKeyEvent(uint32_t key_code, KeyState state) {
    if (!m_xkbState) {
        std::cout << "No XKB state available" << std::endl;
        return;
    }

    uint32_t xkb_keycode = key_code + 8;

    if (state == KeyState::Pressed) {
        xkb_state_update_key(m_xkbState, xkb_keycode, XKB_KEY_DOWN);
    } else if (state == KeyState::Released) {
        xkb_state_update_key(m_xkbState, xkb_keycode, XKB_KEY_UP);
    }

    xkb_keysym_t keysym = xkb_state_key_get_one_sym(m_xkbState, xkb_keycode);

    Qt::Key key = xkbKeysymToQtKey(keysym);

    char buffer[64];
    int size =
        xkb_state_key_get_utf8(m_xkbState, xkb_keycode, buffer, sizeof(buffer));
    QString text;
    if (size > 0) {
        text = QString::fromUtf8(buffer, size);
    }

    Qt::KeyboardModifiers modifiers = xkbStateToQtModifiers();

    if (state == KeyState::Pressed) {
        sendKeyEvent(QEvent::KeyPress, key, modifiers, text);
    } else if (state == KeyState::Released) {
        sendKeyEvent(QEvent::KeyRelease, key, modifiers, text);
    }
}

void KeyboardHandler::sendKeyEvent(QEvent::Type eventType, Qt::Key key,
                                   Qt::KeyboardModifiers modifiers,
                                   const QString &text) {
    QKeyEvent *event = new QKeyEvent(eventType, key, modifiers, text);
    QObject *target = nullptr;

    if (QGuiApplication::focusObject()) {
        target = QGuiApplication::focusObject();
    } else if (m_renderer->window && m_renderer->window->activeFocusItem()) {
        target = m_renderer->window->activeFocusItem();
    } else if (m_renderer->rootItem) {
        target = m_renderer->rootItem;
    } else if (m_renderer->window) {
        target = m_renderer->window;
    }

    if (target) {
        std::cout << "Sent key event\n";
        QCoreApplication::postEvent(target, event);
    } else {
        delete event;
    }
}

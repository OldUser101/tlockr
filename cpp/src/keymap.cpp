// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "keyboard.hpp"

Qt::Key KeyboardHandler::xkbKeysymToQtKey(xkb_keysym_t keysym) {
    if (keysym >= 0x20 && (keysym < 0xD800 || keysym > 0xDFFF) &&
        keysym <= 0x10FFFF) {
        if (keysym >= 'a' && keysym <= 'z') {
            // All Qt keys are uppercase
            return static_cast<Qt::Key>(keysym - 'a' + 'A');
        } else {
            return static_cast<Qt::Key>(keysym);
        }
    }

    switch (keysym) {
        default:
            return Qt::Key_unknown;
    }
}

Qt::KeyboardModifiers KeyboardHandler::xkbStateToQtModifiers() {
    Qt::KeyboardModifiers modifiers = Qt::NoModifier;

    if (!m_xkbState) {
        return modifiers;
    }

    if (xkb_state_mod_name_is_active(m_xkbState, XKB_MOD_NAME_SHIFT,
                                     XKB_STATE_MODS_EFFECTIVE)) {
        modifiers |= Qt::ShiftModifier;
    }
    if (xkb_state_mod_name_is_active(m_xkbState, XKB_MOD_NAME_CTRL,
                                     XKB_STATE_MODS_EFFECTIVE)) {
        modifiers |= Qt::ControlModifier;
    }
    if (xkb_state_mod_name_is_active(m_xkbState, XKB_MOD_NAME_ALT,
                                     XKB_STATE_MODS_EFFECTIVE)) {
        modifiers |= Qt::AltModifier;
    }
    if (xkb_state_mod_name_is_active(m_xkbState, XKB_MOD_NAME_LOGO,
                                     XKB_STATE_MODS_EFFECTIVE)) {
        modifiers |= Qt::MetaModifier;
    }

    return modifiers;
}
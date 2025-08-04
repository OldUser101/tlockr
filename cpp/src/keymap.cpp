// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "keyboard.hpp"

Qt::Key KeyboardHandler::xkbKeysymToQtKey(xkb_keysym_t keysym) {
    if (keysym >= 0x20 && keysym < 0x100) {
        if (keysym >= 'a' && keysym <= 'z') {
            // All Qt keys are uppercase
            return static_cast<Qt::Key>(keysym - 'a' + 'A');
        } else {
            return static_cast<Qt::Key>(keysym);
        }
    }

    if (keysym >= 0x1000100 && keysym <= 0x110FFFF) {
        return static_cast<Qt::Key>(keysym - 0x1000000);
    }

    if (keysym >= XKB_KEY_F1 && keysym <= XKB_KEY_F35) {
        // Shift the keysym into the Qt function range
        return static_cast<Qt::Key>(keysym + 0xFF0072);
    }

    switch (keysym) {
        case XKB_KEY_Escape:
            return Qt::Key_Escape;
        case XKB_KEY_Tab:
            return Qt::Key_Tab;
        case XKB_KEY_BackTab:
            return Qt::Key_Backtab;
        case XKB_KEY_BackSpace:
            return Qt::Key_Backspace;
        case XKB_KEY_Return:
            return Qt::Key_Return;
        case XKB_KEY_KP_Enter:
            return Qt::Key_Enter;
        case XKB_KEY_Insert:
            return Qt::Key_Insert;
        case XKB_KEY_Delete:
            return Qt::Key_Delete;
        case XKB_KEY_Pause:
            return Qt::Key_Pause;
        case XKB_KEY_Print:
            return Qt::Key_Print;
        case XKB_KEY_Sys_Req:
            return Qt::Key_SysReq;
        case XKB_KEY_Home:
            return Qt::Key_Home;
        case XKB_KEY_End:
            return Qt::Key_End;
        case XKB_KEY_Left:
            return Qt::Key_Left;
        case XKB_KEY_Up:
            return Qt::Key_Up;
        case XKB_KEY_Right:
            return Qt::Key_Right;
        case XKB_KEY_Down:
            return Qt::Key_Down;
        case XKB_KEY_Page_Up:
            return Qt::Key_PageUp;
        case XKB_KEY_Page_Down:
            return Qt::Key_PageDown;
        case XKB_KEY_Shift_L:
        case XKB_KEY_Shift_R:
            return Qt::Key_Shift;
        case XKB_KEY_Control_L:
        case XKB_KEY_Control_R:
            return Qt::Key_Control;
        case XKB_KEY_Meta_L:
        case XKB_KEY_Meta_R:
            return Qt::Key_Meta;
        case XKB_KEY_Alt_L:
        case XKB_KEY_Alt_R:
            return Qt::Key_Alt;
        case XKB_KEY_Caps_Lock:
            return Qt::Key_CapsLock;
        case XKB_KEY_Num_Lock:
            return Qt::Key_NumLock;
        case XKB_KEY_Scroll_Lock:
            return Qt::Key_ScrollLock;
        case XKB_KEY_Super_L:
            return Qt::Key_Super_L;
        case XKB_KEY_Super_R:
            return Qt::Key_Super_R;
        case XKB_KEY_Menu:
            return Qt::Key_Menu;
        case XKB_KEY_Hyper_L:
            return Qt::Key_Hyper_L;
        case XKB_KEY_Hyper_R:
            return Qt::Key_Hyper_R;
        case XKB_KEY_Help:
            return Qt::Key_Help;
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
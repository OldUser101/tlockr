// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "pointer.hpp"

Qt::MouseButton PointerHandler::waylandButtonToQtButton(uint32_t button) {
    switch (button) {
        case BTN_LEFT:
            return Qt::LeftButton;
        case BTN_RIGHT:
            return Qt::RightButton;
        case BTN_MIDDLE:
            return Qt::MiddleButton;
        case BTN_BACK:
            return Qt::BackButton;
        case BTN_FORWARD:
            return Qt::ForwardButton;
        case BTN_TASK:
            return Qt::ExtraButton4;
        default:
            return Qt::NoButton;
    }
}
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "interface.hpp"
#include "event.hpp"
#include "logging.hpp"
#include "render.hpp"

static const char *FILENAME = "tlockr_qt/interface.cpp";

Interface::Interface(QmlRenderer *renderer, QObject *parent)
    : m_renderer(renderer), QObject(parent) {}

Interface::~Interface() = default;

Q_INVOKABLE void Interface::sendMessage(const QString &msg) {
    writeEvent(m_renderer->appState->authWriteFd, EventType::AuthSubmit, 0, 0);
    info_log(FILENAME, msg.toStdString().c_str());
}

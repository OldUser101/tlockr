// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "interface.hpp"
#include "event.hpp"
#include "ffi.hpp"
#include "logging.hpp"
#include "render.hpp"

static const char *FILENAME = "tlockr_qt/interface.cpp";

Interface::Interface(QmlRenderer *renderer, QObject *parent)
    : m_renderer(renderer), QObject(parent) {}

Interface::~Interface() = default;

Q_INVOKABLE void Interface::sendAuthSubmit(const QString &msg) {
    QByteArray bm = msg.toUtf8();
    ForeignBuffer *fbu =
        build_ffi_buffer(static_cast<void *>(bm.data()), bm.length());
    writeEvent(m_renderer->appState->authWriteFd, EventType::AuthSubmit,
               reinterpret_cast<EventParam>(fbu), 0);
    debug_log(FILENAME, "Sent AuthSubmit event to authenticator");
}

Q_INVOKABLE void Interface::debug(const QString &msg) {
    debug_log(m_renderer->appState->qmlPath, msg.toStdString().c_str());
}

Q_INVOKABLE void Interface::info(const QString &msg) {
    info_log(m_renderer->appState->qmlPath, msg.toStdString().c_str());
}

Q_INVOKABLE void Interface::warn(const QString &msg) {
    warn_log(m_renderer->appState->qmlPath, msg.toStdString().c_str());
}

Q_INVOKABLE void Interface::error(const QString &msg) {
    error_log(m_renderer->appState->qmlPath, msg.toStdString().c_str());
}


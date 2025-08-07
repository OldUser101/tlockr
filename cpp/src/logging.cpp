// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "logging.hpp"

void qtMessageHandler(QtMsgType type, const QMessageLogContext &,
                      const QString &msg) {
    QByteArray localMsg = msg.toLocal8Bit();

    switch (type) {
        case QtDebugMsg:
            debug_log("tlockr_qt", localMsg.constData());
            break;
        case QtInfoMsg:
            info_log("tlockr_qt", localMsg.constData());
            break;
        case QtWarningMsg:
            warn_log("tlockr_qt", localMsg.constData());
            break;
        case QtCriticalMsg:
        case QtFatalMsg:
            error_log("tlockr_qt", localMsg.constData());
            break;
    }
}
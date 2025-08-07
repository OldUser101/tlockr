// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef LOGGING_HPP
#define LOGGING_HPP

#include <QDebug>
#include <QMessageLogContext>
#include <QString>
#include <QtGlobal>
#include <sstream>

template <typename... Args> std::string format_log(Args &&...args) {
    std::ostringstream ss;
    (ss << ... << args);
    return ss.str();
}

void qtMessageHandler(QtMsgType type, const QMessageLogContext &,
                      const QString &msg);

#ifdef __cplusplus
extern "C" {
#endif

void trace_log(const char *file, const char *msg);
void debug_log(const char *file, const char *msg);
void info_log(const char *file, const char *msg);
void warn_log(const char *file, const char *msg);
void error_log(const char *file, const char *msg);

#ifdef __cplusplus
}
#endif

#endif
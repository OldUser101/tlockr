// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef INTERFACE_HPP
#define INTERFACE_HPP

#include <QObject>
#include <QString>

struct QmlRenderer;

class Interface : public QObject {
    Q_OBJECT
private:
    QmlRenderer *m_renderer;

public:
    explicit Interface(QmlRenderer *renderer, QObject *parent = nullptr);
    ~Interface();

    Q_INVOKABLE void sendAuthSubmit(const QString &msg);
    
    Q_INVOKABLE void debug(const QString &msg);
    Q_INVOKABLE void info(const QString &msg);
    Q_INVOKABLE void warn(const QString &msg);
    Q_INVOKABLE void error(const QString &msg);

    enum AuthState {
        Pending = 0,
        Failed = 1,
        Success = 2,
    };
    Q_ENUM(AuthState)

signals:
    void authStateChange(AuthState state);
};

#endif

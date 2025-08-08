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

    Q_INVOKABLE void sendMessage(const QString &msg);
};

#endif

// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef EVENT_HPP
#define EVENT_HPP

#include <cstdint>

#ifdef __cplusplus
extern "C"
{
#endif

    enum class EventType : uint64_t
    {
        Wayland = 1,
        Renderer = 2,
    };

    typedef uint64_t EventParam;

    typedef struct
    {
        EventType event_type;
        EventParam param_1;
        EventParam param_2;
    } Event;

#ifdef __cplusplus
}
#endif

#endif

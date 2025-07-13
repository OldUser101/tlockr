# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2025, Nathan Gill

cmake_minimum_required(VERSION 3.12)

project(tlockr_qt LANGUAGES CXX)

add_subdirectory(cpp)

set_property(GLOBAL PROPERTY USE_FOLDERS ON)
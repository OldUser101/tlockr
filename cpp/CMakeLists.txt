# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2025, Nathan Gill

cmake_minimum_required(VERSION 3.12)

project(tlockr_qt LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

find_package(Qt6 COMPONENTS Core Quick REQUIRED)

add_library(${PROJECT_NAME} STATIC
  src/render.cpp
  src/event_handler.cpp
)

target_link_libraries(${PROJECT_NAME} PRIVATE
  Qt6::Core
  Qt6::Quick
)

install(TARGETS ${PROJECT_NAME}
  ARCHIVE DESTINATION lib
  LIBRARY DESTINATION lib
  RUNTIME DESTINATION bin
)

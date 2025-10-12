# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2025, Nathan Gill

cmake_minimum_required(VERSION 3.12)

project(tlockr_qt LANGUAGES CXX)

add_subdirectory(cpp)

set_property(GLOBAL PROPERTY USE_FOLDERS ON)

set(CARGO_MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR}/target)

option(RUST_VERBOSE "Enable verbose output for Cargo build and test" OFF)

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_PROFILE "")
    set(CARGO_OUTPUT ${CARGO_TARGET_DIR}/debug/tlockr)
else()
    set(CARGO_PROFILE "--release")
    set(CARGO_OUTPUT ${CARGO_TARGET_DIR}/release/tlockr)
endif()

if(RUST_VERBOSE)
    set(CARGO_VERBOSE "--verbose")
else()
    set(CARGO_VERBOSE "")
endif()

add_custom_command(
    OUTPUT ${CARGO_OUTPUT}
    COMMAND cargo build ${CARGO_PROFILE} ${CARGO_VERBOSE} --manifest-path ${CARGO_MANIFEST_PATH} --target-dir ${CARGO_TARGET_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building tlockr with Cargo (${CMAKE_BUILD_TYPE})"
    DEPENDS tlockr_qt
)

add_custom_target(tlockr ALL
    DEPENDS ${CARGO_OUTPUT}
)

add_test(NAME rust_tests
    COMMAND cargo test ${CARGO_VERBOSE} --manifest-path ${CARGO_MANIFEST_PATH} --target-dir ${CARGO_TARGET_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

set_tests_properties(rust_tests PROPERTIES
    DEPENDS tlockr
)

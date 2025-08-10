// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#pragma once

#ifndef FFI_HPP
#define FFI_HPP

#include <cstddef>
#include <cstdlib>
#include <cstring>

#ifdef __cplusplus
extern "C" {
#endif

/// `ForeignBuffer` structure for sending and receiving buffers
/// from threads written in languages other than C or C++.
///
/// This is mainly provided by the `dealloc` member of this
/// struct, which is a function pointer to the buffer's
/// deallocator.
struct ForeignBuffer {
    void *data;
    size_t len;
    void (*dealloc)(void *);
};

/// C deallocator for use in FFI buffer deallocation
void c_free(void *p);

/// Create a `ForeignBuffer` from a buffer pointer
ForeignBuffer *build_ffi_buffer(void *data, size_t len);

#ifdef __cplusplus
}
#endif

#endif

// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

#include "ffi.hpp"
#include "logging.hpp"

static const char *FILENAME = "tlockr_qt/ffi.cpp";

/// C deallocator for use in FFI buffer deallocation
void c_free(void *p) { std::free(p); }

/// Create a `ForeignBuffer` from a buffer pointer
ForeignBuffer *build_ffi_buffer(void *buf, size_t len) {
    void *data = std::malloc(len + 1);

    if (data == nullptr) {
        error_log(
            FILENAME,
            format_log("Buffer allocation of ", len, " bytes failed.").c_str());
        return nullptr;
    }

    std::memcpy(data, buf, len);

    char *cdata = static_cast<char *>(data);
    cdata[len] = '\0';

    ForeignBuffer *fb =
        static_cast<ForeignBuffer *>(std::malloc(sizeof(ForeignBuffer)));

    if (fb == nullptr) {
        error_log(FILENAME, format_log("Buffer allocation of ",
                                       sizeof(ForeignBuffer), " bytes failed.")
                                .c_str());
        return nullptr;
    }

    fb->data = data;
    fb->len = len;
    fb->dealloc = &c_free;

    return fb;
}


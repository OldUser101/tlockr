#pragma once

#ifndef RENDER_HPP
#define RENDER_HPP

#ifdef __cplusplus
extern "C"
{
#endif

    int render_single_frame(const char *qml_path, int width, int height, void *buffer);

#ifdef __cplusplus
}
#endif

#endif

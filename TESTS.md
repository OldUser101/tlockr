# tlockr tests

The `test` directory contains some helpful tests for the renderer.

- `test_lock.qml`
    - Shows a mock "lock screen" with a central panel.
    - Most basic test file.
- `test_anim.qml`
    - Shows a blue square sliding horizontally across the screen.
    - Good for testing video output.
- `test_img.qml`
    - Shows a full screen image with blur.
    - Good for testing graphical effects.
    - **NOTE**: Requires full path to the image to be set in the QML file before running.

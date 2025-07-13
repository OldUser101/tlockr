import QtQuick
import Qt5Compat.GraphicalEffects

Rectangle {
    width: 800
    height: 600
    color: "black"
    
    Image {
        id: sourceImage
        anchors.fill: parent
        source: "file://"                   // If you want to use this test, fill the absolute path to `test_img.jpg` in this directory
        fillMode: Image.PreserveAspectCrop
        smooth: true
        visible: false
    }
    
    GaussianBlur {
        anchors.fill: parent
        source: sourceImage
        radius: 32
        samples: 32
        deviation: 12
    }
}

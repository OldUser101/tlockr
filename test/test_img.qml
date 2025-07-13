import QtQuick
import Qt5Compat.GraphicalEffects

Rectangle {
    width: 800
    height: 600
    color: "black"
    
    Image {
        id: sourceImage
        anchors.fill: parent
        source: Qt.resolvedUrl("test_img.jpg")
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

import QtQuick 2.15

Rectangle {
    id: root
    width: 320
    height: 240
    color: "#222"

    Rectangle {
        id: movingRect
        width: 60
        height: 60
        color: "deepskyblue"
        radius: 12
        y: (root.height - height) / 2
        x: 0

        SequentialAnimation on x {
            loops: Animation.Infinite
            NumberAnimation {
                to: root.width - movingRect.width
                duration: 10000
                easing.type: Easing.InOutQuad
            }
            NumberAnimation {
                to: 0
                duration: 10000
                easing.type: Easing.InOutQuad
            }
        }
    }
}
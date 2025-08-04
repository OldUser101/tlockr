import QtQuick 6.6
import QtQuick.Window 6.6

Item {
    id: root
    width: 800
    height: 600

    Rectangle {
        id: background
        anchors.fill: parent
        color: mouseArea.mouseX < root.width / 2 ? "skyblue" : "salmon"

        MouseArea {
            id: mouseArea
            anchors.fill: parent
            hoverEnabled: true
        }

        Text {
            anchors.centerIn: parent
            text: `Mouse: (${mouseArea.mouseX.toFixed(0)}, ${mouseArea.mouseY.toFixed(0)})`
            font.pixelSize: 20
        }
    }
}

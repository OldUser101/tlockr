import QtQuick 6.6
import QtQuick.Window 6.6

Item {
    id: root
    width: 800
    height: 600

    property bool pressed: false
    property string buttonText: "None"

    Rectangle {
        id: background
        anchors.fill: parent
        color: root.pressed ? "red" : "blue"

        MouseArea {
            id: mouseArea
            anchors.fill: parent
            hoverEnabled: true
            acceptedButtons: Qt.AllButtons

            onPressed: (mouse) => {
                root.pressed = true;
                root.buttonText = `${mouse.button}`;
            }

            onReleased: {
                root.pressed = false;
            }
        }

        Text {
            anchors.centerIn: parent
            text: `Mouse: (${mouseArea.mouseX.toFixed(0)}, ${mouseArea.mouseY.toFixed(0)})\nPressed: ${root.buttonText}`
            font.pixelSize: 20
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
        }
    }
}

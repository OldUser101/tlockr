import QtQuick 6.0

Rectangle {
    id: root
    width: 400
    height: 300
    focus: true
    color: colors[colorIndex]

    property int colorIndex: 0
    property var colors: ["black", "darkblue", "darkgreen", "darkred", "darkmagenta", "darkcyan"]
    property string lastKey: ""

    Keys.onReleased: (event) => {
        colorIndex = (colorIndex + 1) % colors.length
        lastKey = event.text !== "" ? event.text : event.key
    }

    Text {
        anchors.centerIn: parent
        text: lastKey === "" ? "Press any key" : "Released: " + lastKey
        font.pixelSize: 24
        color: "white"
    }
}

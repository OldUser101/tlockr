import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    width: 1920
    height: 1200
    color: "#1e1e1e"
    
    Rectangle {
        anchors.centerIn: parent
        width: 400
        height: 300
        color: "#2d2d2d"
        radius: 10
        border.color: "#444444"
        border.width: 1
        
        Column {
            anchors.centerIn: parent
            spacing: 20
            
            Text {
                text: "Screen Lock"
                color: "#ffffff"
                font.pixelSize: 24
                font.bold: true
                anchors.horizontalCenter: parent.horizontalCenter
            }
            
            Rectangle {
                width: 300
                height: 40
                color: "#404040"
                radius: 5
                border.color: "#555555"
                border.width: 1
                
                TextInput {
                    id: passwordInput
                    anchors.fill: parent
                    anchors.margins: 10
                    color: "#ffffff"
                    font.pixelSize: 16
                    echoMode: TextInput.Password
                    
                    Rectangle {
                        anchors.fill: parent
                        color: "transparent"
                        Text {
                            color: "#888888"
                            font.pixelSize: passwordInput.font.pixelSize
                            visible: passwordInput.text.length === 0
                            anchors.verticalCenter: parent.verticalCenter
                            anchors.left: parent.left
                            anchors.leftMargin: 5
                        }
                    }
                }
            }
            
            Button {
                text: "Unlock"
                width: 100
                height: 35
                anchors.horizontalCenter: parent.horizontalCenter
                
                background: Rectangle {
                    color: parent.pressed ? "#0056b3" : "#007bff"
                    radius: 5
                    border.color: "#0056b3"
                    border.width: 1
                }
                
                contentItem: Text {
                    text: parent.text
                    color: "#ffffff"
                    font.pixelSize: 14
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
            }
        }
    }
    
    Text {
        anchors.bottom: parent.bottom
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.bottomMargin: 50
        text: "TLockR - Session Lock Test"
        color: "#666666"
        font.pixelSize: 12
    }
}
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

import QtQuick 2.15
import QtQuick.Controls 2.0

import Qt5Compat.GraphicalEffects

Rectangle {
    id: root
    width: 640
    height: 480

    Connections {
        target: tlockr
        function onAuthStateChange(state) {
            if (state == 1) {   // Login failed
                backgroundBorder.border.width = 5;
                animateBorder.restart();
                passwordInput.clear(); 
            } else if (state == 2) {    // Login success
                backgroundBorder.border.width = 0;
                animateBorder.stop();
            }
        }
    }

    Item {
        id: mainFrame
        width: tlockr.Width
        height: tlockr.Height
        
        Rectangle {
            id: background
            visible: true
            anchors.fill: parent
            color: "#FFFFFF"

            Rectangle {
                id: backgroundBorder
                anchors.fill: parent
                z: 4
                border.color: "#FF0000"
                border.width: 0
                color: "transparent"

                Behavior on border.width {
                    SequentialAnimation {
                        id: animateBorder
                        running: false
                        loops: Animation.Infinite                        
                        NumberAnimation { from: 5; to: 10; duration: 700 }
                        NumberAnimation { from: 10; to: 5; duration: 400 }
                    }
                }
            }          
        }

        TextInput {
            id: passwordInput
            width: parent.width * 0.5
            height: 200
            font.pointSize: 96
            font.bold: true
            font.letterSpacing: 20
            font.family: "monospace"
            anchors {
                verticalCenter: parent.verticalCenter
                horizontalCenter: parent.horizontalCenter
            }
            echoMode: TextInput.Password
            color: "#000000"
            selectionColor: "#000000"
            selectedTextColor: "#FFFFFF"
            clip: true
            horizontalAlignment: TextInput.AlignHCenter
            verticalAlignment: TextInput.AlignVCenter
            passwordCharacter: "*"
            cursorVisible: false
            activeFocusOnTab: false
            focus: true

            onAccepted: {
                if (text != "") {
                    tlockr.sendAuthSubmit(text);
                }
            }
        }

        Component.onCompleted: {
            Qt.callLater(function() {
                passwordInput.forceActiveFocus();
                passwordInput.cursorVisible = false;
            });
        }
    }

    Loader {
        active: true
        anchors.fill: parent
        sourceComponent: MouseArea {
            enabled: false
            cursorShape: Qt.BlankCursor
        }
    }
}

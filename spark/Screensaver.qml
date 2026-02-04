import QtQuick
import Quickshell
import Quickshell.Wayland

PanelWindow {
    id: root
    
    // --- Layout ---
    anchors {
        top: true
        bottom: true
        left: true
        right: true
    }
    
    WlrLayershell.layer: WlrLayer.Overlay
    WlrLayershell.namespace: "screensaver"
    WlrLayershell.keyboardFocus: WlrKeyboardFocus.Exclusive
    
    // Default colors (Hardcoded from Fuse config)
    property color colorBackground: "#000000"
    property color colorText: "#ffffff"
    property color colorAccent: "#c0c0c0"
    property color colorSecondary: "#080808"

    color: "transparent" 

    // Explicit solid background
    Rectangle {
        anchors.fill: parent
        color: root.colorBackground
        z: -100
    }

    // --- Color Loading (Backup) ---
    Component.onCompleted: {
        root.contentItem.forceActiveFocus()
        loadColors()
    }
    
    function loadColors() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file://" + "/home/igora/.config/alloy/colors.json")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
                try {
                    var json = JSON.parse(xhr.responseText)
                    if (json.background) root.colorBackground = json.background
                    if (json.text) root.colorText = json.text
                    if (json.accent) root.colorAccent = json.accent
                    if (json.secondary) root.colorSecondary = json.secondary
                } catch (e) {}
            }
        }
        xhr.send()
    }

    // --- Background Grid ---
    Item {
        anchors.fill: parent
        opacity: 0.15
        
        Repeater {
            model: 6
            delegate: Rectangle {
                width: parent.width; height: 1
                color: root.colorText
                y: parent.height / 5 * index
            }
        }
        Repeater {
            model: 6
            delegate: Rectangle {
                width: 1; height: parent.height
                color: root.colorText
                x: parent.width / 5 * index
            }
        }
    }

    // --- Main Layout ---
    Item {
        id: mainContainer
        anchors.fill: parent
        anchors.margins: 100
        
        // 1. Hours
        Text {
            id: hourText
            text: Qt.formatTime(new Date(), "HH")
            font.family: "Inter, Roboto, sans-serif"
            font.weight: Font.Black
            font.pixelSize: 200
            color: root.colorText
            anchors.left: parent.left
            anchors.top: parent.top
        }
        
        // 2. Minutes
        Text {
            id: minText
            text: Qt.formatTime(new Date(), "mm")
            font.family: "Inter, Roboto, sans-serif"
            font.weight: Font.Black
            font.pixelSize: 200
            color: root.colorText
            anchors.left: parent.left
            anchors.top: hourText.bottom
            anchors.topMargin: -20
        }
        
        // 3. Date
        Text {
            id: dateText
            text: Qt.formatDate(new Date(), "yyyy-MM-dd").toUpperCase() + "\n" + Qt.formatDate(new Date(), "dddd").toUpperCase()
            font.family: "Monospace"
            font.pixelSize: 24
            font.bold: true
            color: root.colorText
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            horizontalAlignment: Text.AlignRight
        }

        // --- Focus Ring ---
        Rectangle {
            id: focusRing
            color: "transparent"
            border.color: root.colorAccent
            border.width: 4
            
            x: hourText.x - 20; y: hourText.y
            width: hourText.contentWidth + 40; height: hourText.contentHeight
            
            SequentialAnimation {
                running: true; loops: Animation.Infinite
                ParallelAnimation {
                    NumberAnimation { target: focusRing; property: "x"; to: hourText.x - 20; duration: 800; easing.type: Easing.InOutQuart }
                    NumberAnimation { target: focusRing; property: "y"; to: hourText.y + 15; duration: 800; easing.type: Easing.InOutQuart }
                    NumberAnimation { target: focusRing; property: "width"; to: hourText.contentWidth + 40; duration: 800; easing.type: Easing.InOutQuart }
                    NumberAnimation { target: focusRing; property: "height"; to: hourText.contentHeight - 20; duration: 800; easing.type: Easing.InOutQuart }
                }
                PauseAnimation { duration: 4000 }
                ParallelAnimation {
                    NumberAnimation { target: focusRing; property: "x"; to: minText.x - 20; duration: 800; easing.type: Easing.InOutQuart }
                    NumberAnimation { target: focusRing; property: "y"; to: minText.y + 15; duration: 800; easing.type: Easing.InOutQuart }
                    NumberAnimation { target: focusRing; property: "width"; to: minText.contentWidth + 40; duration: 800; easing.type: Easing.InOutQuart }
                    NumberAnimation { target: focusRing; property: "height"; to: minText.contentHeight - 20; duration: 800; easing.type: Easing.InOutQuart }
                }
                PauseAnimation { duration: 4000 }
                ParallelAnimation {
                    NumberAnimation { target: focusRing; property: "x"; to: dateText.x - 20; duration: 800; easing.type: Easing.InOutQuart }
                    NumberAnimation { target: focusRing; property: "y"; to: dateText.y - 10; duration: 800; easing.type: Easing.InOutQuart }
                    NumberAnimation { target: focusRing; property: "width"; to: dateText.contentWidth + 40; duration: 800; easing.type: Easing.InOutQuart }
                    NumberAnimation { target: focusRing; property: "height"; to: dateText.contentHeight + 20; duration: 800; easing.type: Easing.InOutQuart }
                }
                PauseAnimation { duration: 3000 }
            }
        }
    }

    // --- Seconds Bar ---
    Rectangle {
        id: secondsBar
        anchors.bottom: parent.bottom; anchors.left: parent.left
        height: 6; color: root.colorAccent; width: 0 
    }
    
    Timer {
        interval: 16; repeat: true; running: true
        onTriggered: {
            var d = new Date()
            if (d.getSeconds() !== _lastSeconds) {
                hourText.text = Qt.formatTime(d, "HH")
                minText.text = Qt.formatTime(d, "mm")
                _lastSeconds = d.getSeconds()
            }
            var progress = (d.getSeconds() + d.getMilliseconds()/1000.0) / 60.0
            secondsBar.width = root.width * progress
        }
        property int _lastSeconds: -1
    }

    MouseArea {
        anchors.fill: parent; cursorShape: Qt.BlankCursor
        onClicked: Qt.quit()
    }
    Item {
        focus: true
        Keys.onPressed: event => Qt.quit()
    }
}

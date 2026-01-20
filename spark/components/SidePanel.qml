import Quickshell
import Quickshell.Wayland
import Quickshell.Hyprland
import QtQuick
import QtQuick.Shapes

PanelWindow {
    id: sidePanel
    
    required property var screen
    required property string panelPosition  // "left" or "top" - determines which panel this is
    property string projectPath: ""  // Will be set from environment or auto-detected
    
    screen: sidePanel.screen
    
    // Anchors based on panel position
    anchors.left: panelPosition === "left" ? true : false
    anchors.right: panelPosition === "top" ? true : false
    anchors.top: true
    anchors.bottom: panelPosition === "left" ? true : false
    
    // Dimensions based on panel position
    implicitWidth: panelPosition === "left" ? 36 : (panelPosition === "top" ? (screen ? screen.width : 1920) : 0)
    implicitHeight: panelPosition === "top" ? 36 : (panelPosition === "left" ? (screen ? screen.height : 1080) : 0)
    color: "transparent"

    // Detect if any workspace on this screen has a fullscreen window
    property bool isFullscreenActive: {
        if (!Hyprland || !Hyprland.workspaces || !screen) return false;
        return Hyprland.workspaces.values.some(w => 
            w.monitor && w.monitor.name === screen.name && 
            w.lastIpcObject && w.lastIpcObject.hasfullscreen
        );
    }

    // Visible only when this panel's position matches the current sidebar position and not in fullscreen
    visible: (sharedData && sharedData.sidebarVisible !== undefined ? sharedData.sidebarVisible : true) && 
             (sharedData && sharedData.sidebarPosition === panelPosition) &&
             !isFullscreenActive
    
    WlrLayershell.layer: WlrLayer.Overlay
    WlrLayershell.namespace: "qssidepanel"
    exclusiveZone: visible ? ((panelPosition === "top") ? implicitHeight : implicitWidth) : 0
    
    property var sharedData: null
    
    // Margins based on panel position
    margins {
        left: 0
        top: 0
        bottom: panelPosition === "left" ? 0 : 0
        right: panelPosition === "top" ? 0 : 0

        // Smooth animation when switching panel positions
        Behavior on bottom {
            NumberAnimation {
                duration: 400
                easing.type: Easing.OutCubic
            }
        }
        Behavior on right {
            NumberAnimation {
                duration: 400
                easing.type: Easing.OutCubic
            }
        }
    }
    
    // Background Rectangle - separate from buttons to avoid blocking clicks
    // Material Design background with elevation shadow
    Rectangle {
        id: sidePanelRect
        anchors.fill: parent
        color: (sharedData && sharedData.colorBackground) ? sharedData.colorBackground : "#0d0d0d"
        radius: 0
        enabled: false  // Don't capture mouse events - allows clicks to pass through
        z: -1  // Put background behind everything to ensure buttons are clickable
        
        // Material Design elevation shadow (simulated with border)
        Rectangle {
            anchors.fill: parent
            anchors.margins: -2
            color: "transparent"
            border.color: Qt.rgba(0, 0, 0, 0.2)  // Material shadow
            border.width: 1
            z: -2
        }
        
        // Smooth fade animation when panel appears/disappears
        opacity: sidePanel.visible ? 1.0 : 0.0
        Behavior on opacity {
            NumberAnimation {
                duration: 300
                easing.type: Easing.OutCubic
            }
        }
    }
    
    // MouseArea to close menus when clicking outside
    MouseArea {
        anchors.fill: parent
        z: 99999
        acceptedButtons: Qt.LeftButton
        propagateComposedEvents: true
        onClicked: function(mouse) {
            // Close all menus when clicking outside
            bluetoothMenuVisible = false
            networkMenuVisible = false
            powerMenuVisible = false
            mouse.accepted = false
        }
    }
    
    // Container for all sidebar content (clock, workspace switcher, visualizer)
    Item {
        id: sidePanelContent
        anchors.fill: parent
        enabled: true  // Must be enabled for MouseArea inside to work
        z: 0  // Above background but below buttons (z: 10000)
        clip: false  // Don't clip children (buttons are outside)
        
        // Zegar - layout zależy od pozycji sidebara
        // Pionowy zegar dla pozycji left
        Column {
            id: sidePanelClockColumn
            anchors.top: parent.top
            anchors.topMargin: 14
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: 4
            visible: panelPosition === "left"

            // Smooth fade when switching panel positions
            opacity: visible ? 1.0 : 0.0
            Behavior on opacity {
                NumberAnimation {
                    duration: 300
                    easing.type: Easing.OutCubic
                }
            }
            
            Text {
                id: sidePanelHoursDisplay
                text: "00"
                font.pixelSize: 20
                font.family: "sans-serif"
                font.weight: Font.Bold
                color: (sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff"
                horizontalAlignment: Text.AlignHCenter
                
                Behavior on color {
                    ColorAnimation {
                        duration: 280
                        easing.type: Easing.OutQuart
                    }
                }
            }
            
            Text {
                id: sidePanelMinutesDisplay
                text: "00"
                font.pixelSize: 20
                font.family: "sans-serif"
                font.weight: Font.Bold
                color: (sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff"
                horizontalAlignment: Text.AlignHCenter
                
                Behavior on color {
                    ColorAnimation {
                        duration: 280
                        easing.type: Easing.OutQuart
                    }
                }
            }
        }
        
        // Poziomy zegar dla pozycji top
        Row {
            id: sidePanelClockRow
            anchors.left: parent.left
            anchors.leftMargin: 14
            anchors.verticalCenter: parent.verticalCenter
            spacing: 4
            visible: panelPosition === "top"

            // Smooth fade when switching panel positions
            opacity: visible ? 1.0 : 0.0
            Behavior on opacity {
                NumberAnimation {
                    duration: 300
                    easing.type: Easing.OutCubic
                }
            }
            
            Text {
                id: sidePanelHoursDisplayTop
                text: "00"
                font.pixelSize: 20
                font.family: "sans-serif"
                font.weight: Font.Bold
                color: (sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff"
                verticalAlignment: Text.AlignVCenter
                
                Behavior on color {
                    ColorAnimation {
                        duration: 280
                        easing.type: Easing.OutQuart
                    }
                }
            }
            
            Text {
                text: ":"
                font.pixelSize: 20
                font.family: "sans-serif"
                font.weight: Font.Bold
                color: (sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff"
                verticalAlignment: Text.AlignVCenter
            }
            
            Text {
                id: sidePanelMinutesDisplayTop
                text: "00"
                font.pixelSize: 20
                font.family: "sans-serif"
                font.weight: Font.Bold
                color: (sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff"
                verticalAlignment: Text.AlignVCenter
                
                Behavior on color {
                    ColorAnimation {
                        duration: 280
                        easing.type: Easing.OutQuart
                    }
                }
            }
        }
        
        Timer {
            id: sidePanelClockTimer
            interval: 1000
            repeat: true
            running: true
            onTriggered: {
                var now = new Date()
                var h = now.getHours()
                var m = now.getMinutes()
                var hStr = h < 10 ? "0" + h : h.toString()
                var mStr = m < 10 ? "0" + m : m.toString()
                sidePanelHoursDisplay.text = hStr
                sidePanelMinutesDisplay.text = mStr
                sidePanelHoursDisplayTop.text = hStr
                sidePanelMinutesDisplayTop.text = mStr
            }
            Component.onCompleted: {
                var now = new Date()
                var h = now.getHours()
                var m = now.getMinutes()
                var hStr = h < 10 ? "0" + h : h.toString()
                var mStr = m < 10 ? "0" + m : m.toString()
                sidePanelHoursDisplay.text = hStr
                sidePanelMinutesDisplay.text = mStr
                sidePanelHoursDisplayTop.text = hStr
                sidePanelMinutesDisplayTop.text = mStr
            }
        }
        
        // Workspace switcher - pionowy dla pozycji left
        Item {
            id: sidePanelWorkspaceColumnContainer
            width: 8
            height: parent.height
            visible: panelPosition === "left"
            anchors.horizontalCenter: parent.horizontalCenter
            anchors.verticalCenter: parent.verticalCenter
            z: 50  // Lower than buttons (z: 10000) to not block clicks
            
            Column {
                id: sidePanelWorkspaceColumn
                spacing: 9
                width: parent.width
                x: (parent.width - width) / 2
                y: (parent.height - height) / 2
                
                Repeater {
                    model: 4  // Workspaces 1-4
                
                Item {
                    id: workspaceItem
                    width: 8  // Większa szerokość tylko dla MouseArea
                    height: workspaceLine.height
                    anchors.horizontalCenter: parent.horizontalCenter
                    
                    property bool isActive: Hyprland.focusedWorkspace ? 
                        Hyprland.focusedWorkspace.id === (index + 1) : false
                    property bool hasWindows: {
                        var ws = Hyprland.workspaces.values.find(w => w.id === (index + 1))
                        return ws ? ws.lastIpcObject.windows > 0 : false
                    }
                    property bool wasActive: false
                    
                    onIsActiveChanged: {
                        if (isActive && !wasActive) {
                            workspaceActivateAnim.restart()
                        }
                        wasActive = isActive
                    }
                    
                    Component.onCompleted: wasActive = isActive
                    
                    // Pionowa linia z lepszymi wskaźnikami
                    Rectangle {
                        id: workspaceLine
                        anchors.centerIn: parent
                        width: workspaceItem.isActive ? 5 : (workspaceItem.hasWindows ? 3.5 : 3)
                        height: workspaceItem.isActive ? 45 : (workspaceItem.hasWindows ? 36 : 30)
                        color: workspaceItem.isActive ? 
                            ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff") : 
                            workspaceItem.hasWindows ? 
                            ((sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#3a3a3a") : 
                            ((sharedData && sharedData.colorSecondary) ? sharedData.colorSecondary : "#2a2a2a")
                        radius: 0
                        opacity: workspaceItem.isActive ? 1.0 : (workspaceItem.hasWindows ? 0.8 : 0.5)
                        
                        Behavior on width {
                            NumberAnimation { 
                                duration: 400
                                easing.type: Easing.OutCubic
                            }
                        }
                        
                        Behavior on height {
                            NumberAnimation { 
                                duration: 400
                                easing.type: Easing.OutCubic
                            }
                        }
                        
                        Behavior on color {
                            ColorAnimation { 
                                duration: 400
                                easing.type: Easing.OutCubic
                            }
                        }
                        
                        Behavior on opacity {
                            NumberAnimation { 
                                duration: 400
                                easing.type: Easing.OutCubic
                            }
                        }
                        
                        Behavior on scale {
                            NumberAnimation { 
                                duration: 300
                                easing.type: Easing.OutCubic
                            }
                        }
                    }
                    
                    // Animacja aktywacji - płynniejsza
                    SequentialAnimation {
                        id: workspaceActivateAnim
                        ParallelAnimation {
                            NumberAnimation {
                                target: workspaceLine
                                property: "scale"
                                from: 0.6
                                to: 1.15
                                duration: 300
                                easing.type: Easing.OutCubic
                            }
                            NumberAnimation {
                                target: workspaceLine
                                property: "opacity"
                                from: 0.5
                                to: 1.0
                                duration: 300
                                easing.type: Easing.OutCubic
                            }
                        }
                        NumberAnimation {
                            target: workspaceLine
                            property: "scale"
                            to: 1.0
                            duration: 250
                            easing.type: Easing.OutCubic
                        }
                    }
                    
                    MouseArea {
                        id: workspaceMouseArea
                        anchors.fill: workspaceLine  // Tylko w obszarze workspace line, nie całego item
                        anchors.margins: -2  // Mały margines tylko dla łatwiejszego klikania
                        hoverEnabled: true
                        propagateComposedEvents: true  // Pozwól na propagację zdarzeń poza workspace
                        z: 1  // Very low z to ensure buttons (z: 10000) are on top
                        acceptedButtons: Qt.LeftButton
                        
                        onEntered: {
                            if (!workspaceItem.isActive) {
                                workspaceLine.scale = 1.2
                                workspaceLine.opacity = Math.min(workspaceLine.opacity + 0.2, 1.0)
                            }
                        }
                        
                        onExited: {
                            if (!workspaceItem.isActive) {
                                workspaceLine.scale = 1.0
                                workspaceLine.opacity = workspaceItem.hasWindows ? 0.8 : 0.5
                            }
                        }
                        
                        onClicked: {
                            workspaceClickAnim.restart()
                            Hyprland.dispatch("workspace", index + 1)
                        }
                    }
                    
                    // Animacja kliknięcia - płynniejsza
                    SequentialAnimation {
                        id: workspaceClickAnim
                        NumberAnimation {
                            target: workspaceLine
                            property: "scale"
                            to: 0.75
                            duration: 100
                            easing.type: Easing.InCubic
                        }
                        NumberAnimation {
                            target: workspaceLine
                            property: "scale"
                            to: 1.0
                            duration: 300
                            easing.type: Easing.OutCubic
                        }
                    }
                }
                }
            }
        }
        
        // Workspace switcher - poziomy dla pozycji top
        Item {
            id: sidePanelWorkspaceRowContainer
            width: parent.width
            height: 8
            visible: panelPosition === "top"
            anchors.horizontalCenter: parent.horizontalCenter
            anchors.verticalCenter: parent.verticalCenter
            z: 50  // Lower than buttons (z: 10000) to not block clicks
            
            Row {
                id: sidePanelWorkspaceRow
                spacing: 9
                height: parent.height
                x: (parent.width - width) / 2
                y: (parent.height - height) / 2
                
                Repeater {
                    model: 4  // Workspaces 1-4
                
                    Item {
                        id: workspaceItemTop
                        height: 8  // Większa wysokość tylko dla MouseArea
                        width: workspaceLineTop.width
                        anchors.verticalCenter: parent.verticalCenter
                    
                    property bool isActive: Hyprland.focusedWorkspace ? 
                        Hyprland.focusedWorkspace.id === (index + 1) : false
                    property bool hasWindows: {
                        var ws = Hyprland.workspaces.values.find(w => w.id === (index + 1))
                        return ws ? ws.lastIpcObject.windows > 0 : false
                    }
                    property bool wasActive: false
                    
                    onIsActiveChanged: {
                        if (isActive && !wasActive) {
                            workspaceActivateAnimTop.restart()
                        }
                        wasActive = isActive
                    }
                    
                    Component.onCompleted: wasActive = isActive
                    
                    // Pozioma linia z lepszymi wskaźnikami
                    Rectangle {
                        id: workspaceLineTop
                        anchors.centerIn: parent
                        height: workspaceItemTop.isActive ? 5 : (workspaceItemTop.hasWindows ? 3.5 : 3)
                        width: workspaceItemTop.isActive ? 45 : (workspaceItemTop.hasWindows ? 36 : 30)
                        color: workspaceItemTop.isActive ? 
                            ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff") : 
                            workspaceItemTop.hasWindows ? 
                            ((sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#3a3a3a") : 
                            ((sharedData && sharedData.colorSecondary) ? sharedData.colorSecondary : "#2a2a2a")
                        radius: 0
                        opacity: workspaceItemTop.isActive ? 1.0 : (workspaceItemTop.hasWindows ? 0.8 : 0.5)
                        
                        Behavior on width {
                            NumberAnimation { 
                                duration: 400
                                easing.type: Easing.OutCubic
                            }
                        }
                        
                        Behavior on height {
                            NumberAnimation { 
                                duration: 400
                                easing.type: Easing.OutCubic
                            }
                        }
                        
                        Behavior on color {
                            ColorAnimation { 
                                duration: 400
                                easing.type: Easing.OutCubic
                            }
                        }
                        
                        Behavior on opacity {
                            NumberAnimation { 
                                duration: 400
                                easing.type: Easing.OutCubic
                            }
                        }
                        
                        Behavior on scale {
                            NumberAnimation { 
                                duration: 300
                                easing.type: Easing.OutCubic
                            }
                        }
                    }
                    
                    // Animacja aktywacji - płynniejsza
                    SequentialAnimation {
                        id: workspaceActivateAnimTop
                        ParallelAnimation {
                            NumberAnimation {
                                target: workspaceLineTop
                                property: "scale"
                                from: 0.6
                                to: 1.15
                                duration: 300
                                easing.type: Easing.OutCubic
                            }
                            NumberAnimation {
                                target: workspaceLineTop
                                property: "opacity"
                                from: 0.5
                                to: 1.0
                                duration: 300
                                easing.type: Easing.OutCubic
                            }
                        }
                        NumberAnimation {
                            target: workspaceLineTop
                            property: "scale"
                            to: 1.0
                            duration: 250
                            easing.type: Easing.OutCubic
                        }
                    }
                    
                    MouseArea {
                        id: workspaceMouseAreaTop
                        anchors.fill: workspaceLineTop  // Tylko w obszarze workspace line, nie całego item
                        anchors.margins: -2  // Mały margines tylko dla łatwiejszego klikania
                        hoverEnabled: true
                        propagateComposedEvents: true  // Pozwól na propagację zdarzeń poza workspace
                        z: 1  // Very low z to ensure buttons (z: 10000) are on top
                        acceptedButtons: Qt.LeftButton
                        
                        onEntered: {
                            if (!workspaceItemTop.isActive) {
                                workspaceLineTop.scale = 1.2
                                workspaceLineTop.opacity = Math.min(workspaceLineTop.opacity + 0.2, 1.0)
                            }
                        }
                        
                        onExited: {
                            if (!workspaceItemTop.isActive) {
                                workspaceLineTop.scale = 1.0
                                workspaceLineTop.opacity = workspaceItemTop.hasWindows ? 0.8 : 0.5
                            }
                        }
                        
                        onClicked: {
                            workspaceClickAnimTop.restart()
                            Hyprland.dispatch("workspace", index + 1)
                        }
                    }
                    
                    // Animacja kliknięcia - płynniejsza
                    SequentialAnimation {
                        id: workspaceClickAnimTop
                        NumberAnimation {
                            target: workspaceLineTop
                            property: "scale"
                            to: 0.75
                            duration: 100
                            easing.type: Easing.InCubic
                        }
                        NumberAnimation {
                            target: workspaceLineTop
                            property: "scale"
                            to: 1.0
                            duration: 300
                            easing.type: Easing.OutCubic
                        }
                    }
                }
                }
            }
        }
        
        // Music Visualizer - PIONOWY dla pozycji left
        Item {
            id: musicVisualizerColumnContainer
            width: 24
            height: parent.height - 130  // Height minus space for system icons and screenshot button
            visible: panelPosition === "left"
            anchors.horizontalCenter: parent.horizontalCenter
            anchors.bottom: parent.bottom
            anchors.bottomMargin: 130  // Above system icons (90px + 10px spacing) and screenshot button (32px + spacing)
            z: 0  // Lower z-order to ensure buttons are clickable
            
            Column {
                id: musicVisualizerColumn
                spacing: 2
                width: parent.width
                anchors.bottom: parent.bottom

                // Smooth fade when switching panel positions
                opacity: parent.visible ? 1.0 : 0.0
                Behavior on opacity {
                    NumberAnimation {
                        duration: 300
                        easing.type: Easing.OutCubic
                    }
                }
                
                Repeater {
                    id: visualizerBarsRepeater
                    model: 36  // 36 pasków pionowo - 3x dłuższy visualizer
                
                    Rectangle {
                        id: visualizerBar
                        height: 3  // Grubość paska
                        width: Math.max(3, visualizerBarValue)  // Szerokość zależy od audio
                        x: (parent.width - width) / 2  // Wyśrodkuj bez anchors
                        color: (sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff"
                        radius: 0
                        visible: true
                        
                        property real visualizerBarValue: 5  // Start z widoczną szerokością
                        
                        Behavior on width {
                            NumberAnimation {
                                duration: 80
                                easing.type: Easing.OutQuad
                            }
                        }
                        
                        Behavior on color {
                            ColorAnimation {
                                duration: 100
                            }
                        }
                    }
                }
            }
        }
        
        // Music Visualizer - POZIOMY dla pozycji top
        Item {
            id: musicVisualizerRowContainer
            width: parent.width
            height: 24
            visible: panelPosition === "top"
            anchors.verticalCenter: parent.verticalCenter
            anchors.right: parent.right
            anchors.rightMargin: 100  // Space for buttons on right (moved above buttons)
            z: 1

            Row {
                id: musicVisualizerRow
                spacing: 2
                height: parent.height
                width: parent.width
                x: parent.width - width

                // Smooth fade when switching panel positions
                opacity: parent.visible ? 1.0 : 0.0
                Behavior on opacity {
                    NumberAnimation {
                        duration: 300
                        easing.type: Easing.OutCubic
                    }
                }
                
                Repeater {
                    id: visualizerBarsRepeaterTop
                    model: 36  // 36 pasków poziomo
                    
                    Rectangle {
                        id: visualizerBarTop
                        width: 3  // Grubość paska
                        height: Math.max(3, visualizerBarValueTop)  // Wysokość zależy od audio
                        y: (parent.height - height) / 2  // Wyśrodkuj bez anchors
                        color: (sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff"
                        radius: 0
                        visible: true
                        
                        property real visualizerBarValueTop: 5  // Start z widoczną wysokością
                        
                        Behavior on height {
                            NumberAnimation {
                                duration: 80
                                easing.type: Easing.OutQuad
                            }
                        }
                        
                        Behavior on color {
                            ColorAnimation {
                                duration: 100
                            }
                        }
                    }
                }
            }
        }
        
    }  // End of sidePanelContent
    
    // System Icons Container - Between visualizer and screenshot button
    Item {
        id: systemIconsContainer
        width: panelPosition === "left" ? 32 : 120  // Width for 3 icons horizontally
        height: panelPosition === "left" ? 90 : 32  // Height for 3 icons vertically
        
        anchors.horizontalCenter: panelPosition === "left" ? parent.horizontalCenter : undefined
        anchors.right: panelPosition === "top" ? parent.right : undefined
        anchors.rightMargin: panelPosition === "top" ? 80 : 0  // Between visualizer (100px) and screenshot (48px)
        anchors.bottom: panelPosition === "left" ? parent.bottom : undefined
        anchors.bottomMargin: panelPosition === "left" ? 55 : 0  // Above screenshot button (10px) - 55px for 3 icons (24px each + 8px spacing each) + extra margin
        z: 100000  // Same z as screenshot button
        visible: true
        enabled: true
        
        // Smooth repositioning when panel position changes
        Behavior on anchors.rightMargin {
            NumberAnimation {
                duration: 400
                easing.type: Easing.OutCubic
            }
        }
        Behavior on anchors.bottomMargin {
            NumberAnimation {
                duration: 400
                easing.type: Easing.OutCubic
            }
        }
        
        // Common background for all icons
        Rectangle {
            id: systemIconsBackground
            anchors.fill: parent
            color: (sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#1a1a1a"
            radius: 0
            z: -1
            
            // Material Design elevation shadow
            Rectangle {
                anchors.fill: parent
                anchors.margins: -1
                color: "transparent"
                border.color: Qt.rgba(0, 0, 0, 0.2)
                border.width: 1
                z: -1
            }
        }
        
        // Vertical layout for left position
        Column {
            id: systemIconsColumn
            visible: panelPosition === "left"
            spacing: 8
            anchors.horizontalCenter: parent.horizontalCenter
            anchors.bottom: parent.bottom
            
            opacity: parent.visible ? 1.0 : 0.0
            Behavior on opacity {
                NumberAnimation {
                    duration: 300
                    easing.type: Easing.OutCubic
                }
            }
            
            // Bluetooth Icon (for left position)
            Loader {
                id: bluetoothIconLoaderLeft
                sourceComponent: bluetoothIconComponent
                active: panelPosition === "left"
            }
            
            // Network Icon (for left position)
            Loader {
                id: networkIconLoaderLeft
                sourceComponent: networkIconComponent
                active: panelPosition === "left"
            }
            
            // Power Icon (for left position)
            Loader {
                id: powerIconLoaderLeft
                sourceComponent: powerIconComponent
                active: panelPosition === "left"
            }
        }
        
        // Horizontal layout for top position
        Row {
            id: systemIconsRow
            visible: panelPosition === "top"
            spacing: 8
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
            
            opacity: parent.visible ? 1.0 : 0.0
            Behavior on opacity {
                NumberAnimation {
                    duration: 300
                    easing.type: Easing.OutCubic
                }
            }
            
            // Bluetooth Icon (for top position)
            Loader {
                id: bluetoothIconLoaderTop
                sourceComponent: bluetoothIconComponent
                active: panelPosition === "top"
            }
            
            // Network Icon (for top position)
            Loader {
                id: networkIconLoaderTop
                sourceComponent: networkIconComponent
                active: panelPosition === "top"
            }
            
            // Power Icon (for top position)
            Loader {
                id: powerIconLoaderTop
                sourceComponent: powerIconComponent
                active: panelPosition === "top"
            }
        }
    }
    
    // Bluetooth Icon Component
    Component {
        id: bluetoothIconComponent
        
        Item {
            id: bluetoothIconContainer
            width: 32
            height: 32
        
        // Icon Button
        Item {
            id: bluetoothIconButton
            width: 24
            height: 24
            anchors.centerIn: parent
            
            Rectangle {
                id: bluetoothIconRect
                anchors.fill: parent
                radius: 0
                color: bluetoothMouseArea.containsMouse ?
                    ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff") :
                    ((sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#1a1a1a")
                
                property real iconScale: bluetoothMouseArea.pressed ? 0.95 : (bluetoothMouseArea.containsMouse ? 1.05 : 1.0)
                
                Behavior on color {
                    ColorAnimation {
                        duration: 200
                        easing.type: Easing.OutQuart
                    }
                }
                
                Behavior on iconScale {
                    NumberAnimation {
                        duration: 150
                        easing.type: Easing.OutQuart
                    }
                }
                
                scale: iconScale
                
                Text {
                    text: "󰂯"  // Bluetooth icon (Nerd Fonts)
                    font.pixelSize: 14
                    anchors.centerIn: parent
                    color: bluetoothMouseArea.containsMouse ? 
                        ((sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff") : 
                        ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff")
                    
                    Behavior on color {
                        ColorAnimation {
                            duration: 200
                            easing.type: Easing.OutQuart
                        }
                    }
                }
            }
            
            MouseArea {
                id: bluetoothMouseArea
                anchors.fill: parent
                anchors.margins: -10
                cursorShape: Qt.PointingHandCursor
                hoverEnabled: true
                z: 10001
                
                onClicked: {
                    console.log("Bluetooth icon clicked, current state:", sidePanel.bluetoothMenuVisible)
                    // Toggle menu on click
                    if (sidePanel.bluetoothMenuVisible) {
                        sidePanel.bluetoothMenuVisible = false
                        console.log("Bluetooth menu hidden")
                    } else {
                        // Hide other menus
                        sidePanel.networkMenuVisible = false
                        sidePanel.powerMenuVisible = false
                        sidePanel.bluetoothMenuVisible = true
                        console.log("Bluetooth menu shown, menu height:", bluetoothMenuContent.height)
                        checkBluetoothStatus()
                    }
                }
            }
        }
        
        // Bluetooth Menu
        Rectangle {
            id: bluetoothMenu
            width: 280
            height: Math.max(200, bluetoothMenuContent.height + 20)
            parent: sidePanel  // Make menu a child of sidePanel for proper positioning
            // Position relative to sidebar: for left position, menu appears to the right of sidebar (36px + 8px = 44px)
            // For top position, menu appears below sidebar (36px + 8px = 44px)
            x: panelPosition === "left" ? 44 : ((sidePanel.width - width) / 2)
            y: panelPosition === "left" ? (systemIconsContainer.y + (systemIconsContainer.height - height) / 2) : 44
            visible: sidePanel.bluetoothMenuVisible
            enabled: visible
            z: 200000  // Very high z to ensure it's on top
            color: (sharedData && sharedData.colorBackground) ? sharedData.colorBackground : "#0d0d0d"
            border.color: (sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#1a1a1a"
            border.width: 1
            radius: 4
            
            opacity: visible ? 1.0 : 0.0
            Behavior on opacity {
                NumberAnimation {
                    duration: 200
                    easing.type: Easing.OutCubic
                }
            }
            
            Column {
                id: bluetoothMenuContent
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.top: parent.top
                anchors.margins: 10
                spacing: 12
                
                // Header
                Row {
                    width: parent.width
                    spacing: 12
                    
                    Text {
                        text: "Bluetooth"
                        font.pixelSize: 16
                        font.weight: Font.Bold
                        color: (sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff"
                        anchors.verticalCenter: parent.verticalCenter
                    }
                    
                    Rectangle {
                        width: 40
                        height: 20
                        radius: 10
                        color: bluetoothEnabled ? 
                            ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff") :
                            ((sharedData && sharedData.colorSecondary) ? sharedData.colorSecondary : "#2a2a2a")
                        anchors.verticalCenter: parent.verticalCenter
                        
                        Text {
                            text: bluetoothEnabled ? "ON" : "OFF"
                            font.pixelSize: 10
                            color: "#ffffff"
                            anchors.centerIn: parent
                        }
                        
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: toggleBluetooth()
                        }
                    }
                }
                
                // Devices List
                Column {
                    id: bluetoothDevicesList
                    width: parent.width
                    spacing: 8
                    visible: bluetoothEnabled && bluetoothDevicesModel.count > 0
                    
                    Repeater {
                        model: bluetoothDevicesModel
                        
                        Rectangle {
                            width: bluetoothDevicesList.width
                            height: 40
                            color: deviceMouseArea.containsMouse ?
                                ((sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#1a1a1a") :
                                "transparent"
                            radius: 4
                            
                            Row {
                                anchors.left: parent.left
                                anchors.right: parent.right
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.margins: 8
                                spacing: 8
                                
                                Text {
                                    text: model.name || "Unknown Device"
                                    font.pixelSize: 12
                                    color: (sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff"
                                    anchors.verticalCenter: parent.verticalCenter
                                    width: parent.width - connectButton.width - 16
                                    elide: Text.ElideRight
                                }
                                
                                Rectangle {
                                    id: connectButton
                                    width: 60
                                    height: 24
                                    radius: 4
                                    color: connectButtonMouseArea.containsMouse ?
                                        ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff") :
                                        ((sharedData && sharedData.colorSecondary) ? sharedData.colorSecondary : "#2a2a2a")
                                    anchors.verticalCenter: parent.verticalCenter
                                    
                                    Text {
                                        text: model.connected ? "Disconnect" : "Connect"
                                        font.pixelSize: 10
                                        color: "#ffffff"
                                        anchors.centerIn: parent
                                    }
                                    
                                    MouseArea {
                                        id: connectButtonMouseArea
                                        anchors.fill: parent
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: {
                                            if (model.connected) {
                                                disconnectBluetoothDevice(model.mac)
                                            } else {
                                                connectBluetoothDevice(model.mac)
                                            }
                                        }
                                    }
                                }
                            }
                            
                            MouseArea {
                                id: deviceMouseArea
                                anchors.fill: parent
                                hoverEnabled: true
                                acceptedButtons: Qt.NoButton
                            }
                        }
                    }
                }
                
                Text {
                    text: bluetoothEnabled ? "No devices found" : "Bluetooth is disabled"
                    font.pixelSize: 11
                    color: (sharedData && sharedData.colorSecondary) ? sharedData.colorSecondary : "#666666"
                    visible: !bluetoothEnabled || bluetoothDevicesModel.count === 0
                }
            }
            
            // Close menu when clicking outside
            MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton
                onClicked: {
                    // Allow clicks to pass through to menu content
                    mouse.accepted = false
                }
            }
        }
        }
    }
    
    // Network Icon Component
    Component {
        id: networkIconComponent
        
        Item {
            id: networkIconContainer
            width: 32
            height: 32
        
        property bool wifiEnabled: false
        property string currentWifi: "Not connected"
        
        // Icon Button
        Item {
            id: networkIconButton
            width: 24
            height: 24
            anchors.centerIn: parent
            
            Rectangle {
                id: networkIconRect
                anchors.fill: parent
                radius: 0
                color: networkMouseArea.containsMouse ?
                    ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff") :
                    ((sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#1a1a1a")
                
                property real iconScale: networkMouseArea.pressed ? 0.95 : (networkMouseArea.containsMouse ? 1.05 : 1.0)
                
                Behavior on color {
                    ColorAnimation {
                        duration: 200
                        easing.type: Easing.OutQuart
                    }
                }
                
                Behavior on iconScale {
                    NumberAnimation {
                        duration: 150
                        easing.type: Easing.OutQuart
                    }
                }
                
                scale: iconScale
                
                Text {
                    text: "󰤨"  // Wi-Fi icon (Nerd Fonts)
                    font.pixelSize: 14
                    anchors.centerIn: parent
                    color: networkMouseArea.containsMouse ? 
                        ((sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff") : 
                        ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff")
                    
                    Behavior on color {
                        ColorAnimation {
                            duration: 200
                            easing.type: Easing.OutQuart
                        }
                    }
                }
            }
            
            MouseArea {
                id: networkMouseArea
                anchors.fill: parent
                anchors.margins: -10
                cursorShape: Qt.PointingHandCursor
                hoverEnabled: true
                z: 10001
                
                onClicked: {
                    // Toggle menu on click
                    if (sidePanel.networkMenuVisible) {
                        sidePanel.networkMenuVisible = false
                    } else {
                        // Hide other menus
                        sidePanel.bluetoothMenuVisible = false
                        sidePanel.powerMenuVisible = false
                        sidePanel.networkMenuVisible = true
                        checkNetworkStatus()
                    }
                }
            }
        }
        
        // Network Menu
        Rectangle {
            id: networkMenu
            width: 280
            height: Math.max(200, networkMenuContent.height + 20)
            parent: sidePanel
            x: panelPosition === "left" ? 44 : ((sidePanel.width - width) / 2)
            y: panelPosition === "left" ? (systemIconsContainer.y + systemIconsContainer.height / 3 + (systemIconsContainer.height - height) / 2) : 44
            visible: sidePanel.networkMenuVisible
            enabled: visible
            z: 200000  // Very high z to ensure it's on top
            color: (sharedData && sharedData.colorBackground) ? sharedData.colorBackground : "#0d0d0d"
            border.color: (sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#1a1a1a"
            border.width: 1
            radius: 4
            
            opacity: visible ? 1.0 : 0.0
            Behavior on opacity {
                NumberAnimation {
                    duration: 200
                    easing.type: Easing.OutCubic
                }
            }
            
            Column {
                id: networkMenuContent
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.top: parent.top
                anchors.margins: 10
                spacing: 12
                
                // Header
                Row {
                    width: parent.width
                    spacing: 12
                    
                    Text {
                        text: "Wi-Fi"
                        font.pixelSize: 16
                        font.weight: Font.Bold
                        color: (sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff"
                        anchors.verticalCenter: parent.verticalCenter
                    }
                    
                    Rectangle {
                        width: 40
                        height: 20
                        radius: 10
                        color: networkIconContainer.wifiEnabled ? 
                            ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff") :
                            ((sharedData && sharedData.colorSecondary) ? sharedData.colorSecondary : "#2a2a2a")
                        anchors.verticalCenter: parent.verticalCenter
                        
                        Text {
                            text: networkIconContainer.wifiEnabled ? "ON" : "OFF"
                            font.pixelSize: 10
                            color: "#ffffff"
                            anchors.centerIn: parent
                        }
                        
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: toggleWifi()
                        }
                    }
                }
                
                // Current Connection
                Column {
                    width: parent.width
                    spacing: 4
                    visible: networkIconContainer.wifiEnabled
                    
                    Text {
                        text: "Connected to:"
                        font.pixelSize: 11
                        color: (sharedData && sharedData.colorSecondary) ? sharedData.colorSecondary : "#666666"
                    }
                    
                    Text {
                        text: networkIconContainer.currentWifi
                        font.pixelSize: 12
                        color: (sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff"
                    }
                }
            }
            
            // Close menu when clicking outside
            MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton
                onClicked: {
                    mouse.accepted = false
                }
            }
        }
        }
    }
    
    // Power Profiles Icon Component
    Component {
        id: powerIconComponent
        
        Item {
            id: powerIconContainer
            width: 32
            height: 32
        
        property string currentProfile: "balanced"
        
        // Icon Button
        Item {
            id: powerIconButton
            width: 24
            height: 24
            anchors.centerIn: parent
            
            Rectangle {
                id: powerIconRect
                anchors.fill: parent
                radius: 0
                color: powerMouseArea.containsMouse ?
                    ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff") :
                    ((sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#1a1a1a")
                
                property real iconScale: powerMouseArea.pressed ? 0.95 : (powerMouseArea.containsMouse ? 1.05 : 1.0)
                
                Behavior on color {
                    ColorAnimation {
                        duration: 200
                        easing.type: Easing.OutQuart
                    }
                }
                
                Behavior on iconScale {
                    NumberAnimation {
                        duration: 150
                        easing.type: Easing.OutQuart
                    }
                }
                
                scale: iconScale
                
                Text {
                    text: "󰂄"  // Power profile icon (Nerd Fonts - battery)
                    font.pixelSize: 14
                    anchors.centerIn: parent
                    color: powerMouseArea.containsMouse ? 
                        ((sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff") : 
                        ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff")
                    
                    Behavior on color {
                        ColorAnimation {
                            duration: 200
                            easing.type: Easing.OutQuart
                        }
                    }
                }
            }
            
            MouseArea {
                id: powerMouseArea
                anchors.fill: parent
                anchors.margins: -10
                cursorShape: Qt.PointingHandCursor
                hoverEnabled: true
                z: 10001
                
                onClicked: {
                    // Toggle menu on click
                    if (sidePanel.powerMenuVisible) {
                        sidePanel.powerMenuVisible = false
                    } else {
                        // Hide other menus
                        sidePanel.bluetoothMenuVisible = false
                        sidePanel.networkMenuVisible = false
                        sidePanel.powerMenuVisible = true
                        checkPowerProfile()
                    }
                }
            }
        }
        
        // Power Profiles Menu
        Rectangle {
            id: powerMenu
            width: 280
            height: Math.max(200, powerMenuContent.height + 20)
            parent: sidePanel
            x: panelPosition === "left" ? 44 : ((sidePanel.width - width) / 2)
            y: panelPosition === "left" ? (systemIconsContainer.y + 2 * systemIconsContainer.height / 3 + (systemIconsContainer.height - height) / 2) : 44
            visible: sidePanel.powerMenuVisible
            enabled: visible
            z: 200000  // Very high z to ensure it's on top
            color: (sharedData && sharedData.colorBackground) ? sharedData.colorBackground : "#0d0d0d"
            border.color: (sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#1a1a1a"
            border.width: 1
            radius: 4
            
            opacity: visible ? 1.0 : 0.0
            Behavior on opacity {
                NumberAnimation {
                    duration: 200
                    easing.type: Easing.OutCubic
                }
            }
            
            Column {
                id: powerMenuContent
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.top: parent.top
                anchors.margins: 10
                spacing: 8
                
                // Header
                Text {
                    text: "Power Profile"
                    font.pixelSize: 16
                    font.weight: Font.Bold
                    color: (sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff"
                }
                
                // Profile Options
                Column {
                    width: parent.width
                    spacing: 6
                    
                    Repeater {
                        model: ["performance", "balanced", "power-saver"]
                        
                        Rectangle {
                            width: parent.width
                            height: 32
                            color: (powerIconContainer.currentProfile === modelData) ?
                                ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff") :
                                (profileMouseArea.containsMouse ?
                                    ((sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#1a1a1a") :
                                    "transparent")
                            radius: 4
                            
                            Text {
                                text: modelData.charAt(0).toUpperCase() + modelData.slice(1).replace("-", " ")
                                font.pixelSize: 12
                                color: (powerIconContainer.currentProfile === modelData) ? "#ffffff" :
                                    ((sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff")
                                anchors.left: parent.left
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.leftMargin: 8
                            }
                            
                            MouseArea {
                                id: profileMouseArea
                                anchors.fill: parent
                                cursorShape: Qt.PointingHandCursor
                                hoverEnabled: true
                                onClicked: setPowerProfile(modelData)
                            }
                        }
                    }
                }
            }
            
            // Close menu when clicking outside
            MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton
                onClicked: {
                    mouse.accepted = false
                }
            }
        }
        }
    }
    
    // Screenshot Button - OUTSIDE sidePanelRect and sidePanelContent to ensure it's clickable
    Item {
        id: screenshotButtonContainer
        width: 32
        height: 32

        anchors.horizontalCenter: panelPosition === "left" ? parent.horizontalCenter : undefined
        anchors.right: panelPosition === "top" ? parent.right : undefined
        anchors.rightMargin: panelPosition === "top" ? 48 : 0
        anchors.bottom: panelPosition === "left" ? parent.bottom : undefined
        anchors.bottomMargin: panelPosition === "left" ? 10 : 0
        z: 100000  // Very high z to ensure it's on top of everything (increased from 10000)
        visible: true
        enabled: true
        
        // Debug: Make sure button is visible and clickable
        Component.onCompleted: {
            console.log("Screenshot button container created at z:", z, "visible:", visible, "enabled:", enabled)
        }

        // Smooth repositioning when panel position changes
        Behavior on anchors.rightMargin {
            NumberAnimation {
                duration: 400
                easing.type: Easing.OutCubic
            }
        }
        Behavior on anchors.bottomMargin {
            NumberAnimation {
                duration: 400
                easing.type: Easing.OutCubic
            }
        }
        
        // Material Design button with elevation
        Rectangle {
            id: screenshotButton
            width: 24
            height: 24
            anchors.centerIn: parent
            radius: 0
            // Material Design button color
            color: screenshotButtonMouseArea.containsMouse ?
                ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff") :
                ((sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#1a1a1a")
            
            property real buttonScale: screenshotButtonMouseArea.pressed ? 0.95 : (screenshotButtonMouseArea.containsMouse ? 1.05 : 1.0)
            property real buttonElevation: screenshotButtonMouseArea.pressed ? 1 : (screenshotButtonMouseArea.containsMouse ? 3 : 2)
            
            // Material Design elevation shadow
            Rectangle {
                anchors.fill: parent
                anchors.margins: -buttonElevation
                color: "transparent"
                border.color: Qt.rgba(0, 0, 0, 0.15 + buttonElevation * 0.05)
                border.width: buttonElevation
                z: -1
                
                Behavior on border.color {
                    ColorAnimation {
                        duration: 200
                        easing.type: Easing.OutQuart
                    }
                }
            }
            
            Behavior on color {
                ColorAnimation {
                    duration: 200
                    easing.type: Easing.OutQuart
                }
            }
            
            Behavior on buttonScale {
                NumberAnimation {
                    duration: 150
                    easing.type: Easing.OutQuart
                }
            }
            
            scale: buttonScale
            
            Text {
                text: "󰹑"  // Camera/screenshot icon (Nerd Fonts)
                font.pixelSize: 14
                anchors.centerIn: parent
                color: screenshotButtonMouseArea.containsMouse ? 
                    ((sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff") : 
                    ((sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff")
                
                Behavior on color {
                    ColorAnimation {
                        duration: 200
                        easing.type: Easing.OutQuart
                    }
                }
            }
        }
        
        MouseArea {
            id: screenshotButtonMouseArea
            anchors.fill: parent
            anchors.margins: -10  // Much larger hit area
            cursorShape: Qt.PointingHandCursor
            hoverEnabled: true
            enabled: true
            propagateComposedEvents: false
            acceptedButtons: Qt.LeftButton
            z: 10001
            
            onClicked: {
                console.log("=== SCREENSHOT BUTTON CLICKED ===")
                console.log("Panel position:", panelPosition)
                console.log("Container size:", screenshotButtonContainer.width, "x", screenshotButtonContainer.height)
                console.log("Container position:", screenshotButtonContainer.x, ",", screenshotButtonContainer.y)
                if (screenshotFunction) {
                    screenshotFunction()
                } else {
                    console.log("screenshotFunction is null!")
                }
            }

            onPressed: {
                console.log("Screenshot button pressed")
            }

            onEntered: {
                console.log("Mouse entered screenshot button")
            }
        }
    }
    
    // Opcjonalne funkcje callback
    property var launcherFunction
    property var screenshotFunction
    
    // System Icons Properties
    property bool bluetoothEnabled: false
    property bool bluetoothScanning: false
    property bool bluetoothConnecting: false
    
    // Menu visibility properties
    property bool bluetoothMenuVisible: false
    property bool networkMenuVisible: false
    property bool powerMenuVisible: false
    
    // Bluetooth devices list
    ListModel {
        id: bluetoothDevicesModel
    }
    
    // --- Music Visualizer ---
    property var cavaValues: []
    property bool cavaRunning: false
    
    function startCava() {
        // Sprawdź czy cava jest zainstalowane
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh','-c','which cava > /dev/null 2>&1 && echo 1 > /tmp/quickshell_cava_available || echo 0 > /tmp/quickshell_cava_available']; running: true }", sidePanel)
        
        // Poczekaj i sprawdź dostępność
        Qt.createQmlObject("import QtQuick; Timer { interval: 200; running: true; repeat: false; onTriggered: sidePanel.checkCavaAvailable() }", sidePanel)
    }
    
    function checkCavaAvailable() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_cava_available")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                var available = xhr.responseText.trim() === "1"
                console.log("Cava available:", available, "cavaRunning:", cavaRunning)
                if (available && !cavaRunning) {
                    // Użyj skryptu start-cava.sh do uruchomienia cava z poprawną konfiguracją
                    // Use projectPath if available, otherwise try to detect
                    var scriptPath = (projectPath && projectPath.length > 0) ? (projectPath + "/scripts/start-cava.sh") : ""
                    if (!scriptPath || scriptPath === "/scripts/start-cava.sh") {
                        // Try to get from environment or use relative path
                        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'echo \"$QUICKSHELL_PROJECT_PATH\" > /tmp/quickshell_cava_path 2>/dev/null || echo \"\" > /tmp/quickshell_cava_path']; running: true }", sidePanel)
                        Qt.createQmlObject("import QtQuick; Timer { interval: 100; running: true; repeat: false; onTriggered: sidePanel.readCavaPath() }", sidePanel)
                        return
                    }
                    if (!scriptPath || scriptPath.length === 0 || scriptPath === "/scripts/start-cava.sh") {
                        console.log("Invalid script path for cava:", scriptPath)
                        return
                    }
                    var absScriptPath = scriptPath
                    Qt.createQmlObject('import Quickshell.Io; import QtQuick; Process { command: ["bash", "' + absScriptPath + '"]; running: true }', sidePanel)
                    
                    cavaRunning = true
                    console.log("Cava started with script...")
                    Qt.createQmlObject("import QtQuick; Timer { interval: 500; running: true; repeat: false; onTriggered: sidePanel.readCavaData() }", sidePanel)
                }
            }
        }
        xhr.send()
    }
    
    function readCavaData() {
        // Bezpośredni odczyt z pliku (awk nadpisuje go dla każdej klatki)
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_cava")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                if (xhr.status !== 200 && xhr.status !== 0) {
                    // File not accessible, try to restart cava
                    if (cavaRunning) {
                        console.log("Cava file not accessible, status:", xhr.status)
                        cavaRunning = false
                        startCava()
                    }
                    return
                }
                var data = xhr.responseText
                if (data && data.length > 0) {
                    // Remove any trailing semicolons and split
                    var cleanData = data.trim().replace(/;+$/, '')
                    var values = cleanData.split(";")
                    
                    // Ensure we have at least some values
                    if (values.length > 0) {
                        // Use sharedData colors if available - wszystkie odcienie z theme
                        var colorAccent = (sharedData && sharedData.colorAccent) ? sharedData.colorAccent : "#4a9eff"
                        var colorText = (sharedData && sharedData.colorText) ? sharedData.colorText : "#ffffff"
                        var colorPrimary = (sharedData && sharedData.colorPrimary) ? sharedData.colorPrimary : "#3a3a3a"
                        var colorSecondary = (sharedData && sharedData.colorSecondary) ? sharedData.colorSecondary : "#2a2a2a"
                        
                        for (var i = 0; i < 36; i++) {
                            var val = 0
                            if (i < values.length && values[i]) {
                                val = parseInt(values[i]) || 0
                            }
                            var normalizedWidth = Math.max(3, (val / 100) * 24)
                            var normalizedHeight = Math.max(3, (val / 100) * 24)
                            var intensity = val / 100
                            
                            // Update vertical visualizer (for left position)
                            if (visualizerBarsRepeater.itemAt(i)) {
                                visualizerBarsRepeater.itemAt(i).visualizerBarValue = normalizedWidth
                                if (intensity > 0.7) {
                                    // Najwyższe wartości - accent color (najjaśniejszy, kolorowy)
                                    visualizerBarsRepeater.itemAt(i).color = colorAccent
                                } else if (intensity > 0.4) {
                                    // Średnie wartości - text color (jasny)
                                    visualizerBarsRepeater.itemAt(i).color = colorText
                                } else if (intensity > 0.1) {
                                    // Niskie wartości - primary color (średni)
                                    visualizerBarsRepeater.itemAt(i).color = colorPrimary
                                } else {
                                    // Bardzo niskie wartości - secondary color (ciemniejszy)
                                    visualizerBarsRepeater.itemAt(i).color = colorSecondary
                                }
                            }
                            
                            // Update horizontal visualizer (for top position)
                            if (visualizerBarsRepeaterTop.itemAt(i)) {
                                visualizerBarsRepeaterTop.itemAt(i).visualizerBarValueTop = normalizedHeight
                                if (intensity > 0.7) {
                                    visualizerBarsRepeaterTop.itemAt(i).color = colorAccent
                                } else if (intensity > 0.4) {
                                    visualizerBarsRepeaterTop.itemAt(i).color = colorText
                                } else if (intensity > 0.1) {
                                    visualizerBarsRepeaterTop.itemAt(i).color = colorPrimary
                                } else {
                                    visualizerBarsRepeaterTop.itemAt(i).color = colorSecondary
                                }
                            }
                        }
                    }
                } else {
                    // No data, silently continue - cavaCheckTimer will handle restart if needed
                    // Removed frequent logging to reduce console spam
                }
            }
        }
        xhr.send()
    }
    
    // Timer do odczytu danych z cava
    Timer {
        id: cavaDataTimer
        interval: 16  // ~60 FPS
        repeat: true
        running: cavaRunning
        onTriggered: readCavaData()
    }
    
    // Timer do sprawdzania czy cava działa (fallback)
    Timer {
        id: cavaCheckTimer
        interval: 5000  // Co 5 sekund
        repeat: true
        running: true
        onTriggered: {
            if (cavaRunning) {
                // Sprawdź czy plik istnieje i ma dane
                var xhr = new XMLHttpRequest()
                xhr.open("GET", "file:///tmp/quickshell_cava")
                xhr.onreadystatechange = function() {
                    if (xhr.readyState === XMLHttpRequest.DONE) {
                        if (xhr.status !== 200 && xhr.status !== 0) {
                            console.log("Cava file not accessible, restarting...")
                            cavaRunning = false
                            startCava()
                        }
                    }
                }
                xhr.send()
            } else {
                // Spróbuj ponownie uruchomić cava
                startCava()
            }
        }
    }
    
    // Timer do inicjalizacji visualizera
    Timer {
        id: visualizerInitTimer
        interval: 100
        running: false
        repeat: false
        onTriggered: {
            // Ustaw minimalne wartości dla pasków, żeby były widoczne od razu
            for (var i = 0; i < 36; i++) {
                var value = 5 + (i % 3) * 3  // Różne wartości dla testu
                if (visualizerBarsRepeater.itemAt(i)) {
                    visualizerBarsRepeater.itemAt(i).visualizerBarValue = value
                }
                if (visualizerBarsRepeaterTop.itemAt(i)) {
                    visualizerBarsRepeaterTop.itemAt(i).visualizerBarValueTop = value
                }
            }
        }
    }
    
    // Load project path from environment
    function loadProjectPath() {
        // Try to read path from environment variable
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'echo \"$QUICKSHELL_PROJECT_PATH\" > /tmp/quickshell_sidepanel_path 2>/dev/null || echo \"\" > /tmp/quickshell_sidepanel_path']; running: true }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 100; running: true; repeat: false; onTriggered: sidePanel.readProjectPath() }", sidePanel)
    }
    
    function readProjectPath() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_sidepanel_path")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                var path = xhr.responseText.trim()
                if (path && path.length > 0) {
                    projectPath = path
                    console.log("SidePanel project path loaded:", projectPath)
                    // Start cava after path is loaded
                    startCava()
                } else {
                    // Fallback to default
                    projectPath = "/tmp/sharpshell"
                    console.log("SidePanel using fallback project path:", projectPath)
                    startCava()
                }
            }
        }
        xhr.send()
    }
    
    function readCavaPath() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_cava_path")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                var path = xhr.responseText.trim()
                if (path && path.length > 0) {
                    projectPath = path
                    console.log("SidePanel project path loaded from cava path:", projectPath)
                    // Retry starting cava
                    startCava()
                } else {
                    // Fallback
                    projectPath = "/tmp/sharpshell"
                    console.log("SidePanel using fallback project path (from readCavaPath):", projectPath)
                    startCava()
                }
            }
        }
        xhr.send()
    }
    
    // --- Bluetooth Functions ---
    function checkBluetoothStatus() {
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', '/usr/bin/bluetoothctl show | grep -q \\\"Powered: yes\\\" && echo 1 > /tmp/quickshell_bt_status || echo 0 > /tmp/quickshell_bt_status']; running: true }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 300; running: true; repeat: false; onTriggered: sidePanel.readBluetoothStatus() }", sidePanel)
    }
    
    function readBluetoothStatus() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_bt_status")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                bluetoothEnabled = xhr.responseText.trim() === "1"
                if (bluetoothEnabled) {
                    scanBluetoothDevices()
                } else {
                    bluetoothDevicesModel.clear()
                }
            }
        }
        xhr.send()
    }
    
    function toggleBluetooth() {
        if (bluetoothEnabled) {
            Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'rfkill block bluetooth; /usr/bin/bluetoothctl power off']; running: true }", sidePanel)
        } else {
            Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'rfkill unblock bluetooth; sleep 1; /usr/bin/bluetoothctl power on']; running: true }", sidePanel)
        }
        Qt.createQmlObject("import QtQuick; Timer { interval: 1500; running: true; repeat: false; onTriggered: sidePanel.checkBluetoothStatus() }", sidePanel)
    }
    
    function scanBluetoothDevices() {
        if (!bluetoothEnabled || bluetoothScanning) return
        bluetoothScanning = true
        bluetoothDevicesModel.clear()
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'bluetoothctl --timeout 10 scan on > /tmp/quickshell_bt_scan_output 2>&1']; running: true }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 12000; running: true; repeat: false; onTriggered: sidePanel.getBluetoothDevices() }", sidePanel)
    }
    
    function getBluetoothDevices() {
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'bluetoothctl devices > /tmp/quickshell_bt_devices']; running: true }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 500; running: true; repeat: false; onTriggered: sidePanel.readBluetoothDevices() }", sidePanel)
    }
    
    function readBluetoothDevices() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_bt_devices")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                bluetoothDevicesModel.clear()
                var content = xhr.responseText || ""
                var lines = content.trim().split("\n")
                for (var i = 0; i < lines.length; i++) {
                    var line = lines[i].trim()
                    if (line.length > 0 && line.startsWith("Device")) {
                        var parts = line.split(" ")
                        if (parts.length >= 3) {
                            var mac = parts[1]
                            var name = parts.slice(2).join(" ") || "Unknown Device"
                            bluetoothDevicesModel.append({ mac: mac, name: name, connected: false })
                        }
                    }
                }
                bluetoothScanning = false
            }
        }
        xhr.send()
    }
    
    function connectBluetoothDevice(mac) {
        if (bluetoothConnecting) return
        bluetoothConnecting = true
        var macStr = String(mac).trim()
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['/usr/bin/bluetoothctl', 'pair', '" + macStr + "']; running: true }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 3000; running: true; repeat: false; onTriggered: sidePanel.connectAfterPair('" + macStr + "') }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 8000; running: true; repeat: false; onTriggered: { sidePanel.bluetoothConnecting = false; sidePanel.getBluetoothDevices() } }", sidePanel)
    }
    
    function connectAfterPair(mac) {
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['/usr/bin/bluetoothctl', 'connect', '" + mac + "']; running: true }", sidePanel)
    }
    
    function disconnectBluetoothDevice(mac) {
        var escapedMac = mac.replace(/'/g, "\\'")
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'bluetoothctl disconnect \\\"" + escapedMac + "\\\"']; running: true }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 1000; running: true; repeat: false; onTriggered: sidePanel.getBluetoothDevices() }", sidePanel)
    }
    
    // --- Network Functions ---
    function checkNetworkStatus() {
        // Check Wi-Fi status
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'nmcli radio wifi | grep -q enabled && echo 1 > /tmp/quickshell_wifi_status || echo 0 > /tmp/quickshell_wifi_status']; running: true }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 300; running: true; repeat: false; onTriggered: sidePanel.readWifiStatus() }", sidePanel)
    }
    
    function readWifiStatus() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_wifi_status")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                networkIconContainer.wifiEnabled = xhr.responseText.trim() === "1"
                if (networkIconContainer.wifiEnabled) {
                    getCurrentWifi()
                } else {
                    networkIconContainer.currentWifi = "Not connected"
                }
            }
        }
        xhr.send()
    }
    
    function getCurrentWifi() {
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'nmcli -t -f active,ssid dev wifi | grep yes: | cut -d: -f2 > /tmp/quickshell_wifi_current || echo \"Not connected\" > /tmp/quickshell_wifi_current']; running: true }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 300; running: true; repeat: false; onTriggered: sidePanel.readCurrentWifi() }", sidePanel)
    }
    
    function readCurrentWifi() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_wifi_current")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                var wifi = xhr.responseText.trim()
                networkIconContainer.currentWifi = wifi || "Not connected"
            }
        }
        xhr.send()
    }
    
    function toggleWifi() {
        if (networkIconContainer.wifiEnabled) {
            Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'nmcli radio wifi off']; running: true }", sidePanel)
        } else {
            Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'nmcli radio wifi on']; running: true }", sidePanel)
        }
        Qt.createQmlObject("import QtQuick; Timer { interval: 1000; running: true; repeat: false; onTriggered: sidePanel.checkNetworkStatus() }", sidePanel)
    }
    
    // --- Power Profile Functions ---
    function checkPowerProfile() {
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'powerprofilesctl get 2>/dev/null | head -n1 > /tmp/quickshell_power_profile || echo \"balanced\" > /tmp/quickshell_power_profile']; running: true }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 300; running: true; repeat: false; onTriggered: sidePanel.readPowerProfile() }", sidePanel)
    }
    
    function readPowerProfile() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_power_profile")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                var profile = xhr.responseText.trim().toLowerCase()
                if (profile === "performance" || profile === "balanced" || profile === "power-saver") {
                    powerIconContainer.currentProfile = profile
                } else {
                    powerIconContainer.currentProfile = "balanced"
                }
            }
        }
        xhr.send()
    }
    
    function setPowerProfile(profile) {
        var profileMap = {
            "performance": "performance",
            "balanced": "balanced",
            "power-saver": "power-saver"
        }
        var profileCmd = profileMap[profile] || "balanced"
        Qt.createQmlObject("import Quickshell.Io; import QtQuick; Process { command: ['sh', '-c', 'powerprofilesctl set " + profileCmd + " 2>/dev/null || true']; running: true }", sidePanel)
        Qt.createQmlObject("import QtQuick; Timer { interval: 500; running: true; repeat: false; onTriggered: sidePanel.checkPowerProfile() }", sidePanel)
    }
    
    Component.onCompleted: {
        // Uruchom inicjalizację visualizera
        visualizerInitTimer.start()
        // Load project path first, then start cava
        loadProjectPath()
        // Initialize system icons
        checkBluetoothStatus()
        checkNetworkStatus()
        checkPowerProfile()
    }
}


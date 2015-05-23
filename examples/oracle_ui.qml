import QtQuick 2.2
import QtQuick.Controls 1.2
import QtQuick.Layouts 1.0
import QtQuick.XmlListModel 2.0

ApplicationWindow {
  visible: true
  title: "NaiveLieDetector"

  property int margin: 11
  width: mainLayout.implicitWidth + 2 * margin
  height: mainLayout.implicitHeight + 2 * margin
  minimumWidth: mainLayout.Layout.minimumWidth + 2 * margin
  minimumHeight: mainLayout.Layout.minimumHeight + 2 * margin

  ColumnLayout {
    id: mainLayout
    anchors.fill: parent
    anchors.margins: margin

    RowLayout {
      TextField {
        id: numberField
        Layout.fillWidth: true

        placeholderText: "Enter a statement"
        focus: true

        onAccepted: doCalculate()
      }

      Button {
        text: "Calculate"

        onClicked: doCalculate()
      }
    }

    TextArea {
      id: resultArea
      Layout.fillWidth: true
      Layout.fillHeight: true
    }
  }

  function doCalculate() {
    //var num = parseInt(numberField.text);
    resultArea.text = oracle.calculate( (Math.random() < 0.5 ? true : false) );
  }

/*
  Connections {
    target: oracle
    onTest: console.log("Got test signal!")
  }
*/
}

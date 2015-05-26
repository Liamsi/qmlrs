import QtQuick 2.2
import QtQuick.Controls 1.2
import QtQuick.Layouts 1.0

ApplicationWindow {
  visible: true
  title: "SelfEncrypt"

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

        placeholderText: "Enter absolute filepath"
        focus: true

        onAccepted: encrypt()
      }

      Button {
        text: "encrypt"

        onClicked: encrypt()
      }
    }

    TextArea {
      id: resultArea
      Layout.fillWidth: true
      Layout.fillHeight: true
    }
  }

  function encrypt() {
    var filepath = numberField.text;
    resultArea.text = SelfEncrypt.encrypt(filepath);
  }

}

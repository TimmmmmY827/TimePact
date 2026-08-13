!macro NSIS_HOOK_PREINSTALL
  ; TimePact keeps running in the tray after its window is closed. Stop the
  ; installed process before replacing the executable during an upgrade.
  nsExec::ExecToStack '"$SYSDIR\taskkill.exe" /F /IM "TimePact.exe"'
  Pop $0
  Pop $1

  ; A current-user install cannot write to Program Files without elevation.
  ; Repair locations restored from the earlier broken installers.
  ${If} $INSTDIR == "$PROGRAMFILES\${PRODUCTNAME}"
  ${OrIf} $INSTDIR == "$PROGRAMFILES32\${PRODUCTNAME}"
  ${OrIf} $INSTDIR == "$PROGRAMFILES64\${PRODUCTNAME}"
  ${OrIf} $INSTDIR == "$PROGRAMFILES\时契"
  ${OrIf} $INSTDIR == "$PROGRAMFILES32\时契"
  ${OrIf} $INSTDIR == "$PROGRAMFILES64\时契"
    StrCpy $INSTDIR "$LOCALAPPDATA\${PRODUCTNAME}"
    SetOutPath $INSTDIR
  ${EndIf}
!macroend

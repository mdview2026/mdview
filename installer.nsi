; mdview NSIS installer script
; Usage: makensis /DPRODUCT_VERSION=0.1.0 installer.nsi

!ifndef PRODUCT_VERSION
!define PRODUCT_VERSION "0.1.0"
!endif

; Numeric X.X.X.X version for the version-info resource (NSIS rejects pre-release
; suffixes). Falls back to PRODUCT_VERSION padded with a trailing .0 when not passed.
!ifndef VI_VERSION
!define VI_VERSION "${PRODUCT_VERSION}.0"
!endif

!define PRODUCT_NAME "mdview"
!define PRODUCT_PUBLISHER "mdview"

!ifndef OUT_DIR
!define OUT_DIR "."
!endif

!ifndef SETUP_SLUG
!define SETUP_SLUG ""
!endif
!ifndef PRODUCT_EDITION
!define PRODUCT_EDITION ""
!endif
!ifndef SOURCE_EXE
!define SOURCE_EXE "dist\mdview.exe"
!endif

OutFile "${OUT_DIR}\${PRODUCT_NAME}-setup${SETUP_SLUG}-${PRODUCT_VERSION}.exe"

InstallDir "$PROGRAMFILES\${PRODUCT_NAME}"
InstallDirRegKey HKLM "Software\${PRODUCT_NAME}" "InstallDir"

Icon "static\icon.ico"

RequestExecutionLevel admin

!include "MUI2.nsh"

!define MUI_ICON "static\icon.ico"
!define MUI_UNICON "static\icon.ico"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

VIProductVersion "${VI_VERSION}"
VIAddVersionKey "ProductName" "${PRODUCT_NAME}${PRODUCT_EDITION}"
VIAddVersionKey "ProductVersion" "${PRODUCT_VERSION}"
VIAddVersionKey "CompanyName" "${PRODUCT_PUBLISHER}"
VIAddVersionKey "FileDescription" "${PRODUCT_NAME}${PRODUCT_EDITION} Setup"
VIAddVersionKey "FileVersion" "${PRODUCT_VERSION}"

Section "Main Program" SecMain
    SectionIn RO
    SetOutPath $INSTDIR
    File "dist\mdview.exe"

    WriteRegStr HKLM "Software\${PRODUCT_NAME}" "InstallDir" "$INSTDIR"
    WriteRegStr HKLM "Software\${PRODUCT_NAME}" "Version" "${PRODUCT_VERSION}"

    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "DisplayName" "${PRODUCT_NAME}${PRODUCT_EDITION}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "UninstallString" '"$INSTDIR\uninstall.exe"'
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "DisplayIcon" "$INSTDIR\mdview.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "DisplayVersion" "${PRODUCT_VERSION}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "Publisher" "${PRODUCT_PUBLISHER}"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "NoModify" 1
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "NoRepair" 1

    WriteUninstaller "$INSTDIR\uninstall.exe"

    ExecWait '"$INSTDIR\mdview.exe" --install'
SectionEnd

Section "Start Menu Shortcuts" SecStartMenu
    CreateDirectory "$SMPROGRAMS\${PRODUCT_NAME}"
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\${PRODUCT_NAME}.lnk" "$INSTDIR\mdview.exe" "" "$INSTDIR\mdview.exe" 0
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\Settings.lnk" "$INSTDIR\mdview.exe" "--settings" "$INSTDIR\mdview.exe" 0
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\Uninstall.lnk" "$INSTDIR\uninstall.exe" "" "$INSTDIR\uninstall.exe" 0
SectionEnd

Section "Desktop Shortcut" SecDesktop
    CreateShortCut "$DESKTOP\${PRODUCT_NAME}.lnk" "$INSTDIR\mdview.exe" "" "$INSTDIR\mdview.exe" 0
SectionEnd

Section "Uninstall"
    ExecWait '"$INSTDIR\mdview.exe" --uninstall'

    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
    DeleteRegKey HKLM "Software\${PRODUCT_NAME}"

    Delete "$INSTDIR\mdview.exe"
    Delete "$INSTDIR\uninstall.exe"

    Delete "$SMPROGRAMS\${PRODUCT_NAME}\*.*"
    RMDir "$SMPROGRAMS\${PRODUCT_NAME}"
    Delete "$DESKTOP\${PRODUCT_NAME}.lnk"

    RMDir "$INSTDIR"
SectionEnd

Function .onInstSuccess
    ; Unconditionally run the program after a successful install (also applies to silent installs)
    Exec '"$INSTDIR\mdview.exe"'
FunctionEnd

Function un.onInit
    MessageBox MB_ICONQUESTION|MB_YESNO|MB_DEFBUTTON2 "Are you sure you want to uninstall ${PRODUCT_NAME}?" IDYES +2
    Abort
FunctionEnd

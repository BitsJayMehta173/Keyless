!define APPNAME "Keyless"
!define VERSION "1.0.0"
!define INSTALLDIR "$PROGRAMFILES\${APPNAME}"

Outfile "Keyless-Installer.exe"
InstallDir "${INSTALLDIR}"
RequestExecutionLevel admin

Page directory
Page instfiles
UninstPage uninstConfirm
UninstPage instfiles

Section "Install"
    SetOutPath "$INSTDIR"

    ; Copy application
    File "..\target\release\keyless.exe"
    File "..\icon.ico"

    ; Shortcuts
    CreateDirectory "$SMPROGRAMS\${APPNAME}"
    CreateShortCut "$SMPROGRAMS\${APPNAME}\Keyless.lnk" "$INSTDIR\keyless.exe"

    ; Desktop shortcut (optional)
    CreateShortCut "$DESKTOP\Keyless.lnk" "$INSTDIR\keyless.exe"

    ; Uninstaller
    WriteUninstaller "$INSTDIR\uninstall.exe"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\keyless.exe"
    Delete "$INSTDIR\icon.ico"

    Delete "$SMPROGRAMS\${APPNAME}\Keyless.lnk"
    Delete "$DESKTOP\Keyless.lnk"

    Delete "$INSTDIR\uninstall.exe"
    RMDir "$INSTDIR"
SectionEnd

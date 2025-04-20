@echo off
chcp 65001 > nul

for /f "usebackq delims=" %%i in (`rcd %* 2^>^&1`) do (
    if "%%i" NEQ "" (
        cd /d "%%i"
    ) else (
        exit /b 1
    )
)

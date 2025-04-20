@echo off
chcp 65001 > nul
::echo 传递的参数: %*
::echo 正在执行命令: rcd %*

for /f "usebackq delims=" %%i in (`rcd %* 2^>^&1`) do (
    ::echo 接收到的值: %%i
    if "%%i" NEQ "" (
        cd /d "%%i"
        ::echo 成功切换到目录: %%i
    ) else (
        echo Error: 未找到匹配目录（参数：%*）
        exit /b 1
    )
)

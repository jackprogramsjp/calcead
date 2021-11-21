set "targets=aarch64-apple-darwin i686-pc-windows-msvc i686-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-msvc x86_64-unknown-linux-gnu"

if "%1" == "install" (
    where /q rustup
    if %errorlevel% neq 0 (
        echo rustup doesn't exist in the system, please install rust
        exit /b %errorlevel%
    )
    for %%t in (%targets%) do (
        rustup target add %%t
    )
) else if "%1" == "remove" (
    where /q rustup
    if %errorlevel% neq 0 (
        echo rustup doesn't exist in the system, please install rust
        exit /b %errorlevel%
    )
    for %%t in (%targets%) do (
        rustup target remove %%t
    )
) else (
    where /q cargo
    if %errorlevel% neq 0 (
        echo cargo doesn't exist in the system, please install rust
        exit /b %errorlevel%
    )
    for %%t in (%targets%) do (
        cargo build --target %%t
    )
)

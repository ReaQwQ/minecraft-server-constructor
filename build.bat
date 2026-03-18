@echo off
setlocal
chcp 65001 > nul

echo 🚀 MSB Dual-Platform Build System 起動...

echo.
echo [1/2] Windows (x64) バイナリをビルド中...
cargo build --release
if %errorlevel% neq 0 (
    echo ❌ Windows ビルドに失敗しました。
    exit /b %errorlevel%
)

echo.
echo [2/2] Linux (x64 musl) バイナリをビルド中...
echo ※ ターゲット x86_64-unknown-linux-musl を使用します
cargo build --release --target x86_64-unknown-linux-musl
if %errorlevel% neq 0 (
    echo ⚠️ Linux ビルドには 'rustup target add x86_64-unknown-linux-musl' とリンカの設定が必要です。
    echo ⚠️ 今回はスキップ、または失敗しました。
)

echo.
echo ✨ ビルド完了！
echo 出力先:
echo   - Windows: target/release/msb.exe
echo   - Linux:   target/x86_64-unknown-linux-musl/release/msb
echo.
pause

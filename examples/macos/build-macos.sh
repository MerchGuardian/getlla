# Package as app
cargo b
mkdir -p getlla.app/Contents/MacOS/
cp ../../target/debug/getlla getlla.app/Contents/MacOS/
cp Info.plist getlla.app/Contents
codesign --force --deep --sign - getlla.app


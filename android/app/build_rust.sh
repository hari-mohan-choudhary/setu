cd .. && cd .. && pwd && cd src
export PATH="$HOME/.cargo/bin:$PATH"
cargo ndk -t armeabi-v7a -t arm64-v8a -o ../android/app/src/main/jniLibs build --release
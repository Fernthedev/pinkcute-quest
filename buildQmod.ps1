# Builds a .qmod file
cargo ndk build --release

if ($?) {
    Compress-Archive -Path "./mod.json", "./target/aarch64-linux-android/release/libpinkcute.so" -DestinationPath "./pink_cute_v0.1.1.zip" -Update
    Remove-Item "./pink_cute_v0.1.1.qmod"
    Rename-Item "./pink_cute_v0.1.1.zip" "./pink_cute_v0.1.1.qmod"
}

pub fn get() {
    const TEXT: &str = r#"
Version: 4.6.2 - stable
[Windows 7+] -> https://sos-ch-dk-2.exo.io/utopia/geph-releases/windows-stable/4.6.2/geph-windows-setup.exe
[macOS 10.10+] -> https://sos-ch-dk-2.exo.io/utopia/geph-releases/macos-stable/4.6.2/geph-macos.dmg
[Linux amd64] -> https://sos-ch-dk-2.exo.io/utopia/geph-releases/linux-stable/4.6.2/Geph-x86_64.flatpak
[Android APK] -> https://sos-ch-dk-2.exo.io/utopia/geph-releases/android-stable/4.6.2/geph-android.apk
"#;
    println!("{TEXT}");
}
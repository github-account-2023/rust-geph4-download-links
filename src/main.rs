fn main() {
    const TEXT: &str = r#"
Version: 4.6 - beta
[Windows 7+] -> https://output.circle-artifacts.com/output/job/3c76c1e5-5066-4e7d-992a-b7650a23674a/artifacts/0/geph-windows-setup.exe
[macOS 10.10+] -> https://output.circle-artifacts.com/output/job/c084679d-7cf8-487b-b2b9-f21da16954bd/artifacts/0/geph-macos.dmg
[Linux amd64] -> https://output.circle-artifacts.com/output/job/6b9eef0b-f3ca-4d65-8311-96e804750ac5/artifacts/0/Geph-x86_64.flatpak
[Android APK] -> https://drive.proton.me/urls/V24EFQNTAG#QVLDZMD4609C

Version: 4.4.20
[Windows 7+] -> https://f001.backblazeb2.com/file/geph4-dl/Geph4Releases/4.4.20/geph-windows-4.4.20-setup.exe
[macOS 10.10+] -> https://f001.backblazeb2.com/file/geph4-dl/Geph4Releases/4.4.20/geph-macos-4.4.20.dmg
[Linux amd64] -> https://f001.backblazeb2.com/file/geph4-dl/Geph4Releases/4.4.20/geph-linux64-4.4.20.tar.xz
[Android APK] -> https://f001.backblazeb2.com/file/geph4-dl/Geph4Releases/4.4.20/geph-android-4.4.20.apk
"#;
    println!("{TEXT}");
    std::thread::sleep(std::time::Duration::from_secs(30));
}

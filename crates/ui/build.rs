fn main() {
    if cfg!(target_os = "windows") {
        embed_resource::compile("icon.rc", None::<&str>);
    } else if cfg!(target_os = "macos") {
        // macOS 图标通过 Info.plist 中的 CFBundleIconFile 字段引用
        // 需在打包阶段（如 cargo bundle）处理，build.rs 仅做编译时准备
        println!("cargo:rerun-if-changed=../../assets/icon.ico");
    }
    // Linux: 图标通常通过 .desktop 文件和 hicolor 图标主题处理
    // build.rs 无需额外操作
}

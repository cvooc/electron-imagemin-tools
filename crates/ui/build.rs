fn main() {
    if cfg!(target_os = "windows") {
        embed_resource::compile("icon.rc", None::<&str>);
    }
}

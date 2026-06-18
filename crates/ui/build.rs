fn main() {
    println!("cargo:warning=Build script executing...");

    if cfg!(target_os = "windows") {
        println!("cargo:warning=Windows detected, embedding icon...");
        embed_resource::compile("icon.rc", None::<&str>);
        println!("cargo:warning=Icon embedding complete!");
    }
}

fn main() {
    std::env::set_var("SLINT_STYLE", "material");
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}

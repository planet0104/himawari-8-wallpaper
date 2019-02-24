#[cfg(windows)]
fn main() {
    use std::path::Path;
    let mut res = winres::WindowsResource::new();
    if !Path::new("icon.ico").exists(){
        let img = image::open("icon.png").unwrap();
        img.save("icon.ico").unwrap();
    }
    res.set_icon("icon.ico");
    // res.set("TIP_ICON", "icon.ico");
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {
}
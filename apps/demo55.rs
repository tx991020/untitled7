#[cfg(debug_assertions)]
#[cfg(target_os = "macos")]
embed_plist::embed_info_plist!("../Info.plist");
fn main(){
    println!("{}",111);
}
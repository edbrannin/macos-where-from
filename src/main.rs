use std::{path::Path, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    for argument in &args[1..] {
        if argument == "--help" {
            println!("You passed --help as one of the arguments!");
            return;
        }

        let file_path : &Path = Path::new(argument);
        for url in get_urls(file_path) {
            println!("{}", url)
        }
    }
}

fn get_where_froms(path: &Path) -> Vec<u8> {
    return xattr::get(path, "com.apple.metadata:kMDItemWhereFroms").unwrap().unwrap();
}



fn get_urls(path: &Path) -> Vec<String> {
    let plist_bytes = get_where_froms(path);
    let result_plist = plist::from_bytes::<Vec<String>>(plist_bytes.as_slice()).unwrap();
    return result_plist;
}

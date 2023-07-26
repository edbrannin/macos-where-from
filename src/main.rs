use std::{path::Path, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    for argument in &args[1..] {
        if argument == "--help" {
            println!("You passed --help as one of the arguments!");
            return;
        }

        let file_path : &Path = Path::new(argument);
        let urls = get_urls(file_path);
        if urls.is_none() {
            println!("No WhereFroms found");
            continue;
        }
        for url in urls.unwrap() {
            println!("{}", url)
        }
    }
}

fn get_where_froms(path: &Path) -> Option<Vec<u8>> {
    let result = xattr::get(path, "com.apple.metadata:kMDItemWhereFroms");
    if result.is_err() {
        println!("Failed: {:#?}", result.err());
        return Option::None;
    }

    let result = result.unwrap();
    return result;
}



fn get_urls(path: &Path) -> Option<Vec<String>> {
    let plist_bytes = get_where_froms(path);
    if plist_bytes.is_none() {
        return Option::None;
    }

    let plist_bytes = plist_bytes.unwrap();
    let result_plist = plist::from_bytes::<Vec<String>>(plist_bytes.as_slice()).unwrap();
    return Option::Some(result_plist);
}

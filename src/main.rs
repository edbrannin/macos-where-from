use std::{path::Path, env};
use url::Url;

type FilterFunc = fn(std::string::String) -> Option<std::string::String>;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let index: Option<u32> = Option::None;
    let mut verbose = false;
    let mut filter: FilterFunc = passthru;
    let mut index = Option::None;
    for argument in &args[1..] {
        if argument == "--help" {
            println!("You passed --help as one of the arguments!");
            return;
        }

        if argument == "--verbose" || argument == "-v" {
            verbose = true;
            continue;
        }

        if argument == "--domain" || argument == "-d" {
            filter = as_domain;
            continue;
        }

        if argument == "--last" || argument == "-l" {
            index = Option::Some(-1);
            continue;
        }

        let file_path : &Path = Path::new(argument);
        let urls = get_urls(file_path);
        if urls.is_none() {
            if verbose {
                println!("No WhereFroms found");
            }
            continue;
        }

        for url in iter_or_index(urls.unwrap(), index) {
            let original_url = url.clone();
            let result = filter(url);
            if result.is_some() {
                println!("{}", result.unwrap())
            } else if verbose {
                println!("No result from filtering: {}", original_url);
            }
        }
    }
}

fn iter_or_index<T: Clone>(items: Vec<T>, index_option: Option<i32>) -> Vec<T> {
    if index_option.is_none() {
        return items;
    }

    let index = index_option.unwrap();
    if index == -1 {
        let last = items.last().unwrap().to_owned();
        return vec![last]
    }

    let item = items[index as usize].to_owned();
    return vec![item];
}

fn passthru(str: String) -> Option<String> {
    return Option::Some(str);
}

fn as_domain(str: String) -> Option<String> {
    let url = Url::parse(str.as_str());
    if url.is_ok() {
        let unwrapped = url.unwrap();
        let domain = unwrapped.domain();
        if domain.is_some() {
            return Option::Some(domain.unwrap().to_string());
        }
    }
    return Option::None;
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

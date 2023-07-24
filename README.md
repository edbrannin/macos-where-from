# where-from

A MacOS tool for checking where a file was downloaded from

Example:
```shell
# Go to google.com/robots.txt in Chrome, then save it
Downloads % where-from google.com_robots.txt 
https://www.google.com/robots.txt

Downloads %
```

To do:
- [ ] Better error handling

    ```shell
    Downloads % wget google.com/robots.txt
    Downloads % where-from robots.txt 
    thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:19:12
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace 
    ```

    - [ ] Missing file
    - [ ] File has no matching xattr
    - [ ] Wrong platform
- [ ] Formatting options (and `--help`)
    - [ ] Print filename before its URLs
    - [ ] Repeat filename before each URL (like grep, but with a customizable separator)
    - [ ] Don't print filename (current behavior)
    - [ ] Output as JSON
    - [ ] Only print the Nth URL per file

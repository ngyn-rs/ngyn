{
    let mut current_dir = std::env::current_dir().expect("Current working directory could not be read");
    current_dir.push(path_buf);
    let entries = std::fs::read_dir(current_dir.clone()).expect("Specified static path does not exist");
    let str_current_dir = current_dir.to_str().unwrap();

    let mut assets: std::collections::HashMap<String, &[u8]> = std::collections::HashMap::new();

    for entry in entries {
        let path = entry?.path();

        if path.is_file() {
            let file_path = path
                .to_str()
                .expect("file name contains invalid unicode characters");

            let content = std::fs::read(file_path).unwrap(); // Infallible, would always exist

            // Convert file data into a static slice
            let static_content: &'static [u8] = Box::leak(content.into_boxed_slice());

            assets.insert(file_path.replace(str_current_dir, ""), static_content);
        } else if path.is_dir() {
            self.use_static(path)?;
        }
    }

    assets
}
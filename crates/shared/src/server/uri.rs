use hyper::http::uri::Uri;

pub trait ToParts {
    fn parts(&self, path: &str) -> (bool, Vec<(String, String)>);
}

fn match_paths(path_a: &str, path_b: &str) -> (bool, Vec<(String, String)>) {
    let path_b = format!("^{}$", path_b);
    let path_b = regex::Regex::new(&path_b).unwrap();

    if !path_b.is_match(path_a) {
        return (false, Vec::new());
    }

    let named_matches_with_values: Vec<(String, String)> = path_b
        .captures_iter(path_a)
        .filter_map(|capture| {
            let (name, [path]) = capture.extract();
            return Some((name.to_string(), path.to_string()));
        })
        .collect();

    return (true, named_matches_with_values);
}

impl ToParts for Uri {
    fn parts(&self, raw_path: &str) -> (bool, Vec<(String, String)>) {
        let uri_path = self.path().trim_start_matches('/').trim_end_matches('/');
        let parts_path = raw_path.trim_start_matches('/').trim_end_matches('/');

        let has_wildcard = parts_path.contains('*');
        let has_named = parts_path.contains('<');

        if has_wildcard || has_named {
            let parts_path = parts_path.replace("*", "(.*)");
            let parts_path = parts_path.replace("<", "(?P<").replace(">", ">[^/]+)");
            return match_paths(&parts_path, &uri_path);
        }
        
        match_paths(&parts_path, &uri_path)
    }
}

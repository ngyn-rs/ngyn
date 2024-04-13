use hyper::http::uri::Uri;

pub trait ToParts {
    fn parts(&self, path: &str) -> (bool, Vec<(String, String)>);
}

fn match_paths(path_a: &str, path_b: &str) -> (bool, Vec<(String, String)>) {
    let path_b = format!("^{}$", path_b);
    let path_r = regex::Regex::new(&path_b).unwrap();

    if !path_r.is_match(path_a) {
        return (false, Vec::new());
    }

    let named_matches_with_values: Vec<(String, String)> = path_r
        .captures_iter(path_a)
        .filter_map(|capture| {
            if capture.len() < 2 {
                return None;
            }
            let (_, [value]) = capture.extract();
            let name = path_r.capture_names().nth(1).unwrap().unwrap();
            Some((name.to_string(), value.to_string()))
        })
        .collect();

    (true, named_matches_with_values)
}

impl ToParts for Uri {
    fn parts(&self, raw_path: &str) -> (bool, Vec<(String, String)>) {
        let uri_path = self.path().trim_start_matches('/').trim_end_matches('/');
        let parts_path = raw_path.trim_start_matches('/').trim_end_matches('/');

        let has_wildcard = parts_path.contains('*');
        let has_named = parts_path.contains('<');

        if has_wildcard || has_named {
            let parts_path = parts_path.replace('/', r"\/");
            let parts_path = parts_path.replace('*', "(.*)");
            let parts_path = parts_path.replace('<', "(?P<").replace('>', r">[^\/]+)");

            return match_paths(uri_path, &parts_path);
        }

        match_paths(uri_path, parts_path)
    }
}

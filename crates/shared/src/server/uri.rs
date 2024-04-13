use hyper::http::uri::Uri;

pub trait ToParts {
    fn parts(&self, path: &str) -> (bool, Vec<(String, String)>);
}

fn path_to_regex(path: &str) -> regex::Regex {
    let path_b = format!("^{}$", path);
    let path_b = path_b.replace('/', "\\/");
    let path_b = path_b.replace('*', "(.*)");
    let path_b = path_b.replace('<', "(?P<").replace('>', ">[^/]+)");
    regex::Regex::new(&path_b).unwrap()
}

impl ToParts for Uri {
    fn parts(&self, raw_path: &str) -> (bool, Vec<(String, String)>) {
        let uri_path = self.path().trim_start_matches('/').trim_end_matches('/');
        let parts_path = raw_path.trim_start_matches('/').trim_end_matches('/');

        let path_r = path_to_regex(parts_path);

        if !path_r.is_match(uri_path) {
            return (false, Vec::new());
        }

        let path_b = parts_path.split('/').collect::<Vec<&str>>();
        let path_a = uri_path.split('/').collect::<Vec<&str>>();

        let mut named_matches_with_values = Vec::new();

        path_b.iter().for_each(|part| {
            if part.contains('<') {
                let path_r = path_to_regex(part);
                let name = path_r.capture_names().nth(1).unwrap().unwrap();
                let path_b_pos = path_b.iter().position(|&x| &x == part).unwrap();
                let value = {
                    if path_a.len() > path_b.len() {
                        path_a[path_b_pos..].join("/")
                    } else {
                        path_a[path_b_pos].to_string()
                    }
                };
                named_matches_with_values.push((name.to_string(), value));
            }
        });

        (true, named_matches_with_values)
    }
}

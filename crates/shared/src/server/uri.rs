use hyper::http::uri::Uri;

/// Trait for converting a type to its params.
pub trait ToParams {
    /// Extracts the params of the type based on the provided path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path used for extracting the params.
    ///
    /// # Returns
    ///
    /// A tuple containing a boolean indicating whether the extraction was successful and a vector of key-value pairs representing the extracted params.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyper::http::uri::Uri;
    ///
    /// let uri = Uri::from_static("/users/123");
    /// let raw_path = "/users/<id>";
    ///
    /// let (success, params) = uri.params(raw_path);
    ///
    /// assert_eq!(success, true);
    /// assert_eq!(params, vec![("id".to_string(), "123".to_string())]);
    /// ```
    fn into_params(&self, path: &str) -> Option<Vec<(String, String)>>;
}

/// Converts a path string to a regular expression.
fn path_to_regex(path: &str) -> regex::Regex {
    let path_b = format!("^{}$", path);
    let path_b = path_b.replace('/', "\\/");
    let path_b = path_b.replace('*', "(.*)");
    let path_b = path_b.replace('<', "(?P<").replace('>', ">[^/]+)");
    regex::Regex::new(&path_b).unwrap()
}

impl ToParams for Uri {
    fn into_params(&self, raw_path: &str) -> Option<Vec<(String, String)>> {
        let uri_path = self.path().trim_start_matches('/').trim_end_matches('/');
        let params_path = raw_path.trim_start_matches('/').trim_end_matches('/');

        let path_r = path_to_regex(params_path);

        if !path_r.is_match(uri_path) {
            return None;
        }

        let path_b = params_path.split('/').collect::<Vec<&str>>();
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

        Some(named_matches_with_values)
    }
}

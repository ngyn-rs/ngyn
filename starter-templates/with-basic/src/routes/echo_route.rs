use ngyn::prelude::*;

/// `echo_route` route
///
/// # Description
/// This is the `echo_route` route
///
/// # Arguments
/// * `_cx` - The context of the current request
pub fn echo_route(_cx: &mut NgynContext) -> String {
    "Process echo_route".to_string()
}

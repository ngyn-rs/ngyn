use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::server::{NgynContext, NgynResponse, Transformer};

#[derive(Serialize, Deserialize)]
/// Responses are hard to manage, especially when they are not standardized.
/// This is why Ngyn, by default, provides a json response format.
///
/// The json response format is a JSON object with two keys: `data` and `error`.
/// This would ideally make your responses more predictable and easier to manage.
/// A valid response would look like:
/// ```json
/// {
///    "data": {
///       "key": "value"
///   },
///  "error": null
/// }
/// ```
/// A valid error response would look like:
/// ```json
/// {
///   "data": null,
///  "error": {
///    "status": 404,
///    "message": "Not Found"
///   }
/// }
/// ```
/// The `data` key is used to store the response data, while the `error` key is used to store error data.
/// Both keys are optional, but at least one of them should be present.
///
///
/// ### How to create a json response?
/// Ngyn provides an implementation on [`JsonResult`] to convert it to a json response.
/// This means anytime you make use of a `JsonResult` in your controlled routes, it will be converted to a json response.
///
/// #### Example
/// ```rust ignore
/// use ngyn::prelude::*;
///
/// #[controller]
/// struct MyController;
///
/// #[routes]
/// impl MyController {
///    #[get("/")]
///   async fn get(&self, cx: &mut NgynContext) -> Result<Vec<u8>, ()> {
///    let data = vec![1, 2, 3];
///    Ok(data)
///   }
/// }
/// ```
pub struct JsonResponse<D: Serialize, E: Serialize> {
    data: Option<D>,
    error: Option<E>,
}

impl<D: Serialize, E: Serialize> JsonResponse<D, E> {
    /// Creates a new json response.
    pub fn new(data: Option<D>, error: Option<E>) -> Self {
        Self { data, error }
    }

    /// Returns the data.
    pub fn data(&self) -> Option<&D> {
        self.data.as_ref()
    }

    /// Returns the error data.
    pub fn error(&self) -> Option<&E> {
        self.error.as_ref()
    }
}

pub type JsonResult<V> = Result<V, Value>;

impl<'a> Transformer<'a> for &'a NgynResponse {
    fn transform(_cx: &'a mut NgynContext, res: &'a mut NgynResponse) -> Self {
        res
    }
}

impl<'a> Transformer<'a> for &'a mut NgynResponse {
    fn transform(_cx: &'a mut NgynContext, res: &'a mut NgynResponse) -> Self {
        res
    }
}

use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Serialize)]
struct Info {
    title: String,
    version: String,
    description: String,
}

#[derive(Serialize)]
struct Server {
    url: String,
    description: String,
}

#[derive(Serialize)]
struct Contact {
    name: String,
    url: String,
    email: String,
}

#[derive(Serialize)]
struct License {
    name: String,
    url: String,
}

#[derive(Serialize)]
struct Path {
    #[serde(rename = "get")]
    get: Option<Operation>,
    #[serde(rename = "post")]
    post: Option<Operation>,
}

#[derive(Serialize)]
struct Operation {
    summary: String,
    description: String,
    parameters: Vec<Parameter>,
    request_body: Option<RequestBody>,
    responses: Responses,
}

#[derive(Serialize, Clone)]
struct Parameter {
    name: String,
    #[serde(rename = "in")]
    location: String,
    description: String,
    required: bool,
    schema: Schema,
}

#[derive(Serialize, Clone)]
struct Schema {
    #[serde(rename = "type")]
    type_: String,
    format: Option<String>,
}

#[derive(Serialize)]
struct RequestBody {
    description: String,
    content: Option<Value>,
}

#[derive(Serialize)]
struct MediaType {
    schema: Schema,
}

#[derive(Serialize, Clone)]
struct Responses {
    #[serde(rename = "200")]
    response_200: Response,
    #[serde(rename = "400")]
    response_400: Option<Response>,
}

#[derive(Serialize, Clone)]
struct Response {
    description: String,
    content: Option<Value>,
}

#[derive(Serialize)]
struct OpenAPI {
    openapi: String,
    info: Info,
    servers: Vec<Server>,
    paths: Value,
}

fn generate_openapi_spec() -> String {
    let contact = Contact {
        name: "API Support".to_string(),
        url: "https://www.example.com/support".to_string(),
        email: "support@example.com".to_string(),
    };

    let license = License {
        name: "Apache 2.0".to_string(),
        url: "https://www.apache.org/licenses/LICENSE-2.0.html".to_string(),
    };

    let info = Info {
        title: "Sample API".to_string(),
        version: "1.0.0".to_string(),
        description: "This is a sample API".to_string(),
    };

    let server = Server {
        url: "https://api.example.com".to_string(),
        description: "Example server".to_string(),
    };

    let parameter = Parameter {
        name: "itemId".to_string(),
        location: "path".to_string(),
        description: "ID of the item to retrieve".to_string(),
        required: true,
        schema: Schema {
            type_: "string".to_string(),
            format: None,
        },
    };

    let request_body_schema = Schema {
        type_: "object".to_string(),
        format: None,
    };

    let request_body = RequestBody {
        description: "Item to add".to_string(),
        content: None,
    };

    let response_200 = Response {
        description: "Successful response".to_string(),
        content: None,
    };

    let response_400 = Response {
        description: "Bad request".to_string(),
        content: None,
    };

    let responses = Responses {
        response_200,
        response_400: Some(response_400),
    };

    let get_operation = Operation {
        summary: "Get item".to_string(),
        description: "Retrieves an item".to_string(),
        parameters: vec![parameter.clone()],
        request_body: None,
        responses: responses.clone(),
    };

    let post_operation = Operation {
        summary: "Add item".to_string(),
        description: "Adds a new item".to_string(),
        parameters: vec![],
        request_body: Some(request_body),
        responses,
    };

    let path = Path {
        get: Some(get_operation),
        post: Some(post_operation),
    };

    let mut paths = HashMap::new();
    paths.insert("/items/{itemId}".to_string(), path);

    let openapi = OpenAPI {
        openapi: "3.0.0".to_string(),
        info,
        servers: vec![server],
        paths: json!({}),
    };

    serde_json::to_string_pretty(&openapi).unwrap()
}

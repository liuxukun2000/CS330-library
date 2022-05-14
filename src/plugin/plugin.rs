use dlopen::wrapper::{Container, WrapperApi};
use std::collections::HashMap;
use rocket::request::Request;

struct Plugin {
    name: String,
    version: String,
    config: HashMap<String, String>,
    path: String,
    api: Container<PluginApi>
}

impl Plugin {

}

#[derive(WrapperApi)]
struct PluginApi{
    on_mount: fn() -> bool,
    on_unmount: fn() -> bool,
    on_config: fn(config: &HashMap<String, String>) -> bool,
    call: fn(config: &HashMap<String, String>, request: &Request<'_>) -> String
}
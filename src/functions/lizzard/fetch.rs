use crate::functions::error::reference_error;
use reqwest::{blocking::Client, Method};

use v8::{self, ContextScope, HandleScope};

pub fn init_fetch(scope: &mut ContextScope<HandleScope>, lizzard: v8::Local<v8::Object>) {
    let fetch = v8::FunctionTemplate::new(scope, fetch);

    let fetch_key = v8::String::new(scope, "fetch").unwrap();
    let fetch_obj = fetch.get_function(scope).unwrap();

    lizzard.set(scope, fetch_key.into(), fetch_obj.into());
}

fn fetch(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let promise = v8::PromiseResolver::new(scope).unwrap();

    let url = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let options = args.get(1).to_object(scope).unwrap();

    let method_key = v8::String::new(scope, "method").unwrap().into();
    let body_key = v8::String::new(scope, "body").unwrap().into();

    let method = options
        .get(scope, method_key)
        .unwrap()
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope)
        .to_uppercase();

    let body = options
        .get(scope, body_key)
        .unwrap()
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    if !(method == "GET"
        || method == "HEAD"
        || method == "POST"
        || method == "PUT"
        || method == "DELETE"
        || method == "PATCH")
    {
        reference_error(scope, "method parameter missing");
        return;
    }

    let client = Client::new();

    let response = client
        .request(method.parse::<Method>().unwrap(), &url)
        .body(body)
        .send();

    match response {
        Ok(response) => {
            let result = v8::ObjectTemplate::new(scope);

            let status_code_key = v8::String::new(scope, "statusCode").unwrap().into();
            let status_code_data = v8::String::new(scope, response.status().as_str())
                .unwrap()
                .into();

            let response_key = v8::String::new(scope, "response").unwrap().into();
            let response_data = v8::String::new(scope, &response.text().unwrap())
                .unwrap()
                .into();

            result.set(status_code_key, status_code_data);
            result.set(response_key, response_data);

            let result_obj = result.new_instance(scope).unwrap();
            promise.resolve(scope, result_obj.into());
        }
        Err(e) => {
            let err = v8::String::new(scope, &e.to_string()).unwrap().into();
            promise.reject(scope, err);
        }
    }

    rv.set(promise.get_promise(scope).into());
}

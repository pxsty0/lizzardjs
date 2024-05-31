use chrono::Utc;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};
use v8::{self, ContextScope, HandleScope};

lazy_static! {
    static ref TIMES: Mutex<HashMap<String, i64>> = {
        let map = HashMap::new();
        Mutex::new(map)
    };
}
pub fn init_console(scope: &mut ContextScope<HandleScope>, global: v8::Local<v8::Object>) {
    let console = v8::ObjectTemplate::new(scope);

    let console_log = v8::FunctionTemplate::new(scope, log_callback);
    console.set(
        v8::String::new(scope, "log").unwrap().into(),
        console_log.into(),
    );

    let console_time = v8::FunctionTemplate::new(scope, time_callback);
    console.set(
        v8::String::new(scope, "time").unwrap().into(),
        console_time.into(),
    );

    let console_time_end = v8::FunctionTemplate::new(scope, time_end_callback);
    console.set(
        v8::String::new(scope, "timeEnd").unwrap().into(),
        console_time_end.into(),
    );

    let console_obj = console.new_instance(scope).unwrap();
    let console_key = v8::String::new(scope, "console").unwrap();
    global.set(scope, console_key.into(), console_obj.into());
}

fn log_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let args_length: i32 = args.length();
    for i in 0..args_length {
        let message = args.get(i);
        let message_str = message
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);
        println!("{}", message_str);
    }
}

fn time_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let mut label = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    if args.length() == 0 {
        label = String::from("default");
    }
    let timestamp: i64 = Utc::now().timestamp();

    TIMES.lock().unwrap().insert(label, timestamp);
}

fn time_end_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let mut label = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    if args.length() == 0 {
        label = String::from("default");
    }
    let timestamp = Utc::now().timestamp();

    if let Some((key, value)) = TIMES.lock().unwrap().get_key_value(&label) {
        let result = (timestamp - value) * 1000;
        println!("{} counter has ended : {} ms", key, result);
    } else {
        let err_msg = v8::String::new(scope, "invalid timer label")
            .unwrap()
            .into();
        let exception = v8::Exception::reference_error(scope, err_msg);
        scope.throw_exception(exception);
        return;
    }
}

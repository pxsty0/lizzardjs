use v8::{self, ContextScope, HandleScope};

pub fn init_os(scope: &mut ContextScope<HandleScope>, process: v8::Local<v8::Object>) {
    let process_os = v8::ObjectTemplate::new(scope);

    let process_os_type = v8::FunctionTemplate::new(scope, os_type_callback);
    process_os.set(
        v8::String::new(scope, "type").unwrap().into(),
        process_os_type.into(),
    );

    let process_os_ver = v8::FunctionTemplate::new(scope, os_ver_callback);
    process_os.set(
        v8::String::new(scope, "version").unwrap().into(),
        process_os_ver.into(),
    );

    let process_os_bitness = v8::FunctionTemplate::new(scope, os_bitness_callback);
    process_os.set(
        v8::String::new(scope, "bitness").unwrap().into(),
        process_os_bitness.into(),
    );

    let process_os_key = v8::String::new(scope, "os").unwrap();
    let process_os_obj = process_os.new_instance(scope).unwrap();

    process.set(scope, process_os_key.into(), process_os_obj.into());
}
fn os_type_callback(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let info = os_info::get();
    rv.set(
        v8::String::new(scope, &info.os_type().to_string())
            .unwrap()
            .into(),
    );
}
fn os_ver_callback(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let info = os_info::get();
    rv.set(
        v8::String::new(scope, &info.version().to_string())
            .unwrap()
            .into(),
    );
}

fn os_bitness_callback(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let info = os_info::get();
    rv.set(
        v8::String::new(scope, &info.bitness().to_string())
            .unwrap()
            .into(),
    );
}

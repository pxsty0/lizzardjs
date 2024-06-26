use v8::{self, ContextScope, HandleScope};

mod env;
mod os;

pub fn init_process(scope: &mut ContextScope<HandleScope>, global: v8::Local<v8::Object>) {
    let process = v8::ObjectTemplate::new(scope);

    let process_key = v8::String::new(scope, "process").unwrap();
    let process_obj = process.new_instance(scope).unwrap();

    os::init_os(scope, process_obj);
    env::init_env(scope, process_obj);

    global.set(scope, process_key.into(), process_obj.into());
}

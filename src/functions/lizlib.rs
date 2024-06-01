use v8::{self, ContextScope, HandleScope};

mod fs;

pub fn init_lizlib(scope: &mut ContextScope<HandleScope>, global: v8::Local<v8::Object>) {
    let lizlib = v8::ObjectTemplate::new(scope);

    let lizlib_key = v8::String::new(scope, "lizlib").unwrap();
    let lizlib_obj = lizlib.new_instance(scope).unwrap();

    fs::init_fs(scope, lizlib_obj);

    global.set(scope, lizlib_key.into(), lizlib_obj.into());
}

use v8::{self, ContextScope, HandleScope};

pub fn init_fs(scope: &mut ContextScope<HandleScope>, lizlib: v8::Local<v8::Object>) {
    let fs = v8::ObjectTemplate::new(scope);

    let fs_key = v8::String::new(scope, "fs").unwrap();
    let fs_obj = fs.new_instance(scope).unwrap();

    lizlib.set(scope, fs_key.into(), fs_obj.into());
}

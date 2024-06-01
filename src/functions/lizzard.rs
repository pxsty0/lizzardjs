use v8::{self, ContextScope, HandleScope};

pub fn init_lizzard(scope: &mut ContextScope<HandleScope>, global: v8::Local<v8::Object>) {
    let lizzard = v8::ObjectTemplate::new(scope);

    let lizzard_key = v8::String::new(scope, "lizzard").unwrap();
    let lizzard_obj = lizzard.new_instance(scope).unwrap();

    global.set(scope, lizzard_key.into(), lizzard_obj.into());
}

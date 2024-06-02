use std::{fs, path::Path};

use v8::{self, ContextScope, HandleScope};

use super::error::reference_error;

pub fn init_require(scope: &mut ContextScope<HandleScope>, global: v8::Local<v8::Object>) {
    let require = v8::FunctionTemplate::new(scope, require_callback);

    let require_key = v8::String::new(scope, "require").unwrap();
    let require_obj = require.get_function(scope).unwrap();

    global.set(scope, require_key.into(), require_obj.into());
}

fn require_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let file_path = &args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    if args.length() == 0 || Path::new(file_path).exists() == false {
        reference_error(scope, "invalid require file path");
        return;
    }

    let file_data = fs::read_to_string(file_path).unwrap();
    let v8_string = v8::String::new(scope, &file_data).unwrap().into();

    let file_name = v8::String::new(scope, &file_path).unwrap();
    let undefined = v8::undefined(scope);

    let origin = v8::ScriptOrigin::new(
        scope,
        file_name.into(),
        0,
        0,
        false,
        0,
        undefined.into(),
        false,
        false,
        false,
    );

    let script = v8::Script::compile(scope, v8_string, Some(&origin)).expect(&format!(
        "{} Unidentified Error Occurred (possible syntax error)",
        file_path
    ));
    let result = script.run(scope).unwrap();

    if result.is_function() == false {
        let err_msg = v8::String::new(scope, &format!("{} contains no function", file_path))
            .unwrap()
            .into();
        let exception = v8::Exception::reference_error(scope, err_msg);
        scope.throw_exception(exception);
    }

    rv.set(result);
}

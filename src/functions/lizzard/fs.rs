use std::{fs, path::Path};

use v8::{self, ContextScope, HandleScope};

pub fn init_fs(scope: &mut ContextScope<HandleScope>, lizzard: v8::Local<v8::Object>) {
    let fs = v8::ObjectTemplate::new(scope);

    let fs_exists = v8::FunctionTemplate::new(scope, exists_cb);
    fs.set(
        v8::String::new(scope, "exists").unwrap().into(),
        fs_exists.into(),
    );

    let fs_read = v8::FunctionTemplate::new(scope, read_cb);
    fs.set(
        v8::String::new(scope, "read").unwrap().into(),
        fs_read.into(),
    );

    let fs_write = v8::FunctionTemplate::new(scope, write_cb);
    fs.set(
        v8::String::new(scope, "write").unwrap().into(),
        fs_write.into(),
    );

    let fs_key = v8::String::new(scope, "fs").unwrap();
    let fs_obj = fs.new_instance(scope).unwrap();

    lizzard.set(scope, fs_key.into(), fs_obj.into());
}

fn exists_cb(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() == 0 {
        let err_msg = v8::String::new(scope, "invalid file path").unwrap().into();
        let exception = v8::Exception::reference_error(scope, err_msg);
        scope.throw_exception(exception);
        return;
    }
    let exists = Path::new(
        &args
            .get(0)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope),
    )
    .exists();

    rv.set_bool(exists);
}

fn read_cb(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let file_path = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    if args.length() == 0 || Path::new(&file_path).exists() == false {
        let err_msg = v8::String::new(scope, "invalid file path").unwrap().into();
        let exception = v8::Exception::reference_error(scope, err_msg);
        scope.throw_exception(exception);
        return;
    }
    let reading_data = fs::read_to_string(&file_path).unwrap();

    rv.set(v8::String::new(scope, &reading_data).unwrap().into());
}

fn write_cb(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 2 {
        let err_msg = v8::String::new(scope, "missing parameters").unwrap().into();
        let exception = v8::Exception::reference_error(scope, err_msg);
        scope.throw_exception(exception);
        return;
    }
    let path = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let content = args
        .get(1)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    let status = fs::write(path, content);

    if let Err(e) = status {
        let err_msg = v8::String::new(scope, &e.to_string()).unwrap().into();
        let exception = v8::Exception::error(scope, err_msg);
        scope.throw_exception(exception);
    } else {
        rv.set_bool(true);
    }
}

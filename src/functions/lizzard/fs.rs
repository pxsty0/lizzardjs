use std::{fs, path::Path};

use v8::{self, ContextScope, HandleScope};

pub fn init_fs(scope: &mut ContextScope<HandleScope>, lizzard: v8::Local<v8::Object>) {
    let fs = v8::ObjectTemplate::new(scope);

    let fs_exists = v8::FunctionTemplate::new(scope, exists_cb);
    fs.set(
        v8::String::new(scope, "exists").unwrap().into(),
        fs_exists.into(),
    );

    let fs_read_file = v8::FunctionTemplate::new(scope, read_file_cb);
    fs.set(
        v8::String::new(scope, "readFile").unwrap().into(),
        fs_read_file.into(),
    );

    let fs_append_file = v8::FunctionTemplate::new(scope, append_file_cb);
    fs.set(
        v8::String::new(scope, "appendFile").unwrap().into(),
        fs_append_file.into(),
    );

    let fs_write_file = v8::FunctionTemplate::new(scope, write_file_cb);
    fs.set(
        v8::String::new(scope, "writeFile").unwrap().into(),
        fs_write_file.into(),
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

fn read_file_cb(
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
    let status = fs::read_to_string(&file_path);

    match status {
        Ok(reading_data) => {
            println!("Dosya başarıyla okundu");
            rv.set(v8::String::new(scope, &reading_data).unwrap().into());
        }
        Err(e) => {
            let err_msg = v8::String::new(scope, &e.to_string()).unwrap().into();
            let exception = v8::Exception::error(scope, err_msg);
            scope.throw_exception(exception);
            return;
        }
    }
}

fn write_file_cb(
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
        return;
    } else {
        rv.set_bool(true);
    }
}

fn append_file_cb(
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
    let file_path = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let content = args
        .get(1)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    let read_status = fs::read_to_string(&file_path);

    let mut reading_data = String::from("");

    match read_status {
        Ok(reading_str) => {
            reading_data.push_str(&reading_str);
        }
        Err(..) => {
            reading_data.push_str("");
        }
    }
    reading_data.push_str(&content);

    let write_status = fs::write(file_path, reading_data);

    if let Err(e) = write_status {
        let err_msg = v8::String::new(scope, &e.to_string()).unwrap().into();
        let exception = v8::Exception::error(scope, err_msg);
        scope.throw_exception(exception);
        return;
    } else {
        rv.set_bool(true);
    }
}

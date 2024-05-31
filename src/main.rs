fn main() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        let handle_scope = &mut v8::HandleScope::new(isolate);

        let context = v8::Context::new(handle_scope);

        let scope = &mut v8::ContextScope::new(handle_scope, context);

        let global = context.global(scope);

        let console = v8::ObjectTemplate::new(scope);
        let console_log = v8::FunctionTemplate::new(scope, log_callback);
        console.set(
            v8::String::new(scope, "log").unwrap().into(),
            console_log.into(),
        );

        let console_env = v8::FunctionTemplate::new(scope, env_callback);
        console.set(
            v8::String::new(scope, "env").unwrap().into(),
            console_env.into(),
        );

        let console_obj = console.new_instance(scope).unwrap();
        let console_key = v8::String::new(scope, "console").unwrap();

        global.set(scope, console_key.into(), console_obj.into());

        let js_code: &str = r#"
        console.log("selam test 123");
        console.log("developed by mustafa 'pxsty' kok");
        const result = console.env("sa");
        console.log(JSON.parse(result).name)
        "#;

        let code = v8::String::new(scope, js_code).unwrap();
        let try_catch = &mut v8::TryCatch::new(scope);
        let script = v8::Script::compile(try_catch, code, None);

        match script {
            Some(script) => {
                let result = script.run(try_catch);
                if result.is_none() {
                    let exception = try_catch.exception().unwrap();
                    let exception_string = exception.to_string(try_catch).unwrap();
                    println!(
                        "js err: {}",
                        exception_string.to_rust_string_lossy(try_catch)
                    );
                }
            }
            None => {
                let exception = try_catch.exception().unwrap();
                let exception_string = exception.to_string(try_catch).unwrap();
                println!(
                    "js completion err: {}",
                    exception_string.to_rust_string_lossy(try_catch)
                );
            }
        }
    }

    unsafe {
        v8::V8::dispose();
    }
    v8::V8::dispose_platform();
}

fn log_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    // JavaScript argÃ¼manlarÄ±nÄ± al ve String'e dÃ¶nÃ¼ÅŸtÃ¼r
    let message = args.get(0);
    let message_str = message
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    // Rust tarafÄ±nda mesajÄ± konsola yazdÄ±r
    println!("{}", message_str);
}

fn env_callback(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let json_data = r#"{"name": "John", "age": 30}"#;
    let json_str = v8::String::new(scope, json_data).unwrap();
    rv.set(json_str.into());
}

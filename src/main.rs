mod functions;

use functions::console;
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

        console::init_console(scope, global);

        let js_code: &str = r#"
        console.log("selam test 123","mustafa kok");
        console.time("x");
        console.timeEnd();
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

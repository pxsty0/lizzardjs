mod functions;

use functions::console;

use std::{env, fs, path::Path};
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || Path::new(&args[1].to_string()).exists() == false {
        panic!("lizzard Error : invalid file path");
    }

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

        let name = v8::String::new(scope, &args[1].to_string()).unwrap();
        let undefined = v8::undefined(scope);

        let origin = v8::ScriptOrigin::new(
            scope,
            name.into(),
            0,
            0,
            false,
            0,
            undefined.into(),
            false,
            false,
            false,
        );

        let js_code: String = fs::read_to_string(args[1].to_string()).unwrap();
        let code = v8::String::new(scope, &js_code).unwrap();
        let try_catch = &mut v8::TryCatch::new(scope);
        let script = v8::Script::compile(try_catch, code, Some(&origin));

        match script {
            Some(script) => {
                let result = script.run(try_catch);
                if result.is_none() {
                    let stack_trace = try_catch.stack_trace().unwrap();
                    let stack_trace_string = stack_trace.to_string(try_catch).unwrap();
                    println!("{}", stack_trace_string.to_rust_string_lossy(try_catch));
                }
            }
            None => {
                let exception = try_catch.exception().unwrap();
                let exception_string = exception.to_string(try_catch).unwrap();
                println!(
                    "CompletionError: {}",
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

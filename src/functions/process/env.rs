use std::{env as renv, fs, path::Path};

use v8::{ContextScope, HandleScope};

pub fn init_env(scope: &mut ContextScope<HandleScope>, process: v8::Local<v8::Object>) {
    let env = v8::ObjectTemplate::new(scope);

    let home_dir = renv::current_dir().unwrap();
    let env_path = Path::new(&home_dir).join(".env");

    match fs::read_to_string(env_path) {
        Ok(env_raw) => {
            let splitted_env: Vec<&str> = env_raw.split("\n").filter(|x| !x.is_empty()).collect();
            for line in splitted_env {
                let splitted_line: Vec<&str> = line.split("=").collect();

                let key = v8::String::new(scope, splitted_line[0]).unwrap().into();
                let value = v8::String::new(scope, splitted_line[1]).unwrap().into();

                env.set(key, value);
            }
        }
        Err(..) => {
            println!("env yoktu es geçtim");
            return;
        }
    }

    let env_key = v8::String::new(scope, "env").unwrap();
    let env_obj = env.new_instance(scope).unwrap();

    process.set(scope, env_key.into(), env_obj.into());
}

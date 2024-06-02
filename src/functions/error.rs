pub fn _error(scope: &mut v8::HandleScope, message: String) {
    let err_msg = v8::String::new(scope, &format!("{}", message))
        .unwrap()
        .into();
    let exception = v8::Exception::error(scope, err_msg);
    scope.throw_exception(exception);
}

pub fn reference_error(scope: &mut v8::HandleScope, message: &str) {
    let err_msg = v8::String::new(scope, &format!("{}", message))
        .unwrap()
        .into();
    let exception = v8::Exception::reference_error(scope, err_msg);
    scope.throw_exception(exception);
}

pub fn _syntax_error(scope: &mut v8::HandleScope, message: &str) {
    let err_msg = v8::String::new(scope, &format!("{}", message))
        .unwrap()
        .into();
    let exception = v8::Exception::syntax_error(scope, err_msg);
    scope.throw_exception(exception);
}

pub fn _type_error(scope: &mut v8::HandleScope, message: &str) {
    let err_msg = v8::String::new(scope, &format!("{}", message))
        .unwrap()
        .into();
    let exception = v8::Exception::type_error(scope, err_msg);
    scope.throw_exception(exception);
}

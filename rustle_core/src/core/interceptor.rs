#[macro_export]
/// The `interceptor` macro is used to implement an `intercept` function for a given type.
/// This function is intended to be used for intercepting and potentially altering the execution of a function.
///
/// # Examples
///
/// ```
/// struct MyInterceptor;
///
/// interceptor!(MyInterceptor);
///
/// let my_interceptor = MyInterceptor;
/// my_interceptor.intercept(&mut some_function);
/// ```
macro_rules! interceptor {
    ($t:ty) => {
        impl $t {
            /// The `intercept` function takes a mutable reference to any type.
            /// It is intended to be overridden with logic to intercept and potentially alter the execution of a function.
            fn intercept(&self, execution: &mut dyn std::any::Any) {
                // TODO: Implement the logic of the interceptor here
            }
        }
    };
}

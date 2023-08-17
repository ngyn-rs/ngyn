/// `Module` is a macro that generates a struct and its implementation.
/// The struct `Module` contains a vector of boxed dynamic components.
/// 
/// # Examples
///
/// ```
/// module!(MyComponent, MyOtherComponent);
/// let module = Module::new();
/// let my_component: Option<&MyComponent> = module.get();
/// ```
#[macro_export]
macro_rules! module {
    ($($t:ty),+ $(,)?) => {
        /// Struct `Module` contains a vector of boxed dynamic components.
        pub struct Module {
            components: Vec<Box<dyn std::any::Any>>,
        }

        impl Module {
            /// Creates a new `Module` with the specified components.
            /// 
            /// # Examples
            ///
            /// ```
            /// let module = Module::new();
            /// ```
            pub fn new() -> Self {
                Module {
                    components: vec![$(Box::new(<$t>::new())),+],
                }
            }

            /// Returns a reference to a component of type `T` if it exists in the module.
            /// 
            /// # Examples
            ///
            /// ```
            /// let my_component: Option<&MyComponent> = module.get();
            /// ```
            pub fn get<T: 'static>(&self) -> Option<&T> {
                for component in &self.components {
                    if let Some(c) = component.downcast_ref::<T>() {
                        return Some(c);
                    }
                }
                None
            }

            /// Adds a component to the module.
            /// 
            /// # Examples
            ///
            /// ```
            /// module.add(MyComponent::new());
            /// ```
            pub fn add<T: 'static>(&mut self, component: T) {
                self.components.push(Box::new(component));
            }

            /// Removes a component from the module.
            /// 
            /// # Examples
            ///
            /// ```
            /// module.remove::<MyComponent>();
            /// ```
            pub fn remove<T: 'static>(&mut self) {
                self.components.retain(|component| component.downcast_ref::<T>().is_none());
            }
        }
    };
}

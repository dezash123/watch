//! allows for kernel configuration
/// trait that should be implemented by all config structs
pub trait Config<T: ?Sized> {
    const DEFAULT: Self;
}

/// trait to be implemented by all things configurable
pub trait Configurable<T: Config<Self>> {
    fn configure(&mut self, config: T);
}

pub trait ConfigOption {
    type OptionType;
    const LABEL: &'static str;
}

pub struct ExampleConfig {}

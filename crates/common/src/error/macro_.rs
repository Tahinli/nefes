#[macro_export]
macro_rules! error_from {
    ($target:ty { $($rest:tt)* }) => {
        $crate::error_from!(@arm $target, $($rest)*);
    };

    (@arm $target:ty, $source:ty => $variant:path as $middle:ty, $($rest:tt)*) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                $variant(<$middle>::from(value).into())
            }
        }
        $crate::error_from!(@arm $target, $($rest)*);
    };

    (@arm $target:ty, $source:ty => $variant:path, $($rest:tt)*) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                $variant(value)
            }
        }
      $crate::error_from!(@arm $target, $($rest)*);
    };

    (@arm $target:ty,) => {};
}

#[macro_export]
macro_rules! error_from_display {
    ($target:ty { $($source:ty => $variant:path),+ $(,)? }) => {
        $(
            impl From<$source> for $target {
                fn from(value: $source) -> Self {
                    $variant(value.to_string())
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! error_display {
    ($target:ty,
        delegate = [$($variant:path),* $(,)?],
        unit = [$($unit:path => $message:literal),* $(,)?] $(,)?
    ) => {
        impl std::fmt::Display for $target {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($variant(error_value) => std::fmt::Display::fmt(error_value, formatter),)*
                    $($unit => formatter.write_str($message),)*
                }
            }
        }
    };
}

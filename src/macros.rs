macro_rules! handle {
    ($name: ident) => {
        impl ::std::os::windows::io::FromRawHandle for $name {
            unsafe fn from_raw_handle(handle: ::std::os::windows::io::RawHandle) -> $name {
                $name($crate::handle::Handle::from_raw_handle(handle))
            }
        }

        impl ::std::os::windows::io::AsRawHandle for $name {
            fn as_raw_handle(&self) -> ::std::os::windows::io::RawHandle {
                self.0.as_raw_handle()
            }
        }

        impl ::std::os::windows::io::IntoRawHandle for $name {
            fn into_raw_handle(self) -> ::std::os::windows::io::RawHandle {
                self.0.into_raw_handle()
            }
        }
    }
}

macro_rules! access {
    (
        $name: ident,
        $($var_name: ident => $var_value: expr),+;
        $($fn_name: ident => $fn_value: expr),*
    ) => {
        #[cfg_attr(feature = "clippy", allow(enum_variant_names))]
        #[repr(u32)]
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        pub enum $name {
            $($var_name = $var_value),+
        }

        impl $crate::access::Access for $name {
            fn mask(&self) -> u32 {
                *self as u32
            }
        }

        impl $name {
            $(
                #[inline]
                pub fn $fn_name() -> $crate::access::CustomAccess {
                    $crate::access::CustomAccess($fn_value)
                }
            )*
        }

        impl $crate::access::CombinableAccess for $name {}

        impl<T: $crate::access::CombinableAccess> ::std::ops::BitOr<T> for $name {
            type Output = $crate::access::CustomAccess;

            fn bitor(self, other: T) -> $crate::access::CustomAccess {
                $crate::access::CombinableAccess::combine(&self, other)
            }
        }
    }
}

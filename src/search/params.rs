use std::cell::UnsafeCell;

// `std::cell::SyncUnsafeCell` is nightly only
pub struct SyncUnsafeCell<T>(pub UnsafeCell<T>);

unsafe impl<T: Sync> Sync for SyncUnsafeCell<T> {}

macro_rules! params {
    ($($name:ident : $ty:ty => $default:literal;)*) => {
        pub struct Params;

        $(
            #[cfg(feature = "tune")]
            pub static $name: SyncUnsafeCell<$ty> = SyncUnsafeCell::new($default);
        )*

        impl Params {
            $(
                #[cfg(feature = "tune")]
                pub const fn $name() -> $ty { unsafe { *$name.0.get() } }

                #[cfg(not(feature = "tune"))]
                pub const fn $name() -> $ty { $default }
            )*

            #[cfg(feature = "tune")]
            pub fn set_param(name: &str, value: String) {
                match name {
                    $(
                        stringify!($name) => {
                            let value = match value.parse::<$ty>() {
                                Ok(value) => value,
                                Err(e) => {
                                    println!("info string {:?}", UciParseError::InvalidInteger(e));
                                    return;
                                }
                            };

                            unsafe { *$name.0.get() = value };
                        },
                    )*
                }
            }
        }
    }
}

params! {
    quiet_bonus_base:  i32 => 128;
    quiet_bonus_scale: i32 => 128;
    quiet_bonus_max:   i32 => 2048;
    quiet_malus_base:  i32 => 128;
    quiet_malus_scale: i32 => 128;
    quiet_malus_max:   i32 => 2048;

    rfp_base: i32 => 0;
    rfp_scale: i32 => 50;
}

impl Params {
    #[inline]
    pub fn quiet_bonus(depth: i32) -> i32 {
        (Self::quiet_bonus_base() + Self::quiet_bonus_scale() * depth).min(Self::quiet_bonus_max())
    }

    #[inline]
    pub fn quiet_malus(depth: i32) -> i32 {
        (Self::quiet_malus_base() + Self::quiet_malus_scale() * depth).min(Self::quiet_malus_max())
    }

    #[inline]
    pub const fn rfp_margin(depth: i32) -> i32 {
        Self::rfp_base() + Self::rfp_scale() * depth
    }
}

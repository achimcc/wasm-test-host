use ark_std::vec::Vec;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(nonstandard_style)]
static mut _BOUNDARY: Option<&'static (dyn NativeBoundary)> = None;

#[cfg(feature = "fallback")]
#[allow(nonstandard_style)]
static mut _BOUNDARY_FALLBACK: bool = true;

#[cfg(not(feature = "fallback"))]
#[allow(nonstandard_style)]
static mut _BOUNDARY_FALLBACK: bool = false;

pub struct Boundary;
impl Boundary {
    #[allow(dead_code)]
    pub fn set(nb: Option<&'static (dyn NativeBoundary)>) {
        unsafe {
            _BOUNDARY = nb;
        }
    }

    #[allow(dead_code)]
    pub fn set_fallback(fall: bool) {
        unsafe {
            _BOUNDARY_FALLBACK = fall;
        }
    }

    #[allow(dead_code)]
    pub fn get() -> Option<&'static (dyn NativeBoundary)> {
        unsafe { _BOUNDARY }
    }

    #[allow(dead_code)]
    pub fn get_fallback() -> bool {
        unsafe { _BOUNDARY_FALLBACK }
    }

    #[allow(dead_code)]
    pub fn disable() -> Option<&'static (dyn NativeBoundary)> {
        let old = Boundary::get();
        Boundary::set(None);
        Boundary::set_fallback(true);
        old
    }
}

#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CallId {
    // variable_base::multi_scalar_mul
    VBMul,
    // fixed_base::multi_scalar_mul
    FBMul,
    // fixed_base::get_window_table
    FBWindowTable,
    // fixed_base::windowed_mul
    FBWindowMul,
    // ProjectiveCurve::batch_normalization
    ProjBN,
}

pub trait NativeBoundary {
    // This methods call the native host with serialized args
    fn call(&self, id: CallId, args: Vec<u8>, cp: Vec<u8>) -> Result<Vec<u8>, ()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NB;
    impl NativeBoundary for NB {
        fn call(&self, _: CallId, _: Vec<u8>, _: Vec<u8>) -> Result<Vec<u8>, ()> {
            Ok(vec![])
        }
    }

    #[test]
    fn test_set_boundary() {
        Boundary::set(Some(&NB));
        Boundary::get().unwrap();

        Boundary::set_fallback(true);
        assert_eq!(Boundary::get_fallback(), true);
    }
}
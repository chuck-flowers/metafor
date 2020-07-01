use metafor::metafor;
use core::convert::TryInto;

enum PolyValue {
    Bool(bool),
    Float(f64),
    SignedInteger(i128),
    Str(String),
    UnsignedInteger(u128),
}

#[metafor(variant = [
    (Bool, bool), 
    (Float, f64), 
    (SignedInteger, i128), 
    (Str, String), 
    (UnsignedInteger, u128)
])]
impl From<__variant__1__> for PolyValue {
    fn from(src: __variant__1__) -> Self {
        PolyValue::__variant__0__(src)
    }
}

#[metafor(variant = [
    { name: Bool, value: bool }, 
    { name: Float, value: f64 }, 
    { name: SignedInteger, value: i128 }, 
    { name: Str, value: String }, 
    { name: UnsignedInteger, value: u128 }
])]
impl TryInto<__variant__value__> for PolyValue {
    type Error = ();
    fn try_into(self) -> Result<__variant__value__, Self::Error> {
        if let PolyValue::__variant__name__(inner) = self {
            Ok(inner)
        } else {
            Err(())
        }
    } 
}

fn main() {
    let bool_val = PolyValue::from(true);
    let float_val = PolyValue::from(1.0f64);
    let signed_val = PolyValue::from(1i128);
    let str_val = PolyValue::from(String::new());
    let unsigned_val = PolyValue::from(1u128);

    let _: bool = bool_val.try_into().unwrap();
    let _: f64 = float_val.try_into().unwrap();
    let _: i128 = signed_val.try_into().unwrap();
    let _: String = str_val.try_into().unwrap();
    let _: u128 = unsigned_val.try_into().unwrap();
}
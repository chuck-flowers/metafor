use metafor::metafor;

trait Integer {}

#[metafor(integer_type = [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128])]
impl Integer for __integer_type__ {}

fn main() {
    let a = 0u8;
    let b = 0u16;
    let c = 0u32;
    let d = 0u64;
    let e = 0u128;
    let f = 0i8;
    let g = 0i16;
    let h = 0i32;
    let i = 0i64;
    let j = 0i128;

    integer_fn(a);
    integer_fn(b);
    integer_fn(c);
    integer_fn(d);
    integer_fn(e);
    integer_fn(f);
    integer_fn(g);
    integer_fn(h);
    integer_fn(i);
    integer_fn(j);
}

fn integer_fn(_: impl Integer) {}

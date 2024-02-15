use std::convert::TryInto;

pub trait ReadInteger<T> {
    fn from_le_bytes(data: &[u8]) -> T;
    fn from_be_bytes(data: &[u8]) -> T;
}

macro_rules! impl_read_integer {
    ($($t:ty),+) => {
        $(impl ReadInteger<$t> for $t {
            fn from_le_bytes(data: &[u8]) -> $t {
                <$t>::from_le_bytes(data.try_into().unwrap())
            }
            fn from_be_bytes(data: &[u8]) -> $t {
                <$t>::from_be_bytes(data.try_into().unwrap())
            }
        })+
    }
}

impl_read_integer!(u8, i16, i32, u32, i64);

fn read_integer<T: ReadInteger<T>>(data: &[u8]) -> T {
    T::from_le_bytes(&data[..std::mem::size_of::<T>()])
}



fn main() {
    let slice= &[254, 255, 0, 0];

    let i32_integer = read_integer::<i32>(slice);
    let u32_integer = read_integer::<u32>(slice);
    println!("{}, {}", i32_integer, u32_integer);
}

use std::str;
fn main() {
    let dao = '道';
    let dao_u32 = dao as u32;

    assert_eq!(36947, dao_u32);

    //utf-8转字符串
    let dao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    assert_eq!("道", dao)
}

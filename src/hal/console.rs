use crate::arch::get_arch_interfaces;
use spin::Mutex;

pub struct IDesc {
    pub init: fn(),
    pub putc: fn(char),
    pub puts: fn(&str),
    pub get_w: fn() -> usize,
    pub get_h: fn() -> usize,
    pub clear: fn(),
}

const ICONSOLE: Mutex<&IDesc> = Mutex::new(&get_arch_interfaces().iconsole);

pub fn putc(c: char) {
    (ICONSOLE.lock().putc)(c);
}

pub fn puts(s: &str) {
    (ICONSOLE.lock().puts)(s);
}

pub fn get_w() -> usize {
    (ICONSOLE.lock().get_w)()
}

pub fn get_h() -> usize {
    (ICONSOLE.lock().get_h)()
}

pub fn clear() {
    (ICONSOLE.lock().clear)();
}

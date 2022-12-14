//! Interrupt Descriptor Table.

use super::gdt;
use core::arch::asm;

#[repr(u8)]
enum Attribute {
    TaskGate = 0b0101,
    TrapGate16 = 0b0111,
    TrapGate32 = 0b1111,
    IntGate16 = 0b0110,
    IntGate32 = 0b1110,
    DPL0 = 0,
    DPL1 = 0b01_00000,
    DPL2 = 0b10_00000,
    DPL3 = 0b11_00000,
    Present = 0b1000_0000,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IDTEntry {
    offset_1: u16,
    selector: u16,
    ist: u8,
    type_attributes: u8,
    offset_2: u16,
    offset_3: u32,
    _zero: u32,
}

impl IDTEntry {
    const fn new_null() -> Self {
        Self {
            offset_1: 0,
            selector: 0,
            ist: 0,
            type_attributes: 0,
            offset_2: 0,
            offset_3: 0,
            _zero: 0,
        }
    }
}

#[repr(C, packed)]
struct IDTR {
    size: u16,
    paged_offset: u64,
}

impl IDTR {
    fn new(idt: &IDT) -> Self {
        Self {
            size: core::mem::size_of::<IDT>() as u16,
            paged_offset: idt as *const _ as u64,
        }
    }
}

util::proc_macros::statemachine! {
    #[repr(C)]
    #[repr(align(0x10))]
    static IDT = {
        entries: [IDTEntry; 256] = [IDTEntry::new_null(); 256],
    }
    impl {
        /// Sets a descriptor in the IDT.
        pub fn set_descriptor(&mut self, index: u8, interrupt_handler: unsafe extern "C" fn()) {
            self.set_entry(index, interrupt_handler as u64);
        }

        /// Removes the descriptor from the table.
        pub fn remove_descriptor(&mut self, index: u8) {
            self.set_entry_from(index, IDTEntry::new_null());
        }

        /// Installs the IDT.
        pub fn install(&self) {
            let idtr = IDTR::new(self);
            let idtr_ptr = &idtr as *const IDTR as u64;
            unsafe {
                asm!(
                    "lidt [rax]",
                    in("rax")(idtr_ptr)
                )
            }
        }

        fn set_entry(&mut self, entry: u8, val: u64) {
            let entry = &mut self.entries[entry as usize];
            entry.offset_1 = val as u16;
            entry.offset_2 = (val >> 16) as u16;
            entry.offset_3 = (val >> 32) as u32;
            entry.selector = gdt::KERNEL_CODE_SELECTOR;
            entry.ist = 0;
            entry.type_attributes = Attribute::Present as u8 | Attribute::IntGate32 as u8;
            entry._zero = 0;
        }

        fn set_entry_from(&mut self, entry: u8, val: IDTEntry) {
            let entry = &mut self.entries[entry as usize];
            *entry = val;
        }
    }
}

static _ASSERT_ENTRY_SIZE: () = assert!(core::mem::size_of::<IDTEntry>() == 16);
static _ASSERT_IDT_SIZE: () = assert!(core::mem::size_of::<IDT>() == 16 * 256);
static _ASSERT_IDR_SIZE: () = assert!(core::mem::size_of::<IDTR>() == 10);

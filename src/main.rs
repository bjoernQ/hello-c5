#![no_std]
#![no_main]

//use esp_println::println;
use core::{arch::{asm, global_asm}, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

global_asm!(
    "
    .global _start
_start:
     j main
     "
);

#[unsafe(link_section = ".data")]
static HELLO: &[u8] =
    b"Hello, world! Hello, world! Hello, world! Hello, world! Hello, world! Hello, world! ";

#[unsafe(no_mangle)]
unsafe extern "C" fn main() -> ! {
    const UART_TX_ONE_CHAR2: u32 = 0x40000058;

    let uart_tx_one_char: extern "C" fn(u8) = unsafe { core::mem::transmute(UART_TX_ONE_CHAR2) };

    loop {
        for byte in HELLO {
            uart_tx_one_char(*byte);
        }
        uart_tx_one_char(b'\n' as u8);

        delay(10_000_000);
    }
}

fn delay(d: u32) {
    for _ in 0..d {
        unsafe {
            asm!("nop");
        }
    }
}


#[unsafe(export_name = "esp_app_desc")]
#[unsafe(link_section = ".flash.appdesc")]
#[used]
pub static ESP_APP_DESC: EspAppDesc = EspAppDesc::new_internal(
    "0.0.0",
    "gpio_interrupt",
    "0",
    "0",
    "0",
    0,
    u16::MAX,
    64 * 1024,
);

const ESP_APP_DESC_MAGIC_WORD: u32 = 0xABCD5432;

const fn str_to_cstr_array<const C: usize>(s: &str) -> [::core::ffi::c_char; C] {
    let bytes = s.as_bytes();
    let mut ret: [::core::ffi::c_char; C] = [0; C];
    let mut i = 0;
    loop {
        ret[i] = bytes[i] as _;
        i += 1;
        if i >= bytes.len() || i >= C {
            break;
        }
    }
    ret
}

#[repr(C)]
pub struct EspAppDesc {
    /// Magic word ESP_APP_DESC_MAGIC_WORD
    magic_word: u32,
    /// Secure version
    secure_version: u32,
    /// Reserved
    reserv1: [u32; 2],
    /// Application version
    version: [core::ffi::c_char; 32],
    /// Project name
    project_name: [core::ffi::c_char; 32],
    /// Compile time
    time: [core::ffi::c_char; 16],
    /// Compile date
    date: [core::ffi::c_char; 16],
    /// Version IDF
    idf_ver: [core::ffi::c_char; 32],
    /// sha256 of elf file
    app_elf_sha256: [u8; 32],
    /// Minimal eFuse block revision supported by image, in format: major * 100
    /// + minor
    min_efuse_blk_rev_full: u16,
    /// Maximal eFuse block revision supported by image, in format: major * 100
    /// + minor
    max_efuse_blk_rev_full: u16,
    /// MMU page size in log base 2 format
    mmu_page_size: u8,
    /// Reserved
    reserv3: [u8; 3],
    /// Reserved
    reserv2: [u32; 18],
}

impl EspAppDesc {
    /// Needs to be public since it's used by the macro
    #[doc(hidden)]
    #[expect(clippy::too_many_arguments, reason = "For internal use only")]
    pub const fn new_internal(
        version: &str,
        project_name: &str,
        build_time: &str,
        build_date: &str,
        idf_ver: &str,
        min_efuse_blk_rev_full: u16,
        max_efuse_blk_rev_full: u16,
        mmu_page_size: u32,
    ) -> Self {
        Self {
            magic_word: ESP_APP_DESC_MAGIC_WORD,
            secure_version: 0,
            reserv1: [0; 2],
            version: str_to_cstr_array(version),
            project_name: str_to_cstr_array(project_name),
            time: str_to_cstr_array(build_time),
            date: str_to_cstr_array(build_date),
            idf_ver: str_to_cstr_array(idf_ver),
            app_elf_sha256: [0; 32],
            min_efuse_blk_rev_full,
            max_efuse_blk_rev_full,
            mmu_page_size: (mmu_page_size.ilog2()) as u8,
            reserv3: [0; 3],
            reserv2: [0; 18],
        }
    }
}

#![no_main]
#![no_std]

#[macro_use]
mod keymap;
#[macro_use]
mod macros;
mod matrix;
mod uart;
mod vial;

use crate::keymap::{COL, NUM_LAYER, ROW};
use crate::matrix::Matrix;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::{
    bind_interrupts,
    flash::{Async, Flash},
    gpio::{AnyPin, Input, Output},
    peripherals::{self, USB},
    usb::{Driver, InterruptHandler},
};
// use embassy_rp::flash::Blocking;
use panic_probe as _;
use rmk::{
    config::{KeyboardUsbConfig, RmkConfig, StorageConfig, VialConfig},
    debounce::{default_bouncer::DefaultDebouncer, DebouncerTrait},
    split::{
        central::{run_peripheral_monitor, run_rmk_split_central_with_matrix},
        SPLIT_MESSAGE_MAX_SIZE,
    },
};
use static_cell::StaticCell;
use uart::BufferedHalfDuplexUart;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("RMK start!");
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Create the usb driver, from the HAL
    let driver = Driver::new(p.USB, Irqs);

    // Pin config
    let (input_pins, output_pins) = config_matrix_pins_rp!(
        peripherals: p,
        input: [PIN_14, PIN_13, PIN_12, PIN_11, PIN_10, PIN_9],
        output: [PIN_8, PIN_7, PIN_6, PIN_5, PIN_4]
    );

    // Use internal flash to emulate eeprom
    // Both blocking and async flash are support, use different API
    // let flash = Flash::<_, Blocking, FLASH_SIZE>::new_blocking(p.FLASH);
    let flash = Flash::<_, Async, FLASH_SIZE>::new(p.FLASH, p.DMA_CH0);

    let keyboard_usb_config = KeyboardUsbConfig {
        vid: 0x303A,
        pid: 0x4044,
        manufacturer: "Svalboard",
        product_name: "lightly",
        serial_number: "vial:f64c2b3c:000001",
    };

    let storage_config = StorageConfig {
        num_sectors: 16,
        ..Default::default()
    };

    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF);

    let keyboard_config = RmkConfig {
        storage_config,
        usb_config: keyboard_usb_config,
        vial_config,
        ..Default::default()
    };

    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    let uart_receiver = BufferedHalfDuplexUart::new(p.PIO0, p.PIN_0, tx_buf, rx_buf);

    let debouncer = DefaultDebouncer::<6, 5>::new();

    let matrix = Matrix::<
        _,
        _,
        _,
        0, // const ROW_OFFSET: usize,
        0, // const COL_OFFSET: usize,
        6, // const INPUT_PIN_NUM: usize, (COL for row2col)
        5, // const OUTPUT_PIN_NUM: usize, (ROW for row2col)
    >::new(input_pins, output_pins, debouncer);

    // Start serving
    join(
        run_rmk_split_central_with_matrix::<
            Output<'_>,
            _,
            Driver<'_, USB>,
            Flash<peripherals::FLASH, Async, FLASH_SIZE>,
            ROW,       // TOTAL_ROW: usize,
            COL,       // TOTAL_COL: usize,
            NUM_LAYER, // NUM_LAYER: usize,
        >(
            matrix,
            driver,
            flash,
            &mut keymap::get_default_keymap(),
            keyboard_config,
            spawner,
        ),
        run_peripheral_monitor::<
            5, // ROW: usize,
            6, // COL: usize,
            5, // ROW_OFFSET: usize,
            0, // COL_OFFSET: usize,
            _, // S: Read + Write,
        >(0, uart_receiver),
    )
    .await;
}

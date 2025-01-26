#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod matrix;
mod uart;

use crate::matrix::Matrix;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{AnyPin, Input, Output},
    peripherals::USB,
    usb::InterruptHandler,
};
use panic_probe as _;
use rmk::debounce::{default_bouncer::DefaultDebouncer, DebouncerTrait};
use rmk::split::{peripheral::run_rmk_split_peripheral_with_matrix, SPLIT_MESSAGE_MAX_SIZE};
use static_cell::StaticCell;
use uart::BufferedHalfDuplexUart;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("RMK start!");
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Pin config
    let (input_pins, output_pins) = config_matrix_pins_rp!(
        peripherals: p,
        input: [PIN_14, PIN_13, PIN_12, PIN_11, PIN_10, PIN_9],
        output: [PIN_8, PIN_7, PIN_6, PIN_5, PIN_4]
    );

    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    let uart_instance = BufferedHalfDuplexUart::new(p.PIO0, p.PIN_0, tx_buf, rx_buf);

    let debouncer = DefaultDebouncer::<6, 5>::new();

    let matrix = Matrix::<
        _,
        _,
        _,
        5, // const ROW_OFFSET: usize,
        0, // const COL_OFFSET: usize,
        6, // const INPUT_PIN_NUM: usize, (COL for row2col)
        5, // const OUTPUT_PIN_NUM: usize, (ROW for row2col)
    >::new(input_pins, output_pins, debouncer);

    // Start serving
    run_rmk_split_peripheral_with_matrix::<
        _, // M: MatrixTrait,
        _, // S: Write + Read,
        5, // ROW: usize,
        6, // COL: usize,
    >(matrix, uart_instance)
    .await;
}

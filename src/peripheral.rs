#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod uart;

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
use rmk::config::{MatrixConfig, RmkConfig};
use rmk::split::{peripheral::run_rmk_split_peripheral_with_config, SPLIT_MESSAGE_MAX_SIZE};
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
        input: [PIN_8, PIN_7, PIN_6, PIN_5, PIN_4],
        output: [PIN_14, PIN_13, PIN_12, PIN_11, PIN_10, PIN_9]
    );

    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    let uart_instance = BufferedHalfDuplexUart::new(p.PIO0, p.PIN_1, tx_buf, rx_buf);

    let matrix_config = MatrixConfig {
        sample_delay_micros: 90,
        scan_delay_micros: 100,
    };

    let keyboard_config = RmkConfig {
        matrix_config,
        ..Default::default()
    };

    // Start serving
    run_rmk_split_peripheral_with_config::<
        Input<'_>,
        Output<'_>,
        _, // S: Write + Read,
        5, // ROW: usize,
        6, // COL: usize,
    >(input_pins, output_pins, keyboard_config, uart_instance)
    .await;
}

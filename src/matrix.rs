use defmt::info;
use embassy_time::{Instant, Timer};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::digital::Wait;
use rmk::{
    debounce::{DebounceState, DebouncerTrait},
    event::KeyEvent,
    keyboard::KEY_EVENT_CHANNEL,
    matrix::{KeyState, MatrixTrait},
};

/// Matrix is the physical pcb layout of the keyboard matrix.
pub struct Matrix<
    In: Wait + InputPin,
    Out: OutputPin,
    D: DebouncerTrait,
    const ROW_OFFSET: usize,
    const COL_OFFSET: usize,
    const INPUT_PIN_NUM: usize,
    const OUTPUT_PIN_NUM: usize,
> {
    /// Input pins of the pcb matrix
    input_pins: [In; INPUT_PIN_NUM],
    /// Output pins of the pcb matrix
    output_pins: [Out; OUTPUT_PIN_NUM],
    /// Debouncer
    debouncer: D,
    /// Key state matrix
    key_states: [[KeyState; INPUT_PIN_NUM]; OUTPUT_PIN_NUM],
    /// Start scanning
    scan_start: Option<Instant>,
}

impl<
        In: Wait + InputPin,
        Out: OutputPin,
        D: DebouncerTrait,
        const ROW_OFFSET: usize,
        const COL_OFFSET: usize,
        const INPUT_PIN_NUM: usize,
        const OUTPUT_PIN_NUM: usize,
    > MatrixTrait
    for Matrix<In, Out, D, ROW_OFFSET, COL_OFFSET, INPUT_PIN_NUM, OUTPUT_PIN_NUM>
{
    const ROW: usize = OUTPUT_PIN_NUM;
    const COL: usize = INPUT_PIN_NUM;

    async fn scan(&mut self) {
        info!("Matrix scanning");
        loop {
            self.wait_for_key().await;

            // Scan matrix and send report
            for (out_idx, out_pin) in self.output_pins.iter_mut().enumerate() {
                // Pull up output pin, wait 1us ensuring the change comes into effect
                out_pin.set_high().ok();
                Timer::after_micros(90).await;
                for (in_idx, in_pin) in self.input_pins.iter_mut().enumerate() {
                    // Check input pins and debounce
                    let debounce_state = self.debouncer.detect_change_with_debounce(
                        in_idx,
                        out_idx,
                        in_pin.is_high().ok().unwrap_or_default(),
                        &self.key_states[out_idx][in_idx],
                    );

                    match debounce_state {
                        DebounceState::Debounced => {
                            self.key_states[out_idx][in_idx].toggle_pressed();
                            let (row, col, key_state) = (
                                (out_idx + ROW_OFFSET) as u8,
                                (in_idx + COL_OFFSET) as u8,
                                self.key_states[out_idx][in_idx],
                            );

                            KEY_EVENT_CHANNEL
                                .send(KeyEvent {
                                    row,
                                    col,
                                    pressed: key_state.pressed,
                                })
                                .await;
                        }
                        _ => (),
                    }

                    // If there's key still pressed, always refresh the self.scan_start
                    if self.key_states[out_idx][in_idx].pressed {
                        self.scan_start = Some(Instant::now());
                    }
                }
                out_pin.set_low().ok();
            }

            Timer::after_micros(100).await;
        }
    }

    fn get_key_state(&mut self, row: usize, col: usize) -> KeyState {
        self.key_states[row][col]
    }

    fn update_key_state(&mut self, row: usize, col: usize, f: impl FnOnce(&mut KeyState)) {
        f(&mut self.key_states[row][col]);
    }

    async fn wait_for_key(&mut self) {
        use embassy_futures::select::select_slice;
        use heapless::Vec;

        if let Some(start_time) = self.scan_start {
            // If not key over 2 secs, wait for interupt in next loop
            if start_time.elapsed().as_secs() < 1 {
                return;
            } else {
                self.scan_start = None;
            }
        }
        // First, set all output pin to high
        for out in self.output_pins.iter_mut() {
            out.set_high().ok();
        }
        Timer::after_micros(1).await;
        info!("Waiting for high");
        let mut futs: Vec<_, INPUT_PIN_NUM> = self
            .input_pins
            .iter_mut()
            .map(|input_pin| input_pin.wait_for_high())
            .collect();
        let _ = select_slice(futs.as_mut_slice()).await;

        // Set all output pins back to low
        for out in self.output_pins.iter_mut() {
            out.set_low().ok();
        }

        self.scan_start = Some(Instant::now());
    }
}

impl<
        In: Wait + InputPin,
        Out: OutputPin,
        D: DebouncerTrait,
        const ROW_OFFSET: usize,
        const COL_OFFSET: usize,
        const INPUT_PIN_NUM: usize,
        const OUTPUT_PIN_NUM: usize,
    > Matrix<In, Out, D, ROW_OFFSET, COL_OFFSET, INPUT_PIN_NUM, OUTPUT_PIN_NUM>
{
    /// Initialization of central
    pub fn new(
        input_pins: [In; INPUT_PIN_NUM],
        output_pins: [Out; OUTPUT_PIN_NUM],
        debouncer: D,
    ) -> Self {
        Matrix {
            input_pins,
            output_pins,
            debouncer,
            key_states: [[KeyState::default(); INPUT_PIN_NUM]; OUTPUT_PIN_NUM],
            scan_start: None,
        }
    }
}

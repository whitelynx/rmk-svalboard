use defmt::info;
use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};
use rmk::{
    debounce::{DebounceState, DebouncerTrait},
    event::KeyEvent,
    keyboard::KEY_EVENT_CHANNEL,
    matrix::{KeyState, MatrixTrait},
};

/// Matrix is the physical pcb layout of the keyboard matrix.
pub struct Matrix<
    In: InputPin,
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
}

impl<
        In: InputPin,
        Out: OutputPin,
        D: DebouncerTrait,
        const ROW_OFFSET: usize,
        const COL_OFFSET: usize,
        const INPUT_PIN_NUM: usize,
        const OUTPUT_PIN_NUM: usize,
    > MatrixTrait for Matrix<In, Out, D, ROW_OFFSET, COL_OFFSET, INPUT_PIN_NUM, OUTPUT_PIN_NUM>
{
    const ROW: usize = OUTPUT_PIN_NUM;
    const COL: usize = INPUT_PIN_NUM;

    async fn scan(&mut self) {
        info!("Matrix scanning");
        loop {
            // Scan matrix and send report
            for (out_idx, out_pin) in self.output_pins.iter_mut().enumerate() {
                // Pull up output pin, wait 1us ensuring the change comes into effect
                out_pin.set_low().ok();
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
                                out_idx as u8,
                                in_idx as u8,
                                self.key_states[out_idx][in_idx],
                            );

                            // row is finger/thumb (0 and 5 are thumbs), col is position in the cluster.
                            // We want to invert down/center (2) keys in all finger clusters, and
                            // double-down (5) in thumb clusters.
                            let pressed = match (out_idx + ROW_OFFSET, in_idx + COL_OFFSET) {
                                (0, 5)
                                | (1, 2)
                                | (2, 2)
                                | (3, 2)
                                | (4, 2)
                                | (5, 5)
                                | (6, 2)
                                | (7, 2)
                                | (8, 2)
                                | (9, 2) => key_state.pressed,
                                _ => !key_state.pressed,
                            };

                            KEY_EVENT_CHANNEL.send(KeyEvent { row, col, pressed }).await;
                        }
                        _ => (),
                    }
                }
                out_pin.set_high().ok();
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
}

impl<
        In: InputPin,
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
        }
    }
}

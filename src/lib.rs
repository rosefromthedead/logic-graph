pub mod circuit;
pub mod components;
pub mod pin_state;

use pin_state::PinState;

#[derive(Clone)]
pub struct Connection {
    pub state: PinState,
    pub source_id: usize,
    pub sink_id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputPinCount {
    /// The component has a fixed number of pins.
    Fixed(usize),
    /// The component can have any number of pins on this side.
    Any,
}

pub trait Component {
    fn calculate(&self, inputs: &[PinState]) -> Vec<PinState>;
    fn input_count(&self) -> InputPinCount;
    fn output_count(&self) -> usize;
}

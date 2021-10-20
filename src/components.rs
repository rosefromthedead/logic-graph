use std::{cell::Cell, rc::Rc};

use crate::{pin_state::PinState, Component, InputPinCount};

pub struct InputPin(Rc<Cell<PinState>>);

impl InputPin {
    pub fn new(value: Rc<Cell<PinState>>) -> Self {
        Self(value)
    }
}

impl Component for InputPin {
    fn calculate(&self, _inputs: &[PinState]) -> Vec<PinState> {
        debug_assert!(_inputs.is_empty());

        vec![self.0.get()]
    }

    fn input_count(&self) -> InputPinCount {
        InputPinCount::Fixed(0)
    }

    fn output_count(&self) -> usize {
        1
    }
}

pub struct OutputPin(Rc<Cell<PinState>>);

impl OutputPin {
    pub fn new(value: Rc<Cell<PinState>>) -> Self {
        Self(value)
    }
}

impl Component for OutputPin {
    fn calculate(&self, inputs: &[PinState]) -> Vec<PinState> {
        debug_assert_eq!(inputs.len(), 1);
        self.0.set(inputs[0]);

        vec![]
    }

    fn input_count(&self) -> InputPinCount {
        InputPinCount::Fixed(1)
    }

    fn output_count(&self) -> usize {
        0
    }
}

pub struct AndGate;

impl Component for AndGate {
    fn calculate(&self, inputs: &[PinState]) -> Vec<PinState> {
        let output = if inputs.iter().any(|&s| s == PinState::Low) {
            PinState::Low
        } else {
            if inputs.iter().any(PinState::is_error) {
                PinState::Undefined
            } else {
                PinState::High
            }
        };
        vec![output]
    }

    fn input_count(&self) -> InputPinCount {
        InputPinCount::Any
    }

    fn output_count(&self) -> usize {
        1
    }
}

pub struct OrGate;

impl Component for OrGate {
    fn calculate(&self, inputs: &[PinState]) -> Vec<PinState> {
        let output = if inputs.iter().any(|&s| s == PinState::High) {
            PinState::High
        } else {
            if inputs.iter().any(PinState::is_error) {
                PinState::Undefined
            } else {
                PinState::Low
            }
        };
        vec![output]
    }

    fn input_count(&self) -> InputPinCount {
        InputPinCount::Any
    }

    fn output_count(&self) -> usize {
        1
    }
}

pub struct NandGate;

impl Component for NandGate {
    fn calculate(&self, inputs: &[PinState]) -> Vec<PinState> {
        let output = if inputs.iter().any(|&s| s == PinState::Low) {
            PinState::High
        } else {
            if inputs.iter().any(PinState::is_error) {
                PinState::Undefined
            } else {
                PinState::Low
            }
        };
        vec![output]
    }

    fn input_count(&self) -> InputPinCount {
        InputPinCount::Any
    }

    fn output_count(&self) -> usize {
        1
    }
}

pub struct NorGate;

impl Component for NorGate {
    fn calculate(&self, inputs: &[PinState]) -> Vec<PinState> {
        let output = if inputs.iter().any(|&s| s == PinState::High) {
            PinState::Low
        } else {
            if inputs.iter().any(PinState::is_error) {
                PinState::Undefined
            } else {
                PinState::High
            }
        };
        vec![output]
    }

    fn input_count(&self) -> InputPinCount {
        InputPinCount::Any
    }

    fn output_count(&self) -> usize {
        1
    }
}

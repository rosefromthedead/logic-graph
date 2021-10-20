use std::{cell::Cell, io::stdin, rc::Rc};

use logic_graph::{
    circuit::{Circuit, ConnectError},
    components::{InputPin, NorGate, OutputPin},
    pin_state::PinState,
};

fn main() -> Result<(), ConnectError> {
    let mut circuit = Circuit::new();
    let top_nor = circuit.add_component(Box::new(NorGate));
    let bottom_nor = circuit.add_component(Box::new(NorGate));
    let set_value = Rc::new(Cell::new(PinState::Low));
    let set_input = circuit.add_component(Box::new(InputPin::new(set_value.clone())));
    let reset_value = Rc::new(Cell::new(PinState::Low));
    let reset_input = circuit.add_component(Box::new(InputPin::new(reset_value.clone())));
    let q_value = Rc::new(Cell::new(PinState::Low));
    let q_output = circuit.add_component(Box::new(OutputPin::new(q_value.clone())));
    let not_q_value = Rc::new(Cell::new(PinState::Low));
    let not_q_output = circuit.add_component(Box::new(OutputPin::new(not_q_value.clone())));

    circuit.connect(set_input, 0, top_nor, 0)?;
    circuit.connect(reset_input, 0, bottom_nor, 0)?;
    circuit.connect(top_nor, 0, bottom_nor, 1)?;
    circuit.connect(bottom_nor, 0, top_nor, 1)?;
    circuit.connect(top_nor, 0, not_q_output, 0)?;
    circuit.connect(bottom_nor, 0, q_output, 0)?;

    let mut buf = String::new();
    loop {
        buf.clear();
        let _ = stdin().read_line(&mut buf).unwrap();
        match &*buf.trim() {
            "s" => {
                let new = !set_value.get();
                set_value.set(new);
                println!("S <- {:?}", new);
                circuit.recalculate(set_input);
            },
            "r" => {
                let new = !reset_value.get();
                reset_value.set(new);
                println!("R <- {:?}", new);
                circuit.recalculate(reset_input);
            },
            _ => continue,
        }
        println!("Q = {:?}", q_value.get());
        println!("¬Q = {:?}", not_q_value.get());
    }
}

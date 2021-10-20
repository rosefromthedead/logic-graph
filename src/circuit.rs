use petgraph::{graph::NodeIndex, Direction, Graph};

use crate::{pin_state::PinState, Component, Connection, InputPinCount};

struct GraphNode {
    component: Box<dyn Component>,
    input_state: Vec<PinState>,
    output_state: Vec<PinState>,
}

pub struct Circuit {
    graph: Graph<GraphNode, Connection>,
}

#[derive(Debug)]
pub enum ConnectError {
    ComponentNotFound(NodeIndex),
    InputAlreadyConnected,
    OutputPinDoesntExist,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) -> NodeIndex {
        let input_state = match component.input_count() {
            InputPinCount::Fixed(input_count) => vec![PinState::Undefined; input_count],
            _ => Vec::new(),
        };
        let n_outputs = component.output_count();
        self.graph.add_node(GraphNode {
            component,
            input_state,
            output_state: vec![PinState::Floating; n_outputs],
        })
    }

    pub fn connect(
        &mut self,
        from: NodeIndex,
        source_id: usize,
        to: NodeIndex,
        sink_id: usize,
    ) -> Result<(), ConnectError> {
        let source_node = self.graph.node_weight(from)
            .ok_or(ConnectError::ComponentNotFound(from))?;
        let _sink_node = self.graph.node_weight(to)
            .ok_or(ConnectError::ComponentNotFound(to))?;

        if self.graph.edges_directed(to, Direction::Incoming)
               .any(|edge| edge.weight().sink_id == sink_id) {
            return Err(ConnectError::InputAlreadyConnected);
        }

        let output_state = source_node.output_state[source_id].clone();

        let connection = Connection {
            state: *source_node.output_state.get(source_id)
                .ok_or(ConnectError::OutputPinDoesntExist)?,
            source_id,
            sink_id,
        };
        let _edge_id = self.graph.update_edge(from, to, connection);

        self.propagate(from, source_id, output_state);

        Ok(())
    }

    pub fn propagate(
        &mut self,
        changed_node_id: NodeIndex,
        changed_output: usize,
        new_state: PinState,
    ) {
        let sink_node_ids = self.graph.neighbors_directed(changed_node_id, Direction::Outgoing)
            .collect::<Vec<_>>();
        for sink_node_id in sink_node_ids.into_iter() {
            let connection = self.graph.edges_connecting(changed_node_id, sink_node_id)
                .next().unwrap()
                .weight().clone();
            if connection.source_id != changed_output {
                continue;
            }

            let sink_node = self.graph.node_weight_mut(sink_node_id).unwrap();
            match sink_node.input_state.get_mut(connection.sink_id) {
                Some(v) => *v = new_state,
                None => {
                    assert_eq!(sink_node.component.input_count(), InputPinCount::Any);
                    for _ in sink_node.input_state.len()..connection.sink_id {
                        sink_node.input_state.push(PinState::Undefined);
                    }
                    sink_node.input_state.push(new_state);
                },
            }
            let new_output_state = sink_node.component.calculate(&sink_node.input_state);

            let new_output_state2 = new_output_state.clone();
            let old_output_state =
                std::mem::replace(&mut sink_node.output_state, new_output_state2);

            let iter = old_output_state
                .iter()
                .zip(new_output_state.iter())
                .enumerate();
            for (idx, (&old, &new)) in iter {
                if old != new {
                    self.propagate(sink_node_id, idx, new);
                }
            }
        }
    }

    pub fn recalculate(&mut self, node_id: NodeIndex) {
        let node = self.graph.node_weight_mut(node_id).unwrap();
        let old_output_state = node.output_state.clone();
        let new_output_state = node.component.calculate(&node.input_state);
        let iter = old_output_state
            .iter()
            .zip(new_output_state.iter())
            .enumerate();
        for (idx, (&old, &new)) in iter {
            if old != new {
                self.propagate(node_id, idx, new);
            }
        }
    }
}

use crate::hack_computer::{chips::flipflop::LatchCircuit, gates::gates_b1::mux};

#[derive(Clone, Copy)]
pub struct Register1Bit {
    child_circuit: LatchCircuit,
    feedback_out: bool,
}

impl Register1Bit {
    pub fn power_on() -> Self {
        Self {
            child_circuit: LatchCircuit::power_on(),
            feedback_out: false, 
        }
    }

    pub fn register_1bit(&mut self, input: bool, load: bool) -> bool {
        // Visual reprensentation of 1-BIT register
        //            load
        //              0
        //              |
        // in 0 ---- +--+--+          +-----+
        //           │ MUX ┝-mux_out--+ DFF +---┬-- out 0
        //       ┌--+------+          +-----+   |
        //       │                              |
        //       └------------------------------┘
        // As you can see from the visualization, these variables
        // are in order to make connection back to MUX gate from DFF gate.

        let mux_out = mux(input, self.feedback_out, load);
        self.feedback_out = self.child_circuit.digital_flipflop(mux_out, load);

        return self.feedback_out;
    }
}
use crate::{
    emulated_parts::register_16bit_emulated::Register16BitEmulated,
    hack_computer::{
        chips::adder::inc16,
        gates::{gates_b1::or, gates_b16::mux16, gates_mw::mux4way16},
    },
};

use super::register_16bit::Register16Bit;

pub struct ProgramCounter {
    base_circuit: Register16BitEmulated,
    feedback_out: [bool; 16],
}

impl ProgramCounter {
    pub fn power_on() -> Self {
        Self {
            base_circuit: Register16BitEmulated::power_on(),
            feedback_out: [false; 16],
        }
    }
    pub fn program_counter_clocked(
        &mut self,
        input: [bool; 16],
        load: bool,  // should emit input from the register on next cycle
        inc: bool,   // should emit inc16 from the register on next cycle
        reset: bool, // should emit zero from the register on next cycle
        clock: bool,
    ) -> [bool; 16] {
        // https://stackoverflow.com/questions/15034037/trying-to-build-a-pc-counter-for-the-nand2tetris-book-but-im-having-some-tro
        let reset_out = mux16(input, [false; 16], reset);
        let load_or_reset = or(load, reset);

        let reg_in = mux16(self.feedback_out, reset_out, load_or_reset);

        let reg_load = or(load, reset);
        let reg_out = self
            .base_circuit
            .register_16bit_clocked(reg_in, reg_load, clock);

        let inc_out = inc16(reg_out);
        self.feedback_out = mux16(reg_out, inc_out, inc);

        self.feedback_out
    }

    pub fn get_debug_info(&mut self) -> [bool; 16] {
        self.base_circuit.get_debug_info()
    }
}

// TODO: Write either better tests or panel for this.
// in order to test this meaningfully, you might need to consider mocking some of the child circuits
// or just write e2e tests
pub mod test {
    use super::*;

    #[test]
    fn test_register_16bit() {
        // one test is enough, basically just to test  that the struct is initalized correctly
        let mut register = ProgramCounter::power_on();

        let input = [
            true, false, true, false, true, false, true, false, true, false, true, false, true,
            false, true, false,
        ];
        // let load = true;

        let load = false;
        let inc = false;
        let reset = false;
        // let clock = false;
        let mut clock = false;
        let mut output = register.program_counter_clocked(input, load, inc, reset, clock);
        assert_eq!(output, [false; 16], "tick tock 1");

        clock = true;
        output = register.program_counter_clocked(input, load, inc, reset, clock);

        assert_eq!(output, [false; 16], "tick tock 2");
    }
}

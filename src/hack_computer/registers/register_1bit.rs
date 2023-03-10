use crate::hack_computer::{
    chips::latch::Latch,
    gates::gates_b1::{and, not, or},
};

pub struct Register1Bit {
    child_circuit: [Latch; 2],
    pub current_value: bool,
}

impl Register1Bit {
    pub fn power_on() -> Self {
        Self {
            current_value: false,
            child_circuit: [Latch::power_on(), Latch::power_on()],
        }
    }

    pub fn reg_mux(previous_out: bool, store: bool, data: bool) -> bool {
        let and1_out = and(previous_out, not(store));
        let and2_out = and(data, store);

        or(and1_out, and2_out)
    }

    // registed is used only, when clock is active
    pub fn register_1bit_clocked(&mut self, data: bool, store: bool, clock: bool) -> bool {
        // selects old data or current data, whethe store bit is on.
        let selected_data = Self::reg_mux(self.current_value, store, data);

        // (uninuitive), use not(clock) as store indicator for the first latch
        // you could think this latch as current event,
        // that holds the bit either from input or from previous state
        let (queue_out, _) = self.child_circuit[0].d_latch(selected_data, not(clock));

        // gets the bit either from "queue" or from the clock pulse.
        let (out, _) = self.child_circuit[1].d_latch(queue_out, clock);

        out
    }
}

mod test {
    #[test]
    fn test_register_1bit_clocked() {
        let mut register = crate::hack_computer::registers::register_1bit::Register1Bit::power_on();
        println!("POWER ON");

        struct TestCase {
            data: bool,
            clock: bool,
            store: bool,
            expect: bool,
            test_name: &'static str,
        }
        // in order to understand, clock is expected to behave pulse-like
        // it's because the latches are triggered on the rising edge.
        // so the data wont't update until the clock-pulse triggers.
        let test_cases = vec![
            TestCase {
                data: false,
                clock: false,
                store: false,
                expect: false,
                test_name: "test 1 - circuit inactive",
            },
            TestCase {
                data: false,
                clock: false,
                store: true,
                expect: false,
                test_name: "test 2 - store bit activated, clock would set to false",
            },
            TestCase {
                data: false,
                clock: true,
                store: false,
                expect: false,
                // changes internal state, but does not change the output
                test_name: "test 3 - clock on, but other bits inactivated",
            },
            TestCase {
                data: false,
                clock: true,
                store: true,
                expect: false,
                test_name: "test 4 - write same data than on previous clock cycle",
            },
            TestCase {
                data: true,
                clock: false,
                store: false,
                expect: false,
                test_name: "test 5 - set data on for the next clock cycle",
            },
            TestCase {
                data: true,
                clock: false,
                store: true,
                expect: false,
                test_name: "test 6 - activate store, clock still off, no change",
            },
            TestCase {
                data: true,
                clock: true,
                store: true,
                expect: false,
                test_name: "test 7 - clock triggered",
            },
            TestCase {
                data: true,
                clock: false,
                store: true,
                expect: false,
                test_name: "test 7.1 - clock untriggered",
            },
            TestCase {
                data: true,
                clock: false,
                store: true,
                expect: false,
                // note that the state change depnds of low_q:s of other circuits
                test_name: "test 8 - TODO: Verify that is this correct",
            },
            TestCase {
                data: false,
                clock: false,
                store: false,
                expect: false,
                test_name: "test 9 - get old value",
            },
        ];

        for test in test_cases {
            println!("\n\n\n");
            println!(
                "Running test: {} - with parms data: {}, clock: {}, store: {}",
                test.test_name,
                crate::utils::convert::b2n(test.data),
                crate::utils::convert::b2n(test.clock),
                crate::utils::convert::b2n(test.store)
            );
            let res = register.register_1bit_clocked(test.data, test.store, test.clock);

            println!(
                "\nTEST: Expected {}, and got: {}.\n",
                crate::utils::convert::b2n(test.expect),
                crate::utils::convert::b2n(res)
            );
            assert_eq!(
                res,
                test.expect,
                "{}",
                format!("Test failed: {}", test.test_name)
            );
        }

        println!("\n\n\n");
    }
}

use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    pub struct OutputChoice {
        pub choice: bool, // Player's choice: true for heads, false for tails
    }

    #[instruction]
    pub fn rand_output(input_ctxt: Enc<Shared, OutputChoice>) -> bool {
        let input = input_ctxt.to_arcis();
        let toss = ArcisRNG::bool();
        (input.choice == toss).reveal()
    }
}

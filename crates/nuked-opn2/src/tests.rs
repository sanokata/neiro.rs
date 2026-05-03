mod tests {
    use crate::chip::Ym3438;
    use crate::ffi::*;

    #[test]
    fn test_c_reference_initialization() {
        unsafe {
            // Initialize chip state with zeros
            let mut chip = Ym3438::default();

            // Call C implementation of OPN2_Reset
            OPN2_Reset(&mut chip);

            // Verify that the initial cycles count is 0
            assert_eq!(chip.cycles, 0);

            // Verify that some initial values are set correctly
            // Nuked-OPN2 sets pan_l/r to 1 by default
            assert_eq!(chip.pan_l[0], 1);
            assert_eq!(chip.pan_r[0], 1);
        }
    }

    #[test]
    fn test_c_reference_clock_step() {
        unsafe {
            let mut chip = Ym3438::default();
            OPN2_Reset(&mut chip);

            // Output buffer for 1 sample (L/R)
            let mut buffer = [0i16; 2];

            // Step the emulator by 1 clock cycle using C implementation
            OPN2_Clock(&mut chip, buffer.as_mut_ptr());

            // Nuked-OPN2 increments cycles counter on each clock
            assert_eq!(chip.cycles, 1);

            // Print values for visual confirmation
            println!(
                "C Reference step - Cycles: {}, Output: [{}, {}]",
                chip.cycles, buffer[0], buffer[1]
            );
        }
    }
}

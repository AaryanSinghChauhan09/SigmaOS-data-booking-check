//! Integration tests for SigmaOS-data-booking-check.

use sigmaos_data_booking_check_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}

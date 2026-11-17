// Integration test for SigmaOS-gsettings-desktop-schemas
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_gsettings_desktop_schemas as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}

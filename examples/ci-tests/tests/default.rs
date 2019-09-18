#![allow(clippy::cognitive_complexity)]

use molecule::prelude::*;

use molecule_ci_tests::testset;

macro_rules! verify_default {
    ($type:ident) => {
        let default = $type::default();
        let result = $type::from_slice(default.as_slice());
        assert!(
            result.is_ok(),
            "failed to verify default for {}",
            $type::NAME
        );
    };
}

#[test]
fn default_can_verify() {
    testset!(all, verify_default);
}

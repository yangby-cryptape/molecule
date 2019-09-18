#![allow(clippy::cognitive_complexity)]

use molecule::prelude::*;

use molecule_ci_tests::types;

macro_rules! compatible_table_test {
    (strict, $type:ident, [$( !$insufficient:ident, )* *$exact:ident $( ,!$redundant:ident)*]) => {
        $( compatible_table_test!(from_slice, $type, $insufficient, is_err); )*
        compatible_table_test!(from_slice, $type, $exact, is_ok);
        $( compatible_table_test!(from_slice, $type, $redundant, is_err); )*
    };
    (compatible, $type:ident, [$( $bad:ident, )* () $( ,$good:ident )*]) => {
        $( compatible_table_test!(from_compatible_slice, $type, $bad, is_err); )*
        let mut cnt = 0;
        $(
            compatible_table_test!(from_compatible_slice, $type, $good, is_ok);
            compatible_table_test!(extra_fields, $type, $good, cnt);
            cnt += 1;
        )*
        let _ = cnt;
    };
    (extra_fields, $type:ident, $slice:ident, $expected:ident) => {
        let t = $type::from_compatible_slice($slice).unwrap();
        let result = t.count_extra_fields();
        assert_eq!(
            result,
            $expected,
            "failed at (count_extra_fields, {}, {}): actual {}, expect {}",
            stringify!($type),
            stringify!($slice),
            result,
            $expected,
        );
        assert_eq!(
            t.has_extra_fields(),
            $expected != 0,
            "failed at (has_extra_fields, {}, {}, {})",
            stringify!($type),
            stringify!($slice),
            $expected,
        );
    };
    ($func:ident, $type:ident, $slice:ident, $expected:ident) => {
        let result = $type::$func($slice);
        assert!(
            result.$expected(),
            "failed at ({}, {}, {}, {}): {}",
            stringify!($func),
            stringify!($type),
            stringify!($slice),
            stringify!($expected),
            result.unwrap_err(),
        );
    };
}

#[test]
fn compatible_table() {
    use types::{
        Table0 as T0, Table1 as T1, Table2 as T2, Table3 as T3, Table4 as T4, Table5 as T5,
        Table6 as T6,
    };

    let t0_entity = T0::default();
    let t1_entity = T1::default();
    let t2_entity = T2::default();
    let t3_entity = T3::default();
    let t4_entity = T4::default();
    let t5_entity = T5::default();
    let t6_entity = T6::default();

    let t0 = t0_entity.as_slice();
    let t1 = t1_entity.as_slice();
    let t2 = t2_entity.as_slice();
    let t3 = t3_entity.as_slice();
    let t4 = t4_entity.as_slice();
    let t5 = t5_entity.as_slice();
    let t6 = t6_entity.as_slice();

    compatible_table_test!(strict, T0, [*t0, !t1, !t2, !t3, !t4, !t5, !t6]);
    compatible_table_test!(strict, T1, [!t0, *t1, !t2, !t3, !t4, !t5, !t6]);
    compatible_table_test!(strict, T2, [!t0, !t1, *t2, !t3, !t4, !t5, !t6]);
    compatible_table_test!(strict, T3, [!t0, !t1, !t2, *t3, !t4, !t5, !t6]);
    compatible_table_test!(strict, T4, [!t0, !t1, !t2, !t3, *t4, !t5, !t6]);
    compatible_table_test!(strict, T5, [!t0, !t1, !t2, !t3, !t4, *t5, !t6]);
    compatible_table_test!(strict, T6, [!t0, !t1, !t2, !t3, !t4, !t5, *t6]);

    compatible_table_test!(compatible, T0, [(), t0, t1, t2, t3, t4, t5, t6]);
    compatible_table_test!(compatible, T1, [t0, (), t1, t2, t3, t4, t5, t6]);
    compatible_table_test!(compatible, T2, [t0, t1, (), t2, t3, t4, t5, t6]);
    compatible_table_test!(compatible, T3, [t0, t1, t2, (), t3, t4, t5, t6]);
    compatible_table_test!(compatible, T4, [t0, t1, t2, t3, (), t4, t5, t6]);
    compatible_table_test!(compatible, T5, [t0, t1, t2, t3, t4, (), t5, t6]);
    compatible_table_test!(compatible, T6, [t0, t1, t2, t3, t4, t5, (), t6]);
}

#![allow(clippy::cognitive_complexity)]

use molecule::prelude::*;
use slices::u8_slice as s;

use molecule_ci_tests::types;

macro_rules! test_default {
    ($type:ident, $expected:expr) => {
        let result = types::$type::default();
        assert_eq!(
            result.as_slice(),
            &$expected[..],
            "failed to test {} default",
            stringify!($type)
        );
        assert!(
            types::$type::from_slice(result.as_slice()).is_ok(),
            "failed to verify {} default",
            stringify!($type)
        );
    };
}

macro_rules! test_option_set_default {
    ($type:ident, $type_inner:ident) => {
        let result = types::$type::new_builder()
            .set(Some(Default::default()))
            .build();
        let expected = types::$type_inner::default();
        assert_eq!(
            result.as_slice(),
            expected.as_slice(),
            "failed to test {} with {}",
            stringify!($type),
            stringify!($type_inner),
        );
    };
}

macro_rules! test_vector_push_default {
    ($type:ident, $expected1:expr, $expected2:expr, $expected3:expr) => {
        let t = types::$type::default();
        let t = test_vector_push_default!($type, t, $expected1);
        let t = test_vector_push_default!($type, t, $expected2);
        let _ = test_vector_push_default!($type, t, $expected3);
    };
    ($type:ident, $input:ident, $expected:expr) => {{
        let result = $input.as_builder().push(Default::default()).build();
        let expected = $expected;
        assert_eq!(
            result.as_slice(),
            &expected[..],
            "failed to test {} with {} items",
            stringify!($type),
            result.len(),
        );
        assert!(
            types::$type::from_slice(result.as_slice()).is_ok(),
            "failed to verify {} with {} items",
            stringify!($type),
            result.len(),
        );
        result
    }};
}

#[test]
fn option_default() {
    let slice = s!("0x");
    test_default!(ByteOpt, slice);
    test_default!(WordOpt, slice);
    test_default!(StructAOpt, slice);
    test_default!(StructPOpt, slice);
    test_default!(BytesOpt, slice);
    test_default!(WordsOpt, slice);
    test_default!(BytesVecOpt, slice);
    test_default!(WordsVecOpt, slice);
    test_default!(Table0Opt, slice);
    test_default!(Table6Opt, slice);
    test_default!(Table6OptOpt, slice);
}

#[test]
fn union_default() {
    test_default!(
        UnionA,
        s!("0x\
            09000000\
            00000000\
            00\
            ")
    );
}

#[test]
fn array_default() {
    test_default!(Byte2, s!("0x0000"));
    test_default!(Byte3, s!("0x000000"));
    test_default!(Byte4, s!("0x00000000"));
    test_default!(Byte5, s!("0x00000000_00"));
    test_default!(Byte6, s!("0x00000000_0000"));
    test_default!(Byte7, s!("0x00000000_000000"));
    test_default!(Byte8, s!("0x00000000_00000000"));
    test_default!(Byte9, s!("0x00000000_00000000_00"));
    test_default!(Byte10, s!("0x00000000_00000000_0000"));
    test_default!(Byte11, s!("0x00000000_00000000_000000"));
    test_default!(Byte12, s!("0x00000000_00000000_00000000"));
    test_default!(Byte13, s!("0x00000000_00000000_00000000_00"));
    test_default!(Byte14, s!("0x00000000_00000000_00000000_0000"));
    test_default!(Byte15, s!("0x00000000_00000000_00000000_000000"));
    test_default!(Byte16, s!("0x00000000_00000000_00000000_00000000"));

    test_default!(Word, s!("0x0000"));
    test_default!(Word2, s!("0x00000000"));
    test_default!(Word3, s!("0x00000000_0000"));
    test_default!(Word4, s!("0x00000000_00000000"));
    test_default!(Word5, s!("0x00000000_00000000_0000"));
    test_default!(Word6, s!("0x00000000_00000000_00000000"));
    test_default!(Word7, s!("0x00000000_00000000_00000000_0000"));
    test_default!(Word8, s!("0x00000000_00000000_00000000_00000000"));

    test_default!(
        Byte3x3,
        s!("0x\
            000000____00\
            000000____00\
            000000\
            ")
    );
    test_default!(
        Byte5x3,
        s!("0x\
            00000000_00____000000\
            00000000_00____000000\
            00000000_00\
            ")
    );
    test_default!(
        Byte7x3,
        s!("0x\
            00000000_000000____00\
            00000000_000000____00\
            00000000_000000\
            ")
    );
    test_default!(
        Byte9x3,
        s!("0x\
            00000000_00000000_00____00000000_000000\
            00000000_00000000_00____00000000_000000\
            00000000_00000000_00\
            ")
    );
    test_default!(
        StructIx3,
        s!("0x\
            000000____00\
            000000____00\
            000000____00\
            ")
    );
}

#[test]
fn struct_default() {
    test_default!(
        StructA,
        s!("0x\
            00\
            00\
            0000\
            0000\
            ")
    );
    test_default!(
        StructB,
        s!("0x\
            00\
            00\
            0000\
            000000\
            ")
    );
    test_default!(
        StructC,
        s!("0x\
            00\
            00\
            0000\
            00000000\
            ")
    );
    test_default!(
        StructD,
        s!("0x\
            00\
            00\
            0000____00000000\
            00000000_00\
            ")
    );
    test_default!(
        StructE,
        s!("0x\
            00____00\
            0000\
            00____00\
            0000\
            ")
    );
    test_default!(
        StructF,
        s!("0x\
            00____000000\
            000000\
            00\
            ")
    );
    test_default!(
        StructG,
        s!("0x\
            000000\
            00\
            0000\
            00000000\
            ")
    );
    test_default!(
        StructH,
        s!("0x\
            000000\
            00\
            0000____0000\
            00000000\
            ")
    );
    test_default!(
        StructI,
        s!("0x\
            000000\
            00\
            ")
    );
    test_default!(
        StructJ,
        s!("0x\
            00000000_0000\
            00\
            ")
    );
    test_default!(
        StructO,
        s!("0x\
            00000000_00000000_00000000\
            00\
            ")
    );
    test_default!(
        StructP,
        s!("0x\
            00000000_000000\
            00\
            ")
    );
}

#[test]
fn fixvec_default() {
    let slice = s!("0x00000000");
    test_default!(Bytes, slice);
    test_default!(Words, slice);
    test_default!(Byte3Vec, slice);
    test_default!(Byte7Vec, slice);
    test_default!(StructJVec, slice);
    test_default!(StructPVec, slice);
}

#[test]
fn dynvec_default() {
    let slice = s!("0x08000000_00000000");
    test_default!(BytesVec, slice);
    test_default!(WordsVec, slice);
    test_default!(ByteOptVec, slice);
    test_default!(WordOptVec, slice);
    test_default!(WordsOptVec, slice);
    test_default!(BytesOptVec, slice);
}

#[test]
fn table_default() {
    test_default!(
        Table0,
        s!("0x\
            08000000_00000000\
            ")
    );
    test_default!(
        Table1,
        s!("0x\
            0d000000_01000000\
            \
            0c000000\
            \
            00\
            ")
    );
    test_default!(
        Table2,
        s!("0x\
            16000000_02000000\
            \
            10000000\
            12000000\
            \
            00____00\
            00000000\
            ")
    );
    test_default!(
        Table3,
        s!("0x\
            20000000_03000000\
            \
            14000000\
            16000000\
            1a000000\
            \
            00____00\
            00000000\
            00000000_0000\
            ")
    );
    test_default!(
        Table4,
        s!("0x\
            28000000_04000000\
            \
            18000000\
            1a000000\
            1e000000\
            24000000\
            \
            00____00\
            00000000\
            00000000_0000\
            00000000\
            ")
    );
    test_default!(
        Table5,
        s!("0x\
            34000000_05000000\
            \
            1c000000\
            1e000000\
            22000000\
            28000000\
            2c000000\
            \
            00____00\
            00000000\
            00000000_0000\
            00000000\
            08000000_00000000\
            ")
    );
    test_default!(
        Table6,
        s!("0x\
            6c000000_06000000\
            \
            20000000\
            22000000\
            26000000\
            2c000000\
            30000000\
            38000000\
            \
            00____00\
            00000000\
            00000000_0000\
            00000000\
            08000000_00000000\
            \
            34000000_05000000\
            1c000000_1e000000_22000000_28000000_2c000000\
            00____00\
            00000000\
            00000000_0000\
            00000000\
            08000000_00000000\
            ")
    );
}

#[test]
fn option_set_default() {
    test_option_set_default!(ByteOpt, Byte);
    test_option_set_default!(WordOpt, Word);
    test_option_set_default!(StructAOpt, StructA);
    test_option_set_default!(StructPOpt, StructP);
    test_option_set_default!(BytesOpt, Bytes);
    test_option_set_default!(WordsOpt, Words);
    test_option_set_default!(BytesVecOpt, BytesVec);
    test_option_set_default!(WordsVecOpt, WordsVec);
    test_option_set_default!(Table0Opt, Table0);
    test_option_set_default!(Table6Opt, Table6);
    test_option_set_default!(Table6OptOpt, Table6Opt);
}

#[test]
fn fixvec_push_default() {
    test_vector_push_default!(
        Bytes,
        s!("0x\
            01000000\
            00\
            "),
        s!("0x\
            02000000\
            00\
            00\
            "),
        s!("0x\
            03000000\
            00\
            00\
            00\
            ")
    );
    test_vector_push_default!(
        Words,
        s!("0x\
            01000000\
            0000\
            "),
        s!("0x\
            02000000\
            0000\
            0000\
            "),
        s!("0x\
            03000000\
            0000\
            0000\
            0000\
            ")
    );
    test_vector_push_default!(
        Byte3Vec,
        s!("0x\
            01000000\
            000000\
            "),
        s!("0x\
            02000000\
            000000____00\
            000000\
            "),
        s!("0x\
            03000000\
            000000____00\
            000000____00\
            000000\
            ")
    );
    test_vector_push_default!(
        Byte7Vec,
        s!("0x\
            01000000____00000000\
            00000000_000000\
            "),
        s!("0x\
            02000000____00000000\
            00000000_000000____00\
            00000000_000000\
            "),
        s!("0x\
            03000000____00000000\
            00000000_000000____00\
            00000000_000000____00\
            00000000_000000\
            ")
    );
    test_vector_push_default!(
        StructIVec,
        s!("0x\
            01000000\
            00000000\
            "),
        s!("0x\
            02000000\
            00000000\
            00000000\
            "),
        s!("0x\
            03000000\
            00000000\
            00000000\
            00000000\
            ")
    );
    test_vector_push_default!(
        StructJVec,
        s!("0x\
            01000000____00000000\
            00000000_000000\
            "),
        s!("0x\
            02000000____00000000\
            00000000_000000____00\
            00000000_000000\
            "),
        s!("0x\
            03000000____00000000\
            00000000_000000____00\
            00000000_000000____00\
            00000000_000000\
            ")
    );
    test_vector_push_default!(
        StructPVec,
        s!("0x\
            01000000____00000000\
            00000000_00000000\
            "),
        s!("0x\
            02000000____00000000\
            00000000_00000000\
            00000000_00000000\
            "),
        s!("0x\
            03000000____00000000\
            00000000_00000000\
            00000000_00000000\
            00000000_00000000\
            ")
    );
}

#[test]
fn dynvec_push_default() {
    test_vector_push_default!(
        BytesVec,
        s!("0x\
            10000000_01000000\
            \
            0c000000\
            \
            00000000\
            "),
        s!("0x\
            18000000_02000000\
            \
            10000000\
            14000000\
            \
            00000000\
            00000000\
            "),
        s!("0x\
            20000000_03000000\
            \
            14000000\
            18000000\
            1c000000\
            \
            00000000\
            00000000\
            00000000\
            ")
    );
    test_vector_push_default!(
        WordsVec,
        s!("0x\
            10000000_01000000\
            \
            0c000000\
            \
            00000000\
            "),
        s!("0x\
            18000000_02000000\
            \
            10000000\
            14000000\
            \
            00000000\
            00000000\
            "),
        s!("0x\
            20000000_03000000\
            \
            14000000\
            18000000\
            1c000000\
            \
            00000000\
            00000000\
            00000000\
            ")
    );
    let s1 = s!("0x\
                 0c000000_01000000\
                 \
                 0c000000\
                 ");
    let s2 = s!("0x\
                 10000000_02000000\
                 \
                 10000000\
                 10000000\
                 ");
    let s3 = s!("0x\
                 14000000_03000000\
                 \
                 14000000\
                 14000000\
                 14000000\
                 ");
    test_vector_push_default!(ByteOptVec, s1, s2, s3);
    test_vector_push_default!(WordOptVec, s1, s2, s3);
    test_vector_push_default!(WordsOptVec, s1, s2, s3);
    test_vector_push_default!(BytesOptVec, s1, s2, s3);
}

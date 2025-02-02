#![feature(test)]

extern crate test;

use anymap2::AnyMap;

use test::{black_box, Bencher};

#[bench]
fn insertion(b: &mut Bencher) {
    b.iter(|| {
        let mut data = AnyMap::new();
        for _ in 0..100 {
            let _ = data.insert(42);
        }
    })
}

#[bench]
fn get_missing(b: &mut Bencher) {
    b.iter(|| {
        let data = AnyMap::new();
        for _ in 0..100 {
            assert_eq!(data.get(), None::<&i32>);
        }
    })
}

#[bench]
fn get_present(b: &mut Bencher) {
    b.iter(|| {
        let mut data = AnyMap::new();
        let _ = data.insert(42);
        // These inner loops are a feeble attempt to drown the other factors.
        for _ in 0..100 {
            assert_eq!(data.get(), Some(&42));
        }
    })
}

macro_rules! big_benchmarks {
    ($name:ident, $($T:ident)*) => (
        #[bench]
        fn $name(b: &mut Bencher) {
            $(
                struct $T(&'static str);
            )*

            b.iter(|| {
                let mut data = AnyMap::new();
                $(
                    let _ = black_box(data.insert($T(stringify!($T))));
                )*
                $(
                    let _ = black_box(data.get::<$T>());
                )*
            })
        }
    );
}

// Caution: if the macro does too much (e.g. assertions) this goes from being
// slow to being *really* slow (like add a minute for each assertion on it) and
// memory-hungry (like, adding several hundred megabytes to the peak for each
// assertion).
big_benchmarks! {
    insert_and_get_on_260_types,
    A0 B0 C0 D0 E0 F0 G0 H0 I0 J0 K0 L0 M0 N0 O0 P0 Q0 R0 S0 T0 U0 V0 W0 X0 Y0 Z0
    A1 B1 C1 D1 E1 F1 G1 H1 I1 J1 K1 L1 M1 N1 O1 P1 Q1 R1 S1 T1 U1 V1 W1 X1 Y1 Z1
    A2 B2 C2 D2 E2 F2 G2 H2 I2 J2 K2 L2 M2 N2 O2 P2 Q2 R2 S2 T2 U2 V2 W2 X2 Y2 Z2
    A3 B3 C3 D3 E3 F3 G3 H3 I3 J3 K3 L3 M3 N3 O3 P3 Q3 R3 S3 T3 U3 V3 W3 X3 Y3 Z3
    A4 B4 C4 D4 E4 F4 G4 H4 I4 J4 K4 L4 M4 N4 O4 P4 Q4 R4 S4 T4 U4 V4 W4 X4 Y4 Z4
    A5 B5 C5 D5 E5 F5 G5 H5 I5 J5 K5 L5 M5 N5 O5 P5 Q5 R5 S5 T5 U5 V5 W5 X5 Y5 Z5
    A6 B6 C6 D6 E6 F6 G6 H6 I6 J6 K6 L6 M6 N6 O6 P6 Q6 R6 S6 T6 U6 V6 W6 X6 Y6 Z6
    A7 B7 C7 D7 E7 F7 G7 H7 I7 J7 K7 L7 M7 N7 O7 P7 Q7 R7 S7 T7 U7 V7 W7 X7 Y7 Z7
    A8 B8 C8 D8 E8 F8 G8 H8 I8 J8 K8 L8 M8 N8 O8 P8 Q8 R8 S8 T8 U8 V8 W8 X8 Y8 Z8
    A9 B9 C9 D9 E9 F9 G9 H9 I9 J9 K9 L9 M9 N9 O9 P9 Q9 R9 S9 T9 U9 V9 W9 X9 Y9 Z9
}

big_benchmarks! {
    insert_and_get_on_26_types,
    A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
}

use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ba

fn main() {
    input! {
        n: i64,
    };

    let s: Vec<i64> = vec![
        1,
        3,
        7,
        15,
        31,
        63,
        127,
        255,
        511,
        1023,
        2047,
        4095,
        8191,
        16383,
        32767,
        65535,
        131071,
        262143,
        524287,
        1048575,
        2097151,
        4194303,
        8388607,
        16777215,
        33554431,
        67108863,
        134217727,
        268435455,
        536870911,
        1073741823,
        2147483647,
        4294967295,
        8589934591,
        17179869183,
        34359738367,
        68719476735,
        137438953471,
        274877906943,
        549755813887,
        1099511627775,
        2199023255551,
        4398046511103,
        8796093022207,
        17592186044415,
        35184372088831,
        70368744177663,
        140737488355327,
        281474976710655,
        562949953421311,
        1125899906842623,
        2251799813685247,
        4503599627370495,
        9007199254740991,
        18014398509481983,
        36028797018963967,
        72057594037927935,
        144115188075855871,
        288230376151711743,
        576460752303423487,
        1152921504606846975,
        2305843009213693951,
        4611686018427387903,
    ];

    if s.contains(&n) {
        println!("Second");
    } else {
        println!("First");
    }
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c061";

    #[test]
    fn sample_c061_1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"2
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "First\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c061_2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"3
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "Second\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample_c061_3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"1000000000000000000
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "First\n");
        assert!(output.stderr_str().is_empty());
    }
}

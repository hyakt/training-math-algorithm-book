use proconio::input;

fn main() {
    input! {
        n: usize
    }
    println!("{}", n + 5)
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;

    const BIN: &'static str = "./c001";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin("2")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "7\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin("4")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "9\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin("8")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "13\n");
        assert!(output.stderr_str().is_empty());
    }
}

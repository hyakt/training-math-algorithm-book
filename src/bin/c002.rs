use proconio::input;

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize
    }
    println!("{}", a1 + a2 + a3)
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;

    const BIN: &'static str = "./c002";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin("10 20 50")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "80\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin("1 1 1")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "3\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin("100 100 100")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "300\n");
        assert!(output.stderr_str().is_empty());
    }
}

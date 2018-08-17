extern crate assert_cli;

#[cfg(test)]
mod cli {
    use assert_cli;

    #[test]
    fn invalid_argument() {
        assert_cli::Assert::main_binary()
            .with_args(&["-Z"])
            .fails()
            .and()
            .stderr()
            .contains("error: Found argument '-Z' which wasn't expected")
            .unwrap();
    }

    #[test]
    fn help_works() {
        assert_cli::Assert::main_binary()
            .with_args(&["-h"])
            .succeeds()
            .and()
            .stdout()
            .contains("tracker")
            .unwrap();
    }

    #[test]
    fn start_timer() {
        assert_cli::Assert::main_binary()
            .with_args(&["start"])
            .succeeds()
            .and()
            .stdout()
            .contains("Starting")
            .unwrap();
    }

    #[test]
    fn start_second_timer() {
        assert_cli::Assert::main_binary()
            .with_args(&["start"])
            .succeeds()
            .and()
            .stdout()
            .contains("running")
            .unwrap();
    }

    #[test]
    fn status_timer() {
        assert_cli::Assert::main_binary()
            .with_args(&["status"])
            .succeeds()
            .and()
            .stdout()
            .contains("running")
            .unwrap();
    }

    #[test]
    fn stop_timer() {
        assert_cli::Assert::main_binary()
            .with_args(&["stop"])
            .succeeds()
            .and()
            .stdout()
            .contains("Stopping")
            .unwrap();
    }

    #[test]
    fn print_hours() {
        assert_cli::Assert::main_binary()
            .with_args(&["hours"])
            .succeeds()
            .and()
            .stdout()
            .contains("Total")
            .unwrap();
    }

    #[test]
    fn print_weeks() {
        assert_cli::Assert::main_binary()
            .with_args(&["weeks"])
            .succeeds()
            .and()
            .stdout()
            .contains("Week")
            .unwrap();
    }

    #[test]
    fn print_days() {
        assert_cli::Assert::main_binary()
            .with_args(&["days"])
            .succeeds()
            .and()
            .stdout()
            .contains("Day")
            .unwrap();
    }

}

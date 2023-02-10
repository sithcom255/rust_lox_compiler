pub(crate) struct Log {}

impl Log {

    pub fn error(line: usize, message: &str) {
        Log::report("error", line, message);
    }

    fn report(importance: &str, line: usize, message: &str) {
        println!("[ Line {} ] {} : {}", line, importance, message);
    }

}
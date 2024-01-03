use regex::Regex;

#[derive(Clone)]
pub(super) struct Test {
    divisor: i32,
    true_branch: i32,
    false_branch: i32
}
impl Test {
    pub fn from(test_unparsed: Vec<&str>) -> Test {
        let re = Regex::new("\\d+").unwrap();
        return Test {
            divisor: re.find(test_unparsed[0]).unwrap().as_str().parse::<i32>().unwrap(),
            true_branch: re.find(test_unparsed[1]).unwrap().as_str().parse::<i32>().unwrap(),
            false_branch: re.find(test_unparsed[2]).unwrap().as_str().parse::<i32>().unwrap()
        };
    }

    pub fn apply(&self, value: &i32) -> i32 {
        return if value % self.divisor == 0 {
            self.true_branch
        } else {
            self.false_branch
        };
    }
}
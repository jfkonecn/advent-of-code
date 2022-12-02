pub fn solution_1(file_contents: String) -> i64 {
    file_contents
        .split("\n\n")
        .map(|str| {
            str.split('\n')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .max()
        .unwrap()
}

pub fn solution_2(file_contents: String) -> i64 {
    let mut vec = file_contents
        .split("\n\n")
        .map(|str| {
            str.split('\n')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .collect::<Vec<i64>>();
    vec.sort_by(|a, b| b.cmp(a));
    vec.truncate(3);
    vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    enum FileType {
        Example(i8),
        Real(i8),
    }

    fn make_file_path(file_type: FileType) -> String {
        let mut current_dir = std::env::current_dir().unwrap();
        current_dir.push("src");
        current_dir.push("year_2022");
        current_dir.push("day_1");

        match file_type {
            FileType::Example(x) => {
                current_dir.push("example_inputs");
                current_dir.push(format!("part_{}.txt", x));
            }
            FileType::Real(x) => {
                current_dir.push("real_inputs");
                current_dir.push(format!("part_{}.txt", x));
            }
        };
        let path = current_dir.display().to_string();
        fs::read_to_string(path).unwrap()
    }
    #[test]
    fn example_1_should_work() {
        let contents = make_file_path(FileType::Example(1));
        let result = solution_1(contents);
        assert_eq!(result, 24000);
    }

    #[test]
    fn real_1_should_work() {
        let contents = make_file_path(FileType::Real(1));
        let result = solution_1(contents);
        assert_eq!(result, 69206);
    }

    #[test]
    fn example_2_should_work() {
        let contents = make_file_path(FileType::Example(1));
        let result = solution_2(contents);
        assert_eq!(result, 45000);
    }

    #[test]
    fn real_2_should_work() {
        let contents = make_file_path(FileType::Real(1));
        let result = solution_2(contents);
        assert_eq!(result, 197400);
    }
}

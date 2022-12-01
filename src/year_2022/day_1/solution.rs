pub fn solution(file_name: String) -> Result<i64, String> {
    println!("{}", std::env::current_dir().unwrap().display());
    Ok(1)
}

#[cfg(test)]
mod tests {
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
        current_dir.display().to_string()
    }
    #[test]
    fn example_should_work() {
        let path = make_file_path(FileType::Example(1));
        let result = solution(path);
        if let Ok(x) = result {
            assert_eq!(x, 24000);
        } else if let Err(x) = result {
            panic!("{}", x);
        }
    }
}

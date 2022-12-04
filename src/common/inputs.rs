
macro_rules! challenge_test {
    ($test_name: literal, 
        $function:expr, 
        $expected:literal, 
        $(,$path: literal)*
    ) => {

    }
}

// https://stackoverflow.com/questions/32817193/how-to-get-index-of-macro-repetition-single-element
macro_rules! challenge_test_suite {
    // @ denotes an internal rule 
    // https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
    // https://veykril.github.io/tlborm/decl-macros/patterns/internal-rules.html
    /*
    To create an internal rule, add the rule name starting with @ as
    the argument. Now the macro will never match for an internal 
    rule until explicitly specified as an argument.
    */
    ( 
       @step $idx:expr, 
    ) => {

    };
    ( 
       @step $idx:expr, $solution_fun:ident, 
        $expected_example:literal, 
        $expected_real:literal
       $(,$solution_tail_fun:ident 
        ,$expected_tail_example:literal 
        ,$expected_tail_real:literal)*,
    ) => {
        paste! {
        #[test]
        fn [<example_ $idx _should_work>]() {
            let contents = make_file_path(FileType::Example);
            let result = $solution_fun(contents);
            assert_eq!(result, $expected_example);
        }

        #[test]
        fn [<real_ $idx _should_work>]() {
            let contents = make_file_path(FileType::Real);
            let result = $solution_fun(contents);
            assert_eq!(result, $expected_real);
        }
        }
// can't do $idx + 1 outside a function :'(
challenge_test_suite!(@step 2, 
$($solution_tail_fun
        ,$expected_tail_example
        ,$expected_tail_real,)*
);
    };
    ( 
       $($solution_fun:ident 
        ,$expected_example:literal 
        ,$expected_real:literal,)+
        $($path: literal),+
    ) => {
        #[cfg(test)]
        mod tests {
            use std::fs;
            use paste::paste;
            use crate::common::inputs::*;
            use super::*;

            enum FileType {
                Example,
                Real,
            }
            fn make_file_path(file_type: FileType) -> String {
                let mut current_dir = std::env::current_dir().unwrap();
                $(
                    current_dir.push($path);
                )+

                match file_type {
                    FileType::Example => {
                        current_dir.push("example_inputs");
                        current_dir.push("input.txt");
                    }
                    FileType::Real => {
                        current_dir.push("real_inputs");
                        current_dir.push("input.txt");
                    }
                };
                let path = current_dir.display().to_string();
                fs::read_to_string(path).unwrap()
            }

challenge_test_suite!(@step 1, 
$($solution_fun
        ,$expected_example
        ,$expected_real,)+
);

        }
    };
    // ($solution_1_fun:expr, 
    //     $expected_1_example:literal, 
    //     $expected_1_real:literal, 
    //     $solution_2_fun:expr, 
    //     $expected_2_example:literal, 
    //     $expected_2_real:literal
    //     $(,$path: literal)+
    // ) => {

    // };
}

pub(crate) use challenge_test_suite;

/*
create at `2019-09-16` by `itachy`
*/

pub mod errors {
    error_chain! {
        foreign_links {
            ReqestError(reqwest::Error);
            RegexError(regex::Error);
        }
        errors {
            IPV4NotFoundError
            ResponseWithErrorStatus(code: String) {
                description("response status code isn't equal 200"),
                display("server response error with status code: {}", code),
            }
        }
    }
}
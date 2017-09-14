use std;

error_chain! {
    foreign_links {
        Io(std::io::Error)          #[doc = "Error during IO"];
    }

    errors {
        // own errors here
    }
}

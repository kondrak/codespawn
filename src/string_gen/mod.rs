use fmt_code::{FormattedCode, Lang};
mod cpp;
mod rust;
use self::cpp::convert as convert_cpp;
use self::rust::convert as convert_rust;

pub fn code_to_str(fmt_code: &FormattedCode) -> String {
    match fmt_code.language {
        Lang::Cpp  => convert_cpp(&fmt_code.elements),
        Lang::Rust => convert_rust(&fmt_code.elements),
    }
}

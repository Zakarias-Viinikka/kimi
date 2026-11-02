use p::file1;
use p::file2;
fn main() {
    let s = file1::Hello {
        text: "bla".to_string(),
    };
    file2::do_something(s);
}

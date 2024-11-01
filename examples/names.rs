use maud::html;
use maud_vs_minijinja::test_maud_vs_minijinja;
use minijinja::{context, Environment};

const NAMES: [&str; 2] = ["John", "Peter"];

fn main() {
    println!("{}", maud());
    println!("{}", minijinja());
    test_maud_vs_minijinja(maud, minijinja);
}

fn minijinja() -> String {
    let mut env = Environment::new();
    env.add_template("hello.txt", include_str!("names.html"))
        .unwrap();
    let tmpl = env.get_template("hello.txt").unwrap();
    tmpl.render(context!(names => NAMES)).unwrap()
}

fn maud() -> String {
    html! {
        @for (idx, name) in NAMES.iter().enumerate() {
            span {
                (idx + 1)". Hello "(name)"!"
            }
        }
    }
    .into_string()
}

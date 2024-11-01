use maud::html;
use maud_vs_minijinja::test_maud_vs_minijinja;
use minijinja::{context, Environment};

const NAME: &'static str = "Amy";

fn main() {
    println!("{}", maud());
    println!("{}", minijinja());
    test_maud_vs_minijinja(maud, minijinja);
}

fn minijinja() -> String {
    let mut env = Environment::new();
    env.add_template("hello.txt", "<span>Hello {{ name }}!</span>")
        .unwrap();
    let tmpl = env.get_template("hello.txt").unwrap();
    tmpl.render(context!(name => NAME)).unwrap()
}

fn maud() -> String {
    html! {
        span {
            "Hello "(NAME)"!"
        }
    }
    .into_string()
}

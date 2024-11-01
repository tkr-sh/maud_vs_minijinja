use maud_vs_minijinja::test_maud_vs_minijinja;
use minijinja::{context, Environment};

fn main() {
    test_maud_vs_minijinja(maud, minijinja);
}

fn minijinja() -> String {
    let mut env = Environment::new();
    env.add_template("index.html", include_str!("./raw_html.html"))
        .unwrap();
    let template = env.get_template("index.html").unwrap();
    template.render(context! {}).unwrap()
}

fn maud() -> String {
    maud::html! {
        html {
            head {
                title {"77 Line HTML Example"}
            }
            body {
                h1 {"Welcome to the 77 Line HTML Example"}
                p {"This is a basic HTML page with 100 lines of code."}
                h2 {"Section 1"}
                p {"This is the first section of the page."}
                ul {
                    li {"Item 1"}
                    li {"Item 2"}
                    li {"Item 3"}
                }
                h2 {"Section 2"}
                p {"This is the second section of the page."}
                ol {
                    li {"Item 1"}
                    li {"Item 2"}
                    li {"Item 3"}
                }
                h2 {"Section 3"}
                p {"This is the third section of the page."}
                table {
                    tr {
                        th {"Name"}
                        th {"Age"}
                        th {"Gender"}
                    }
                    tr {
                        td {"John"}
                        td {"25"}
                        td {"Male"}
                    }
                    tr {
                        td {"Jane"}
                        td {"30"}
                        td {"Female"}
                    }
                    tr {
                        td {"Bob"}
                        td {"35"}
                        td {"Male"}
                    }
                }
                h2 {"Section 4"}
                p {"This is the fourth section of the page."}
                img src="example.jpg" alt="Example Image";
                h2 {"Section 5"}
                p {"This is the fifth section of the page."}
                a href="https://www.example.com" {"Example Link"}
                h2 {"Section 6"}
                p {"This is the sixth section of the page."}
                form {
                    label for="name" {"Name:"}
                    input type="text" id="name" name="name";br;
                    label for="email" {"Email:"}
                    input type="email" id="email" name="email";br;
                    label for="message" {"Message:"}
                    textarea id="message" name="message" {}; br;
                    input type="submit" value="Submit";
                }
                h2 {"Section 7"}
                p {"This is the seventh section of the page."}
                div {
                    p {"This is a div element."}
                }
                h2 {"Section 8"}
                p {"This is the eighth section of the page."}
                span {"This is a span element."}
                h2 {"Section 9"}
                p {"This is the ninth section of the page."}
                footer {
                    p {"&copy; 2023 Example Website. All rights reserved."}
                }
            }
        }
    }
    .into_string()
}

use std::sync::LazyLock;

use maud::{html, PreEscaped};
use maud_vs_minijinja::test_maud_vs_minijinja;
use minijinja::{context, Environment};
use serde::Serialize;

#[derive(Serialize)]
struct Item {
    link: &'static str,
    title: &'static str,
    children: Vec<Item>,
}

const ITEMS: LazyLock<Vec<Item>> = LazyLock::new(|| {
    vec![
        Item {
            link: "/",
            title: "Index",
            children: Vec::new(),
        },
        Item {
            link: "/docs",
            title: "Documentation",
            children: vec![
                Item {
                    link: "/docs/installation",
                    title: "Installation",
                    children: Vec::new(),
                },
                Item {
                    link: "/docs/faq",
                    title: "FAQ",
                    children: Vec::new(),
                },
            ],
        },
    ]
});

fn main() {
    LazyLock::force(&ITEMS);
    println!("{}", maud());
    println!("{}", minijinja());
    test_maud_vs_minijinja(maud, minijinja);
}

fn maud() -> String {
    fn rec(v: &[Item]) -> PreEscaped<String> {
        html! {
            @for item in v {
                li {
                    a href=(item.link) {
                        (item.title)
                    }
                    @if !item.children.is_empty() {
                        ul {
                            (rec(&item.children))
                        }
                    }
                }
            }
        }
    }

    html! {
        ul.nav {
            (rec(&ITEMS))
        }

    }
    .into_string()
}

fn minijinja() -> String {
    let mut env = Environment::new();
    env.set_debug(true);
    env.add_template(
        "loop.html",
        r#"
    <ul class="nav">
    {% for item in nav recursive %}
      <li><a href={{ item.link }}">{{ item.title }}</a>{%
        if item.children %}<ul>{{ loop(item.children) }}</ul>{% endif %}</li>
    {% endfor %}
    </ul>
    "#,
    )
    .unwrap();
    let template = env.get_template("loop.html").unwrap();
    template.render(context!(nav => *ITEMS)).unwrap()
}

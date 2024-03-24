#![feature(proc_macro_hygiene)]
use clap::{clap_app, crate_version};
use maud::html;
use pulldown_cmark::{html::push_html, Event, Parser};

fn wrap_html(s: &str, css: Option<&str>) -> String {
    let res = html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                @if let Some(s) = css {
                    link rel="stylesheet" type="text/css" href=(s) {}
                }

            }
            body {
                (maud::PreEscaped(s))
            }
        }
    };
    res.into_string()
}

fn main() {
    let clap = clap_app!(mdrend =>
        (version: crate_version!())
        (author: "FAMASoon")
        (about: "Renders markdown as you like")
        (@arg input: +required "sets the input file")
        (@arg wrap: -w "wrap in html")
        (@arg event: -e "Event")
        (@arg css: --css +takes_value "Link to css")
    )
    .get_matches();
    println!("Input = {:?}", clap.value_of("input"));
    let infile =
        std::fs::read_to_string(clap.value_of("input").unwrap()).expect("Could not read file");
    let mut res = String::new();
    let ps: Parser = Parser::new(&infile);
    let ps: Vec<Event> = ps.into_iter().collect();
    if clap.is_present("event") {
        for p in &ps {
            println!("{:?}", p);
        }
    }
    push_html(&mut res, ps.into_iter());
    if clap.is_present("wrap") {
        res = wrap_html(&res, clap.value_of("css"))
    }
    println!("{}", res);
}

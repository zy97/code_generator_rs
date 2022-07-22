use std::error::Error;

use tera::{Context, Tera};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        // tera.register_filter("do_nothing", do_nothing_filter);
        println!("Tera initialized:{:?}", tera);
        tera
    };
}
fn main() {
    let mut context = Context::new();
    context.insert("username", &"Bob");
    context.insert("numbers", &vec![1, 2, 3]);
    context.insert("show_all", &false);
    context.insert("bio", &"<script>alert('pwnd');</script>");

    // A one off template
    Tera::one_off("hello", &Context::new(), true).unwrap();

    let mut file = std::fs::File::create(r"C:\1.txt").expect("create failed");
    match TEMPLATES.render_to("users/profile.html", &context, file) {
        Ok(()) => println!("写入成功"),
        Err(e) => {
            println!("Error: {}", e);
            let mut cause = e.source();
            while let Some(e) = cause {
                println!("Reason: {}", e);
                cause = e.source();
            }
        }
    };

    // let result = TEMPLATES.render("hello.html", &context).unwrap();
    // println!("{}", result);
}

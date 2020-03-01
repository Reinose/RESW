#![cfg(test)]

#[cfg(feature = "moz_central")]
extern crate rayon;
#[cfg(feature = "moz_central")]
mod spider_monkey;
use ressa::Parser;

use resw::{
    Writer,
    write_str::WriteString,    
};

#[test]
fn everything_js_es5() {
    ensure_libraries();
    let js = ::std::fs::read_to_string("./node_modules/everything.js/es5.js").expect("Failed to read js file");
    let (first, second) = double_round_trip(&js, false);
    check_round_trips("everything.es5", &first, &second);
}

#[test]
fn everything_js_es2015_script() {
    ensure_libraries();
    let js = ::std::fs::read_to_string("./node_modules/everything.js/es2015-script.js").expect("failed to read js file");
    let (first, second) = double_round_trip(&js, false);
    check_round_trips("everything.es2015-script", &first, &second);
}

#[test]
fn everything_js_es2015_module() {
    ensure_libraries();
    let js = ::std::fs::read_to_string("./node_modules/everything.js/es2015-module.js").expect("failed to read js file");
    let (first, second) = double_round_trip(&js, true);
    check_round_trips("everything.es2015-module", &first, &second);
}

#[test]
fn jquery() {
    ensure_libraries();
    let js = ::std::fs::read_to_string("./node_modules/jquery/dist/jquery.js").expect("failed to read js file");
    let (first, second) = double_round_trip(&js, false);
    check_round_trips("jquery", &first, &second);
}

#[test]
fn angular() {
    ensure_libraries();
    let js = ::std::fs::read_to_string("./node_modules/angular/angular.js").expect("failed to read js file");
    let (first, second) = double_round_trip(&js, false);
    check_round_trips("angular", &first, &second);
}

#[test]
fn react() {
    ensure_libraries();
    let js = ::std::fs::read_to_string("node_modules/react/cjs/react.development.js").expect("failed to read js file");
    let (first, second) = double_round_trip(&js, false);
    check_round_trips("react", &first, &second);
}
#[test]
fn react_dom() {
    ensure_libraries();
    let js = ::std::fs::read_to_string("node_modules/react-dom/cjs/react-dom.development.js").expect("failed to read js file");
    let (first, second) = double_round_trip(&js, false);
    check_round_trips("react_dom", &first, &second);
}

#[test]
fn vue() {
    ensure_libraries();
    let js = ::std::fs::read_to_string("node_modules/vue/dist/vue.js").expect("failed to read js file");
    let (first, second) = double_round_trip(&js, false);
    check_round_trips("vue", &first, &second);
}

#[test]
fn dexie() {
    ensure_libraries();
    let js = ::std::fs::read_to_string("node_modules/dexie/dist/dexie.js").expect("failed to read js file");
    let (first, second) = double_round_trip(&js, false);
    check_round_trips("dexie", &first, &second);
}

#[test]
fn moment() {
    ensure_libraries();
    let js = ::std::fs::read_to_string("node_modules/moment/moment.js").expect("failed to read js file");
    let (first, second) = double_round_trip(&js, false);
    check_round_trips("moment", &first, &second);
}

fn double_round_trip(js: &str, module: bool) -> (String, Option<String>) {
    let mut first_write = WriteString::new();
    let mut second_write = WriteString::new();
    let first_parser = Parser::builder().module(module).js(js).build().expect("Failed to create parser");
    let mut first_writer = Writer::new(first_write.generate_child());
    for part in first_parser {
        let part = match part {
            Ok(part) => part,
            Err(e) => {
                println!("Error parsing part in first pass {}", e);
                break;
            },
        };
        first_writer.write_part(&part.as_concrete()).expect("Failed to write part");
    }
    let first_pass = first_write.get_string().expect("Invalid utf-8 written to first write");
    let second_parser = Parser::builder().module(module).js(first_pass.as_str()).build().expect("Failed to create second parser");
    let mut second_writer = Writer::new(second_write.generate_child());
    for part in second_parser {
        let part = match part {
            Ok(part) => part,
            Err(e) => {
                println!("Error parsing part in second pass {}", e);
                let parsed = second_write.get_string().expect("Invalid utf-8 written to second write");
                let second_pass = if parsed.len() == 0 {
                    None
                } else {
                    Some(parsed)
                };
                return (first_pass, second_pass)
            },
        };
        second_writer.write_part(&part.as_concrete()).expect("failed to write part");
    }
    let second_pass = second_write.get_string().expect("Invalid utf-8 written to second write");
    (first_pass, Some(second_pass))
}

fn write_failure(name: &str, first: &str, second: &str) {
    use std::io::Write;
    let mut f1 = ::std::fs::File::create(&format!("test_failures/{}.first.js", name)).expect("Failed to create first failure file");
    f1.write_all(first.as_bytes()).expect("failed to write to first failure file");
    let mut f2 = ::std::fs::File::create(&format!("test_failures/{}.second.js", name)).expect("Failed to create second failure file");
    f2.write_all(second.as_bytes()).expect("failed to write second failure file");
}

fn ensure_libraries() {
    if !::std::path::PathBuf::from("node_modules").exists() {
        let output = ::std::process::Command::new("npm").arg("i").output().expect("failed to execute npm i");
        assert!(output.status.success());
    }
}

pub(crate) fn check_round_trips(name: &str, first: &str, second: &Option<String>) {
    if let Some(second) = second {
        if first != second {
            write_failure(name, first, second);
            panic!("Double round trip failed for {0}\ncheck ./test_failures/{0}.first.js and ./test_failures/{0}.second.js", name);
        } else {
            let mut env_write = false;
            for (key, val) in ::std::env::vars() {
                if key == "RESW_WRITE" && val == "1" {
                    env_write = true;
                    break;
                }
            }
            if env_write {
                write_failure(name, first, second);
            }
        }
    } else {
        ::std::fs::write(&format!("test_failures/{}.first.js", name), first).expect("Failed to write file");
        panic!("Double round trip failed to parse second pass for {0}\n check ./test_failures/{0}.first.js", name);
    }
}

#[test]
fn new_member_expr_failure() {
    let js =
"function isElement(node) {
  return !!(node &&
    (node.nodeName  // We are a direct element.
    || (node.prop && node.attr && node.find)));  // We have an on and find method part of jQuery API.
}";
    let (first, second) = double_round_trip(js, false);
    check_round_trips("new_member_expr_failure", &first, &second);
}

#[test]
fn long_args_failure() {
    let js = "
function f(a, b = 0, [c,, d = 0, ...e], {f, g: h, i = 0, i: j = 0}, ...k){}
";
    let (first, second) = double_round_trip(js, false);
    check_round_trips("long_args_failure", &first, &second);
    println!("{:#?}", first);
}
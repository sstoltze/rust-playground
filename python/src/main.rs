//mod py_macro_rules;

use python_macro::python;

fn main() {
    println!("Hello ...");
    run_python("print(\"... world!\")");

    python! {
        print("Hello world!")
    }

    // This fails with a panic for py_macro_rules since stringify! ignores newlines (since it gets a token stream)
    python! {
        print("Hello,")
            print("world")
    }
}

fn run_python(code: &str) {
    println!("-----");
    println!("{}", code);
    println!("-----");
    // Get global interpreter lock
    let py = pyo3::Python::acquire_gil();
    // No locals, no globals
    if let Err(e) = py.python().run(code, None, None) {
        e.print(py.python());
    }
}

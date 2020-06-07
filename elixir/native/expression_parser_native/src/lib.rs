extern crate rustler;

use expression_parser::{ExpressionFile, Variables};
use rustler::{Encoder, Env, Error, Term};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom world;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.ExpressionParserNative",
    [
        ("parse_and_eval", 1, parse_and_eval),
    ],
    None
}

fn parse_and_eval<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let text: &str = args[0].decode()?;

    let value = match ExpressionFile::parse(text) {
        Ok(x) => x,
        Err(e) => return Ok((atoms::error(), format!("{}", e)).encode(env)),
    };
    let value = match ExpressionFile::eval(value, &mut Variables::default()) {
        Ok(x) => x,
        Err(e) => return Ok((atoms::error(), format!("{}", e)).encode(env)),
    };

    let text = serde_json::to_string(&value).unwrap();

    Ok((atoms::ok(), text).encode(env))
}

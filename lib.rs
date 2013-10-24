/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[link(name = "cssparser", vers = "0.1")];

#[feature(globs, macro_rules)];

extern mod extra;

pub use tokenizer::tokenize;
pub use parser::{parse_stylesheet_rules, parse_rule_list, parse_declaration_list,
                 parse_one_rule, parse_one_declaration, parse_one_component_value};
pub use color::{RGBA, Color, CurrentColor};
pub use nth::parse_nth;
pub use serializer::{ToCss, serialize_identifier, serialize_string};

pub mod ast;
mod tokenizer;
mod parser;
mod color;
mod nth;
mod serializer;

#[cfg(test)]
mod tests;

use std::fmt::{Debug, Display};

fn main() {}

fn x(x: impl Display) {}

fn y(x: impl Debug) {}

fn z(x: impl ToString) {}
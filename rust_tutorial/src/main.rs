#![allow(unused)]

use std::collections::HashMap;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

mod restaurant;
use restaurant::order_food;

fn main() {
    order_food();
}

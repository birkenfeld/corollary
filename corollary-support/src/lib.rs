#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

pub trait OpAddable {
    fn add(self, right: Self) -> Self;
}

pub fn __op_addadd<A: OpAddable>(left: A, right: A) -> A {
    OpAddable::add(left, right)
}

impl OpAddable for String {
    fn add(mut self, right: Self) -> Self {
        self.push_str(&right);
        self
    }
}

impl<A> OpAddable for Vec<A> {
    fn add(mut self, right: Self) -> Self {
        self.extend(right);
        self
    }
}


pub struct IO<A: Sized>(A);

pub fn assertEqual<A: Eq + Sized>(desc: String, left: A, right: A) -> IO<()> {
    if left != right {
        panic!("{}", desc);
    }
    IO(())
}

pub fn putStrLn(line: String) -> IO<()> {
    println!("{}", line);
    IO(())
}

pub mod List {
    pub fn reverse<A>(mut input: Vec<A>) -> Vec<A> {
        input.reverse();
        input
    }
}

pub fn __op_index<F, T: ::std::ops::Index<F>>(a: T, pos: F) -> (<T as std::ops::Index<F>>::Output)
where <T as std::ops::Index<F>>::Output: std::marker::Sized + Clone {
    a[pos].clone()
}

#[macro_export]
macro_rules! __assign {
    ($left: expr, {
        $($field_name:ident: $field_type:expr),+ $(,)*
    }) => {
        // TODO
        $left
    }
}

pub enum Either<A, B> {
    Left(A),
    Right(B)
}
pub use self::Either::*;


use std::fmt::Display;
pub fn show<A: Display>(a: A) -> String {
    format!("{}", a)
}

pub enum ExitCode {
    ExitSuccess,
    ExitFailure(isize),
}
pub use self::ExitCode::*;

pub fn isSuffixOf(a: String, r: String) -> bool {
    r.ends_with(&a)
}

pub fn isPrefixOf(a: String, r: String) -> bool {
    r.starts_with(&a)
}

pub fn elem<T: PartialEq>(item: T, value: Vec<T>) -> bool {
    value.contains(&item)
}

pub fn replicate<T: Clone>(rep: isize, item: T) -> Vec<T> {
    (0..rep).map(|_| item.clone()).collect()
}

pub fn words(input: String) -> Vec<String> {
    input.split_whitespace().map(|x| x.to_string()).collect()
}

pub trait Lengthable {
    fn get_len(&self) -> isize;
}
pub fn length<A: Lengthable>(left: A) -> isize {
    Lengthable::get_len(&left)
}
impl Lengthable for String {
    fn get_len(&self) -> isize {
        self.len() as isize
    }
}

pub trait Bindable<T> {
    fn bind_it(self, right: T) -> Self;
}
pub fn __op_bind<A: Bindable<B>, B>(left: A, b: B) -> A {
    Bindable::bind_it(left, b)
}
impl<T: Display> Bindable<T> for String {
    fn bind_it(mut self, right: T) -> Self {
        // TODO
        self.push_str(&format!("{}", right));
        self
    }
}


pub fn __op_forwardslash<A, B>(left: A, right: B) {
    ()
}
pub fn __op_concat<A, B>(left: A, right: B) {
    ()
}


pub fn union<A: PartialEq>(mut left: Vec<A>, right: Vec<A>) -> Vec<A> {
    for item in right {
        if left.iter().position(|x| *x == item).is_none() {
            left.push(item);
        }
    }
    left
}

pub fn toInteger<T: Display>(left: T) -> isize {
    // TODO
    0
}

pub fn fromInteger(left: isize) -> String {
    // TODO
    "".to_string()
}

pub fn shiftL(l: isize, r: isize) -> isize {
    l << r
}

pub fn shiftR(l: isize, r: isize) -> isize {
    l >> r
}

pub fn fromEnum(arg: bool) -> String {
    //TODO
    "WHAT".to_string()
}

pub fn __op_dotted_and(l: isize, r: isize) -> isize {
    l & r
}

pub fn __op_dotted_or(l: isize, r: isize) -> isize {
    l | r
}

pub fn __op_assign_div(l: isize, r: isize) -> isize {
    l / r
}

pub fn __op_tuple2<A, B>(left: A, right: B) -> (A, B) {
    (left, right)
}

pub fn __op_power(l: isize, r: isize) -> isize {
    //TODO
    l
}

pub fn __mod(l: isize, r: isize) -> isize {
    // TODO
    l
}

pub fn not(left: bool) -> bool {
    !left
}

pub fn __break<T, F: Fn(&T) -> bool>(cond: F, input: Vec<T>) -> (Vec<T>, Vec<T>) {
    let mut left = vec![];
    let mut right = vec![];
    for item in input.into_iter() {
        if right.is_empty() && cond(&item) {
            left.push(item);
        } else {
            right.push(item);
        }
    }
    (left, right)
}

pub fn any<T, F: Fn(&T) -> bool>(cond: F, input: Vec<T>) -> bool {
    input.iter()
        .any(cond)
}

pub fn isJust<T>(input: Option<T>) -> bool {
    input.is_some()
}

pub fn null<T>(input: Vec<T>) -> bool {
    input.is_empty()
}

pub fn lines(input: String) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect()
}

pub fn unlines(input: Vec<String>) -> String {
    input.join("\n")
}

pub fn ord(input: char) -> isize {
    input as isize
}

pub fn isAscii(input: char) -> bool {
    // TODO
    false
}

pub fn isPrint(input: char) -> bool {
    // TODO
    false
}

pub fn isOctDigit(input: char) -> bool {
    // TODO
    false
}

pub fn head(input: Vec<char>) -> char {
    input[0]
}

pub fn tail(input: Vec<char>) -> char {
    input[input.len() - 1]
}

pub fn take(len: isize, input: Vec<String>) {
    // TODO
}


pub fn takeWhile<T, F: Fn(&T) -> bool>(cond: F, input: Vec<T>) -> Vec<T> {
    let mut left = vec![];
    for item in input.into_iter() {
        if cond(&item) {
            left.push(item);
        } else {
            return left
        }
    }
    left
}



// Monads

// pub trait Functor {
//   fmap = liftM
// }

// pub trait Applicative P where
//   pure = return
//   (<*>) = ap

// pub trait Monad<P> {
//   fn ret(Self) -> Self;
//   fn bind(Self) -> Self;
//   fn fail(m) -> Self;
// }






















// ShowS, ReadS

pub trait ShowS {
    fn show_s(&self, String) -> String;
}
pub trait ReadS<A> {
    fn read_s(&self, String) -> Vec<(A, String)>;
}


pub struct showOct(pub isize);
impl ShowS for showOct {
    fn show_s(&self, input: String) -> String {
        format!("{:o}{}", self.0, input)
    }
}

pub struct showString(pub String);
impl ShowS for showString {
    fn show_s(&self, input: String) -> String {
        format!("{}{}", self.0, input)
    }
}

pub struct readHex();
impl ReadS<isize> for readHex {
    fn read_s(&self, input: String) -> Vec<(isize, String)> {
        // TODO
        vec![]
    }
}

pub struct readOct();
impl ReadS<isize> for readOct {
    fn read_s(&self, input: String) -> Vec<(isize, String)> {
        // TODO
        vec![]
    }
}

pub struct readDec();
impl ReadS<isize> for readDec {
    fn read_s(&self, input: String) -> Vec<(isize, String)> {
        // TODO
        vec![]
    }
}






// BSC

pub mod BSC {
    pub fn head(input: Vec<char>) -> char {
        input[0]
    }

    pub fn tail(input: Vec<char>) -> char {
        input[input.len() - 1]
    }

    pub fn null() -> Vec<char> {
        vec![]
    }

    pub fn lines(input: Vec<char>) -> Vec<Vec<char>> {
        //TODO
        vec![]
    }

    pub fn pack(input: Vec<String>) {
        // TODO
    }

    pub fn unpack(input: Vec<String>) {
        // TODO
    }

    pub fn take(len: isize, input: Vec<String>) {
        // TODO
    }
}

pub mod BSW {
    use FilePath;

    pub fn null() -> Vec<char> {
        vec![]
    }

    pub fn head(input: Vec<char>) -> char {
        input[0]
    }

    pub fn tail(input: Vec<char>) -> char {
        input[input.len() - 1]
    }

    pub fn readFile(f: FilePath) {
        // TODO
    }
}

pub struct ByteString();



// Map stuff


#[macro_export]
macro_rules! __map {
    ($fn: expr, $target: expr) => {
        $target.into_iter()
            .map($fn)
            .collect::<Vec<_>>()
    }
}

#[macro_export]
macro_rules! __fmap {
    ($fn: expr, $target: expr) => {
        $target.into_iter()
            .map($fn)
            .collect::<Vec<_>>()
    }
}

#[macro_export]
macro_rules! __concatMap {
    ($fn: expr, $target: expr) => {
        $target.into_iter()
            .flat_map($fn)
            .collect::<Vec<_>>()
    }
}

#[macro_export]
macro_rules! __error {
    ($fn: expr) => {
        // TODO
        panic!("ERROR!")
    }
}





// IO fns

#[allow(dead_code)]
pub struct FilePath {
    pub path: String,
}

pub struct FileHandle {
    pub path: (),
}

pub fn openTempFile(t: FilePath, template: FilePath) -> (FilePath, FileHandle) {
    // TODO
    (FilePath {
        path: "".to_string()
    }, FileHandle {
        path: ()
    })
}

pub fn hClose(h: FileHandle) {
    // TODO
}

pub fn removeFile(h: FileHandle) {
    // TODO
}

pub fn getTemporaryDirectory() -> FilePath {
    // TODO
    FilePath {
        path: "TODO".to_string()
    }
}

pub fn getOutputFileName(h: FileHandle) -> String {
    // TODO
    "TODO".to_string()
}




// TODO what do we do here:

pub fn maybe() {
    // TODO
}

pub fn liftM() {
    // TODO
}

//TODO is this even a monadic fn
pub fn bracket() {
    // TODO
}







// Map things

pub mod Map {
    pub fn insert<T>(mut okay: Vec<T>, key: isize, value: T) -> Vec<T> {
        okay.push(value);
        okay
    }

    pub fn lookup<T>(value: T, inside: Vec<T>) -> isize {
        //TODO
        0
    }
}


use std::hash::Hash;
use std::collections::HashSet;
pub struct Set<T>(HashSet<T>);

impl<T: Eq + Hash> Set<T> {
    pub fn member(item: T, list: Self) -> bool {
        list.0.contains(&item)
    }
}
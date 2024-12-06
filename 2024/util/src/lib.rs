use std::ops::{Index, IndexMut, Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Neg};
use std::str::{Bytes, FromStr};
use std::marker::PhantomData;
use std::hash::Hash;

pub const UP: Point = Point::new(0, -1);
pub const RIGHT: Point = Point::new(1, 0);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);

pub const SOUTHEAST: Point = Point::new(1, 1);
pub const SOUTHWEST: Point = Point::new(-1, 1);
pub const NORTHEAST: Point = Point::new(1, -1);
pub const NORTHWEST: Point = Point::new(-1, -1);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const VALUES: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];

    #[inline] #[must_use]
    pub fn to_index(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }

    #[inline] #[must_use]
    pub fn to_point(self) -> Point {
        match self {
            Direction::Up => UP,
            Direction::Right => RIGHT,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
        }
    }

    #[inline] #[must_use]
    pub fn clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    #[inline] #[must_use]
    pub fn counterwise(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {pub x: i32, pub y: i32}

impl Point {
    #[inline] #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    #[inline] #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline] #[must_use]
    pub fn counterwise(self) -> Self {
        Point::new(self.y, -self.x)
    }
}

// Overloadable operators for `Point` struct as per  https://doc.rust-lang.org/std/ops/index.html
impl Add for Point {
    type Output = Self;

    #[inline] #[must_use]
    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    #[inline] #[must_use]
    fn mul(self, other: i32) -> Self {
        Point::new(self.x * other, self.y * other)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline] #[must_use]
    fn sub(self, other: Self) -> Self {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {pub width: i32, pub height: i32, pub elements: Vec<T>}

impl Matrix<u8> {
    fn build_matrix<F: Fn(&u8) -> u8>(data: &[u8], transform: F) -> Self {
        if data.is_empty() { panic!("input is empty"); }

        // Trim trailing newline if present
        let data = if data.last() == Some(&b'\n') {
            &data[..data.len() - 1]
        } else {
            data
        };

        let mut lines = data.split(|&x| x == b'\n');
        let first_line = lines.next().expect("input must not be empty");
        if first_line.is_empty() {
            panic!("first line is empty, expected non-empty lines");
        }

        let width = first_line.len() as i32;
        // Slight overallocation, since this includes 
        let mut elements = Vec::with_capacity(data.len());
        elements.extend(first_line.iter().map(&transform));

        let mut height = 1;
        for line in lines {
            if line.len() as i32 != width {
                panic!("lines are not of uniform length");
            }
            elements.extend(line.iter().map(&transform));
            height += 1;
        }

        Matrix { width, height, elements }
    }

    #[inline]
    pub fn from_bytes(ascii_map: &[u8]) -> Self {
        Self::build_matrix(ascii_map, |&b| b)
    }

    #[inline]
    pub fn from_bytes_as_digits(ascii_map: &[u8]) -> Self {
        Self::build_matrix(ascii_map, |&b| b.wrapping_sub(b'0'))
    }

    #[inline]
    pub fn from_str(ascii_map: &str) -> Self {
        Self::from_bytes(ascii_map.as_bytes())
    }

    #[inline]
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self[Point::new(x, y)] as char);
            }
            println!();
        }
    }
}

impl<T> Index<Point> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.elements[(index.y * self.width  + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.elements[(index.y * self.width  + index.x) as usize]
    }
}

impl<T> Matrix<T> {
    #[inline]
    pub fn contains(&self, coord: Point) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }
}
impl<T: Copy> Matrix<T> {
    pub fn new(width: i32, height: i32, fill: T) -> Matrix<T> {
        Matrix { width, height, elements: vec![fill; (width*height) as usize] }
    }

    #[inline]
    pub fn get(&self, coord: Point) -> Option<&T> {
        if self.contains(coord) {
            Some(&self.elements[(coord.y * self.width  + coord.x) as usize])
        } else {
            None
        }
    }
    
    #[inline]
    pub fn get_or(&self, coord: Point, default: T) -> T {
        if self.contains(coord) {
            self.elements[(coord.y * self.width  + coord.x) as usize]
        } else {
            default
        }
    }
}

impl<T: Copy + PartialEq> Matrix<T> {
    #[inline]
    pub fn find(&self, to_find: T) -> Option<Point> {
        self.elements.iter()
            .position(|&item| item == to_find)
            .map(|index| {
                Point::new(index as i32 % self.width, index as i32 / self.width)
            })
    }

    pub fn find_all(&self, to_find: T) -> impl Iterator<Item = Point> + '_ {
        self.elements.iter()
            .enumerate()
            .filter_map(move |(index, item)| {
                if item == &to_find {
                    Some(Point::new(index as i32 % self.width, index as i32 / self.width))
                } else {
                    None
                }
            })

    }
}

pub struct NumberExtractor<'a, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8>,
{
    bytes: Bytes<'a>,
    _marker: PhantomData<T>,
}

impl<'a, T> Iterator for NumberExtractor<'a, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut num = T::default();
        let mut found_digit = false;

        while let Some(byte) = self.bytes.next() {
            let dig = byte.wrapping_sub(b'0');
            if dig < 10 {
                num *= T::from(10); // note: panics on overflow
                num += T::from(dig);
                found_digit = true;
            } else if found_digit {
                return Some(num);
            }
        }

        if found_digit {
            Some(num)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        // Estimating that each number could on average be about 3-4 bytes long including separators
        (lower/4, upper.map(|u| u / 3))
    }
}

// Iterator that returns each number in a string one by one
pub fn extract_numbers<T>(input: &str) -> NumberExtractor<'_, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8>,
{
    NumberExtractor { bytes: input.bytes(), _marker: PhantomData }
}

/// A version that can extract signed numbers, allowing a '-' sign in front.
pub struct SignedNumberExtractor<'a, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8> + Neg<Output = T>,
{
    bytes: Bytes<'a>,
    _marker: PhantomData<T>,
}

impl<'a, T> Iterator for SignedNumberExtractor<'a, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8> + Neg<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut num = T::default();
        let mut found_digit = false;
        let mut is_negative = false;

        while let Some(byte) = self.bytes.next() {
            if byte == b'-' {
                if found_digit {
                    break;
                } else {
                    is_negative = true;
                }
            } else {
                let dig = byte.wrapping_sub(b'0');
                if dig < 10 {
                    num *= T::from(10);
                    num += T::from(dig);
                    found_digit = true;
                } else if found_digit {
                    // We've hit a non-digit, return the number
                    break;
                } else if is_negative {
                    is_negative = false;
                }
            }
        }

        if !found_digit {
            return None;
        }

        Some(if is_negative { -num } else { num })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        // Rough estimation
        (lower / 4, upper.map(|u| u / 3))
    }
}

/// Iterator that returns each signed number in a string one by one.
/// This supports a leading '-' sign for negative numbers.
pub fn extract_numbers_signed<T>(input: &str) -> SignedNumberExtractor<'_, T>
where
    T: FromStr + Default + AddAssign + MulAssign + From<u8> + Neg<Output = T>,
{
    SignedNumberExtractor { bytes: input.bytes(), _marker: PhantomData }
}


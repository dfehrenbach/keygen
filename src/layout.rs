/// Data structures and methods for creating and shuffling keyboard layouts.
extern crate rand;

use self::rand::random;
use std::fmt;

/* ----- *
 * TYPES *
 * ----- */

// KeyMap format:
//    LEFT HAND   |    RIGHT HAND
// 0 1 2 3  4  5  |  6 7 8 9 10 11 12
// 13 14 15 16 17 | 18 19 20 21 22 23
// 24 25 26 27 28 | 29 30 31 32 33 34
// 35 36 37 38 39 | 40 41 42 43 44
//             45 | 46 (thumb keys)
//
//    LEFT HAND   |    RIGHT HAND
// ` 1 2 3  4  5  |  6 7 8 9 0  -  =
// q  w  e  r  t  | y  u  i  o  p  [
// a  s  d  f  g  | h  j  k  l  ;  '
// z  x  c  v  b  | n  m  ,  .  /
//             <s> | <s> (thumb keys)

pub struct KeyMap<T>(pub [T; 47]);

impl<T: Copy> Clone for KeyMap<T> {
  fn clone(&self) -> KeyMap<T> {
    KeyMap(self.0)
  }
}

#[derive(Clone)]
pub struct Layer(KeyMap<char>);

#[derive(Clone)]
pub struct Layout(Layer, Layer);

pub struct LayoutPermutations {
  orig_layout: Layout,
  swap_idx: Vec<usize>,
  started: bool,
}

pub struct LayoutPosMap([Option<KeyPress>; 128]);

#[derive(Clone)]
pub struct LayoutShuffleMask(pub KeyMap<bool>);

#[derive(Clone, Copy, PartialEq)]
pub enum Finger {
  Thumb,
  Index,
  Middle,
  Ring,
  Pinky,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Hand {
  Left,
  Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Row {
  Number,
  Top,
  Home,
  Bottom,
  Thumb,
}

#[derive(Clone, Copy)]
pub struct KeyPress {
  pub kc: char,
  pub pos: usize,
  pub finger: Finger,
  pub hand: Hand,
  pub row: Row,
  pub center: bool,
}

/* ------- *
 * STATICS *
 * ------- */

pub static INIT_LAYOUT: Layout = Layout(
  Layer(KeyMap([
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', //
    'j', 'c', 'y', 'f', 'k', 'z', 'l', ',', 'u', 'q', '[', //
    'r', 's', 't', 'h', 'd', 'm', 'n', 'a', 'i', 'o', '\'', //
    '/', 'v', 'g', 'p', 'b', 'x', 'w', '.', ';', ']', //
    'e', ' ',
  ])), //
  Layer(KeyMap([
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', //
    'J', 'C', 'Y', 'F', 'K', 'Z', 'L', '<', 'U', 'Q', '{', //
    'R', 'S', 'T', 'H', 'D', 'M', 'N', 'A', 'I', 'O', '"', //
    '?', 'V', 'G', 'P', 'B', 'X', 'W', '>', ':', '}', //
    'E', ' ',
  ])),
); //

pub static QWERTY_LAYOUT: Layout = Layout(
  Layer(KeyMap([
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', //
    'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', //
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', //
    'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', //
    '\0', ' ',
  ])), //
  Layer(KeyMap([
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', //
    'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', //
    'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', //
    'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', //
    '\0', ' ',
  ])),
);

pub static DVORAK_LAYOUT: Layout = Layout(
  Layer(KeyMap([
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '[', ']', //
    '\'', ',', '.', 'p', 'y', 'f', 'g', 'c', 'r', 'l', '/', //
    'a', 'o', 'e', 'u', 'i', 'd', 'h', 't', 'n', 's', '-', //
    ';', 'q', 'j', 'k', 'x', 'b', 'm', 'w', 'v', 'z', //
    '\0', ' ',
  ])), //
  Layer(KeyMap([
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '{', '}', //
    '"', ',', '.', 'P', 'Y', 'F', 'G', 'C', 'R', 'L', '?', //
    'A', 'O', 'E', 'U', 'I', 'D', 'H', 'T', 'N', 'S', '_', //
    ':', 'Q', 'J', 'K', 'X', 'B', 'M', 'W', 'V', 'Z', //
    '\0', ' ',
  ])),
); //

pub static COLEMAK_LAYOUT: Layout = Layout(
  Layer(KeyMap([
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', //
    'q', 'w', 'f', 'p', 'g', 'j', 'l', 'u', 'y', ';', '[', //
    'a', 'r', 's', 't', 'd', 'h', 'n', 'e', 'i', 'o', '\'', //
    'z', 'x', 'c', 'v', 'b', 'k', 'm', ',', '.', '/', //
    '\0', ' ',
  ])), //
  Layer(KeyMap([
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', //
    'Q', 'W', 'F', 'P', 'G', 'J', 'L', 'U', 'Y', ':', '{', //
    'A', 'R', 'S', 'T', 'D', 'H', 'N', 'E', 'I', 'O', '"', //
    'Z', 'X', 'C', 'V', 'B', 'K', 'M', '<', '>', '?', //
    '\0', ' ',
  ])),
); //

pub static QGMLWY_LAYOUT: Layout = Layout(
  Layer(KeyMap([
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', //
    'q', 'g', 'm', 'l', 'w', 'y', 'f', 'u', 'b', ';', '[', //
    'd', 's', 't', 'n', 'r', 'i', 'a', 'e', 'o', 'h', '\'', //
    'z', 'x', 'c', 'v', 'j', 'k', 'p', ',', '.', '/', //
    '\0', ' ',
  ])), //
  Layer(KeyMap([
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', //
    'Q', 'G', 'M', 'L', 'W', 'Y', 'F', 'U', 'B', ':', '{', //
    'D', 'S', 'T', 'N', 'R', 'I', 'A', 'E', 'O', 'H', '"', //
    'Z', 'X', 'C', 'V', 'J', 'K', 'P', '<', '>', '?', //
    '\0', ' ',
  ])),
); //

pub static WORKMAN_LAYOUT: Layout = Layout(
  Layer(KeyMap([
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', //
    'q', 'd', 'r', 'w', 'b', 'j', 'f', 'u', 'p', ';', '[', //
    'a', 's', 'h', 't', 'g', 'y', 'n', 'e', 'o', 'i', '\'', //
    'z', 'x', 'm', 'c', 'v', 'k', 'l', ',', '.', '/', //
    '\0', ' ',
  ])), //
  Layer(KeyMap([
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', //
    'Q', 'D', 'R', 'W', 'B', 'J', 'F', 'U', 'P', ':', '{', //
    'A', 'S', 'H', 'T', 'G', 'Y', 'N', 'E', 'O', 'I', '"', //
    'Z', 'X', 'M', 'C', 'V', 'K', 'L', '<', '>', '?', //
    '\0', ' ',
  ])),
); //

pub static MALTRON_LAYOUT: Layout = Layout(
  Layer(KeyMap([
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', //
    'q', 'p', 'y', 'c', 'b', 'v', 'm', 'u', 'z', 'l', ']', //
    'a', 'n', 'i', 's', 'f', 'd', 't', 'h', 'o', 'r', '\'', //
    ',', '.', 'j', 'g', '/', ';', 'w', 'k', '-', 'x', //
    'e', ' ',
  ])), //
  Layer(KeyMap([
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', //
    'Q', 'P', 'Y', 'C', 'B', 'V', 'M', 'U', 'Z', 'L', '}', //
    'A', 'N', 'I', 'S', 'F', 'D', 'T', 'H', 'O', 'R', '"', //
    '<', '>', 'J', 'G', '?', ':', 'W', 'K', '_', 'X', //
    'E', ' ',
  ])),
); //

pub static MTGAP_LAYOUT: Layout = Layout(
  Layer(KeyMap([
    '\\', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'q', 'z', //
    'y', 'p', 'o', 'u', '-', 'k', 'd', 'l', 'c', 'w', 'x', //
    'i', 'n', 'e', 'a', ',', 'm', 'h', 't', 's', 'r', '"', //
    '(', ')', '\'', '.', '_', 'b', 'f', 'g', 'v', 'j', //
    'z', ' ',
  ])), //
  Layer(KeyMap([
    '^', '~', '[', '{', '<', '|', '#', '>', '}', ']', '%', '_', '+', //
    'Y', 'P', 'O', 'U', '=', 'K', 'D', 'L', 'C', 'W', 'X', //
    'I', 'N', 'E', 'A', ':', 'M', 'H', 'T', 'S', 'R', '~', //
    '`', '?', '*', ';', '&', 'B', 'F', 'G', 'V', 'J', //
    '\0', ' ',
  ])),
); //

pub static CAPEWELL_LAYOUT: Layout = Layout(
  Layer(KeyMap([
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '[', ']', //
    '.', 'y', 'w', 'd', 'f', 'j', 'p', 'l', 'u', 'q', '/', //
    'a', 'e', 'r', 's', 'g', 'b', 't', 'n', 'i', 'o', '-', //
    'x', 'z', 'c', 'v', ';', 'k', 'w', 'h', ',', '\'', //
    '\0', ' ',
  ])), //
  Layer(KeyMap([
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '{', '}', //
    '>', 'Y', 'W', 'D', 'F', 'J', 'P', 'L', 'U', 'Q', '?', //
    'A', 'E', 'R', 'S', 'G', 'B', 'T', 'N', 'I', 'O', '_', //
    'X', 'Z', 'C', 'V', ':', 'K', 'W', 'H', '<', '"', //
    '\0', ' ',
  ])),
); //

pub static ARENSITO_LAYOUT: Layout = Layout(
  Layer(KeyMap([
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '/', '=', //
    'q', 'l', ',', 'p', '\'', ';', 'f', 'u', 'd', 'k', '[', //
    'a', 'r', 'e', 'n', 'b', 'g', 's', 'i', 't', 'o', '-', //
    'z', 'w', '.', 'h', 'j', 'v', 'c', 'y', 'm', 'x', //
    '\0', ' ',
  ])), //
  Layer(KeyMap([
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '?', '+', //
    'Q', 'L', '<', 'P', '"', ':', 'F', 'U', 'D', 'K', '{', //
    'A', 'R', 'E', 'N', 'B', 'G', 'S', 'I', 'T', 'O', '_', //
    'Z', 'W', '>', 'H', 'J', 'V', 'C', 'Y', 'M', 'X', //
    '\0', ' ',
  ])),
); //

pub static LAYOUT_MASK: LayoutShuffleMask = LayoutShuffleMask(KeyMap([
  false, true, true, true, true, true, false, false, false, false, false, false, false, //
  true, true, false, true, true, false, false, false, false, false, false, //
  true, false, false, false, true, false, false, false, false, false, false, //
  true, true, true, true, true, false, false, false, false, false, //
  false, false,
])); //

static KEY_FINGERS: KeyMap<Finger> = KeyMap([
  Finger::Pinky,
  Finger::Pinky,
  Finger::Ring,
  Finger::Middle,
  Finger::Index,
  Finger::Index,
  Finger::Index,
  Finger::Index,
  Finger::Middle,
  Finger::Ring,
  Finger::Pinky,
  Finger::Pinky,
  Finger::Pinky, //
  Finger::Pinky,
  Finger::Ring,
  Finger::Middle,
  Finger::Index,
  Finger::Index,
  Finger::Index,
  Finger::Index,
  Finger::Middle,
  Finger::Ring,
  Finger::Pinky,
  Finger::Pinky, //
  Finger::Pinky,
  Finger::Ring,
  Finger::Middle,
  Finger::Index,
  Finger::Index,
  Finger::Index,
  Finger::Index,
  Finger::Middle,
  Finger::Ring,
  Finger::Pinky,
  Finger::Pinky, //
  Finger::Pinky,
  Finger::Ring,
  Finger::Middle,
  Finger::Index,
  Finger::Index,
  Finger::Index,
  Finger::Index,
  Finger::Middle,
  Finger::Ring,
  Finger::Pinky, //
  Finger::Thumb,
  Finger::Thumb,
]); //
static KEY_HANDS: KeyMap<Hand> = KeyMap([
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right, //
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right, //
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right, //
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Left,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right,
  Hand::Right, //
  Hand::Left,
  Hand::Right,
]); //
static KEY_ROWS: KeyMap<Row> = KeyMap([
  Row::Number,
  Row::Number,
  Row::Number,
  Row::Number,
  Row::Number,
  Row::Number,
  Row::Number,
  Row::Number,
  Row::Number,
  Row::Number,
  Row::Number,
  Row::Number,
  Row::Number, //
  Row::Top,
  Row::Top,
  Row::Top,
  Row::Top,
  Row::Top,
  Row::Top,
  Row::Top,
  Row::Top,
  Row::Top,
  Row::Top,
  Row::Top, //
  Row::Home,
  Row::Home,
  Row::Home,
  Row::Home,
  Row::Home,
  Row::Home,
  Row::Home,
  Row::Home,
  Row::Home,
  Row::Home,
  Row::Home, //
  Row::Bottom,
  Row::Bottom,
  Row::Bottom,
  Row::Bottom,
  Row::Bottom,
  Row::Bottom,
  Row::Bottom,
  Row::Bottom,
  Row::Bottom,
  Row::Bottom, //
  Row::Thumb,
  Row::Thumb,
]); //
static KEY_CENTER_COLUMN: KeyMap<bool> = KeyMap([
  false, false, false, false, false, true, true, false, false, false, false, false, false, //
  false, false, false, false, true, true, false, false, false, false, false, //
  false, false, false, false, true, true, false, false, false, false, false, //
  false, false, false, false, true, true, false, false, false, false, //
  false, false,
]); //

pub static KP_NONE: Option<KeyPress> = None;

static LAYOUT_FILE_IDXS: KeyMap<usize> = KeyMap([
  0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, //
  15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, //
  28, 29, 30, 31, 32, 34, 35, 36, 37, 38, 39, //
  41, 42, 43, 44, 45, 47, 48, 49, 50, 51, 52, 53, //
]); //

/* ----- *
 * IMPLS *
 * ----- */

impl Layout {
  pub fn from_string(s: &str) -> Layout {
    let s: Vec<char> = s.chars().collect();
    let mut lower: [char; 47] = ['\0'; 47];
    let mut upper: [char; 47] = ['\0'; 47];

    for i in 0..47 {
      let file_i = LAYOUT_FILE_IDXS.0[i];
      lower[i] = *s.get(file_i).unwrap_or(&'\0');
      upper[i] = *s.get(file_i + 40).unwrap_or(&'\0');
    }

    Layout(Layer(KeyMap(lower)), Layer(KeyMap(upper)))
  }

  pub fn shuffle(&mut self, times: usize) {
    for _ in 0..times {
      let (i, j) = Layout::shuffle_position();
      let Layout(ref mut lower, ref mut upper) = *self;
      lower.swap(i, j);
      upper.swap(i, j);
    }
  }

  pub fn get_position_map(&self) -> LayoutPosMap {
    let Layout(ref lower, ref upper) = *self;
    let mut map = [None; 128];
    lower.fill_position_map(&mut map);
    upper.fill_position_map(&mut map);

    LayoutPosMap(map)
  }

  fn shuffle_position() -> (usize, usize) {
    let LayoutShuffleMask(KeyMap(ref mask)) = LAYOUT_MASK;
    let mut i = random::<usize>() % mask.len();
    while mask[i] == false {
      i = random::<usize>() % mask.len();
    }
    let mut j = random::<usize>() % (mask.len() - 1);
    while mask[j] == false || j == i {
      j = random::<usize>() % mask.len();
    }

    (i, j)
  }
}

impl Layer {
  fn swap(&mut self, i: usize, j: usize) {
    let Layer(KeyMap(ref mut layer)) = *self;
    let temp = layer[i];
    layer[i] = layer[j];
    layer[j] = temp;
  }

  fn fill_position_map(&self, map: &mut [Option<KeyPress>; 128]) {
    let Layer(KeyMap(ref layer)) = *self;
    let KeyMap(ref fingers) = KEY_FINGERS;
    let KeyMap(ref hands) = KEY_HANDS;
    let KeyMap(ref rows) = KEY_ROWS;
    let KeyMap(ref centers) = KEY_CENTER_COLUMN;
    for (i, c) in layer.into_iter().enumerate() {
      if *c < (128 as char) {
        map[*c as usize] = Some(KeyPress {
          kc: *c,
          pos: i,
          finger: fingers[i],
          hand: hands[i],
          row: rows[i],
          center: centers[i],
        });
      }
    }
  }
}

impl LayoutPosMap {
  pub fn get_key_position(&self, kc: char) -> &Option<KeyPress> {
    let LayoutPosMap(ref map) = *self;
    if kc < (128 as char) {
      &map[kc as usize]
    } else {
      &KP_NONE
    }
  }
}

impl LayoutPermutations {
  pub fn new(layout: &Layout, depth: usize) -> LayoutPermutations {
    let mut swaps = Vec::with_capacity(depth * 2);
    for _ in 0..(depth * 2) {
      swaps.push(next_free_swap(0, 0).unwrap());
    }
    LayoutPermutations {
      orig_layout: layout.clone(),
      swap_idx: swaps,
      started: false,
    }
  }
}

fn next_free_swap(e: usize, i: usize) -> Option<usize> {
  let LayoutShuffleMask(KeyMap(ref mask)) = LAYOUT_MASK;
  (e..(mask.len() - i)).find(|x| mask[*x])
}

impl Iterator for LayoutPermutations {
  type Item = (Layout, Vec<usize>);

  fn next(&mut self) -> Option<(Layout, Vec<usize>)> {
    let mut some = false;
    let mut idx = 0;
    let mut val = 0;

    if self.started {
      for (i, e) in self.swap_idx.iter_mut().enumerate() {
        // find first non-ceiling, non-masked value and increment it.
        if let Some(free) = next_free_swap(*e + 1, i) {
          *e = free;
          some = true;
          idx = i;
          val = *e;
          break;
        }
      }
    } else if !self.started {
      self.started = true;
      some = true;
      idx = 1;
      val = next_free_swap(0, 1).unwrap();
    }

    if some {
      for i in 0..idx {
        self.swap_idx[i] = val + idx - i;
      }

      let mut layout = self.orig_layout.clone();
      let mut i = 0;
      while i < self.swap_idx.len() {
        let ref mut lower = ((layout.0).0).0;
        let ref mut upper = ((layout.1).0).0;
        let swap_left = self.swap_idx[i];
        let swap_right = self.swap_idx[i + 1];
        lower.swap(swap_left, swap_right);
        upper.swap(swap_left, swap_right);
        i += 2;
      }

      Some((layout, self.swap_idx.clone()))
    } else {
      None
    }
  }
}

impl fmt::Display for Layout {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Layout(ref lower, _) = *self;
    lower.fmt(f)
  }
}

impl fmt::Display for Layer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Layer(KeyMap(ref layer)) = *self;
    write!(
      f,
      "{} {} {} {} {} {} | {} {} {} {} {} {} {}
  {} {} {} {} {} | {} {} {} {} {} {}
  {} {} {} {} {} | {} {} {} {} {} {}
  {} {} {} {} {} | {} {} {} {} {}
           {} | {}",
      layer[0],
      layer[1],
      layer[2],
      layer[3],
      layer[4],
      layer[5],
      layer[6],
      layer[7],
      layer[8],
      layer[9],
      layer[10],
      layer[11],
      layer[12],
      layer[13],
      layer[14],
      layer[15],
      layer[16],
      layer[17],
      layer[18],
      layer[19],
      layer[20],
      layer[21],
      layer[22],
      layer[23],
      layer[24],
      layer[25],
      layer[26],
      layer[27],
      layer[28],
      layer[29],
      layer[30],
      layer[31],
      layer[32],
      layer[33],
      layer[34],
      layer[35],
      layer[36],
      layer[37],
      layer[38],
      layer[39],
      layer[40],
      layer[41],
      layer[42],
      layer[43],
      layer[44],
      layer[45],
      layer[46],
    )
  }
}

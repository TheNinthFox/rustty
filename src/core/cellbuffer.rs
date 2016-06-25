use std::ops::{Index, IndexMut, Deref, DerefMut};
use std::cmp;

/// An array of `Cell`s that represents a terminal display.
///
/// A `CellBuffer` is a two-dimensional array of `Cell`s, each pair of indices correspond to a
/// single point on the underlying terminal.
///
/// The first index, `Cellbuffer[y]`, corresponds to a row, and thus the y-axis. The second
/// index, `Cellbuffer[y][x]`, corresponds to a column within a row and thus the x-axis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellBuffer {
    cols: usize,
    rows: usize,
    buf: Vec<Cell>,
}

impl CellBuffer {
    /// Constructs a new `CellBuffer` with the given number of columns and rows.
    pub fn new(cols: usize, rows: usize) -> CellBuffer {
        let len = cols * rows;
        let mut buf = Vec::with_capacity(len);
        buf.resize(len, Cell::default());
        CellBuffer {
            cols: cols,
            rows: rows,
            buf: buf,
        }
    }

    pub fn get<'a>(&'a self, x: usize, y: usize) -> Option<&'a Cell> {
        if x < self.cols && y < self.rows {
            let offset = (self.cols * y) + x;
            self.buf.get(offset)
        } else {
            None
        }
    }

    pub fn get_mut<'a>(&'a mut self, x: usize, y: usize) -> Option<&'a mut Cell> {
        if x < self.cols && y < self.rows {
            let offset = (self.cols * y) + x;
            self.buf.get_mut(offset)
        } else {
            None
        }
    }

    pub fn clear(&mut self, blank: Cell) {
        for cell in &mut self.buf {
            *cell = blank;
        }
    }

    /// Resizes `CellBuffer` to the given number of rows and columns, using the given `Cell` as
    /// a blank.
    // TODO: test this.
    pub fn resize(&mut self, newcols: usize, newrows: usize, blank: Cell) {
        let mut newbuf: Vec<Cell> = Vec::with_capacity(newcols * newrows);

        let oldrows = self.rows;
        let oldcols = self.cols;
        let minrows = cmp::min(oldrows, newrows);
        let mincols = cmp::min(oldcols, newcols);
        let x_ext_len = newcols.saturating_sub(oldcols);
        let y_ext_len = newrows.saturating_sub(oldrows) * newcols;

        for y in 0..minrows {
            let copy_start = oldcols * y;
            let copy_end = (oldcols * y) + mincols;

            newbuf.extend_from_slice(&self.buf[copy_start..copy_end]);
            let curlen = newbuf.len();
            newbuf.resize(curlen + x_ext_len, blank);
        }

        let curlen = newbuf.len();
        newbuf.resize(curlen + y_ext_len, blank);

        self.cols = newcols;
        self.rows = newrows;
        self.buf = newbuf;
    }
}

impl Deref for CellBuffer {
    type Target = [Cell];

    fn deref<'a>(&'a self) -> &'a [Cell] {
        &self.buf
    }
}

impl DerefMut for CellBuffer {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [Cell] {
        &mut self.buf
    }
}

impl Index<(usize, usize)> for CellBuffer {
    type Output = Cell;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a Cell {
        let (x, y) = index;
        self.get(x, y).expect("index out of bounds")
    }
}

impl IndexMut<(usize, usize)> for CellBuffer {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut Cell {
        let (x, y) = index;
        self.get_mut(x, y).expect("index out of bounds")
    }
}

/// A single point on a terminal display.
///
/// A `Cell` contains a character and style.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
    attrs: Attr,
}

impl Cell {
    /// Creates a new `Cell` with the given `char`, `Color`s and `Attr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let cell = Cell::new('x', Color::Default, Color::Green, Attr::Default);
    /// assert_eq!(cell.ch(), 'x');
    /// assert_eq!(cell.fg(), Color::Default);
    /// assert_eq!(cell.bg(), Color::Green);
    /// assert_eq!(cell.attrs(), Attr::Default);
    /// ```
    pub fn new(ch: char, fg: Color, bg: Color, attrs: Attr) -> Cell {
        Cell {
            ch: ch,
            fg: fg,
            bg: bg,
            attrs: attrs,
        }
    }

    /// Returns the `Cell`'s character.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let cell = Cell::default();
    /// assert_eq!(cell.ch(), ' ');
    ///
    /// let cell = Cell::new('x', Color::Default, Color::Default, Attr::Default);
    /// assert_eq!(cell.ch(), 'x');
    /// ```
    pub fn ch(&self) -> char {
        self.ch
    }

    /// Sets the `Cell`'s character to the given `char`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::Cell;
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.ch(), ' ');
    ///
    /// cell.set_ch('x');
    /// assert_eq!(cell.ch(), 'x');
    /// ```
    pub fn set_ch(&mut self, newch: char) -> &mut Cell {
        self.ch = newch;
        self
    }

    /// Returns the `Cell`'s foreground `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let cell = Cell::new(' ', Color::Blue, Color::Default, Attr::Default);
    /// assert_eq!(cell.fg(), Color::Blue);
    /// ```
    pub fn fg(&self) -> Color {
        self.fg
    }

    /// Sets the `Cell`'s foreground `Color` to the given `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.fg(), Color::Default);
    ///
    /// cell.set_fg(Color::White);
    /// assert_eq!(cell.fg(), Color::White);
    /// ```
    pub fn set_fg(&mut self, newfg: Color) -> &mut Cell {
        self.fg = newfg;
        self
    }

    /// Returns the `Cell`'s background `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let mut cell = Cell::new(' ', Color::Default, Color::Green, Attr::Default);
    /// assert_eq!(cell.bg(), Color::Green);
    /// ```
    pub fn bg(&self) -> Color {
        self.bg
    }

    /// Sets the `Cell`'s background `Color` to the given `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color, Attr};
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.bg(), Color::Default);
    ///
    /// cell.set_bg(Color::Black);
    /// assert_eq!(cell.bg(), Color::Black);
    /// ```
    pub fn set_bg(&mut self, newbg: Color) -> &mut Cell {
        self.bg = newbg;
        self
    }

    pub fn attrs(&self) -> Attr {
        self.attrs
    }

    pub fn set_attrs(&mut self, newattrs: Attr) -> &mut Cell {
        self.attrs = newattrs;
        self
    }
}

impl Default for Cell {
    /// Constructs a new `Cell` with a blank `char` and default `Color`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::{Cell, Color};
    ///
    /// let mut cell = Cell::default();
    /// assert_eq!(cell.ch(), ' ');
    /// assert_eq!(cell.fg(), Color::Default);
    /// assert_eq!(cell.bg(), Color::Default);
    /// ```
    fn default() -> Cell {
        Cell::new(' ', Color::Default, Color::Default, Attr::Default)
    }
}

/// The color of a `Cell`.
///
/// `Color::Default` represents the default color of the underlying terminal.
///
/// The eight basic colors may be used directly and correspond to 0x00..0x07 in the 8-bit (256)
/// color range; in addition, the eight basic colors coupled with `Attr::Bold` correspond to
/// 0x08..0x0f in the 8-bit color range.
///
/// `Color::Byte(..)` may be used to specify a color in the 8-bit range.
///
/// # Examples
///
/// ```
/// use rustty::Color;
///
/// // The default color.
/// let default = Color::Default;
///
/// // A basic color.
/// let red = Color::Red;
///
/// // An 8-bit color.
/// let fancy = Color::Byte(0x01);
///
/// // Basic colors are also 8-bit colors (but not vice-versa).
/// assert_eq!(red.as_byte(), fancy.as_byte())
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Byte(u8),
    Default,
}

impl Color {
    /// Returns the `u8` representation of the `Color`.
    pub fn as_byte(&self) -> u8 {
        match *self {
            Color::Black => 0x00,
            Color::Red => 0x01,
            Color::Green => 0x02,
            Color::Yellow => 0x03,
            Color::Blue => 0x04,
            Color::Magenta => 0x05,
            Color::Cyan => 0x06,
            Color::White => 0x07,
            Color::Byte(b) => b,
            Color::Default => panic!("Attempted to cast default color to u8"),
        }
    }
}

/// The attributes of a `Cell`.
///
/// `Attr` enumerates all combinations of attributes a given style may have.
///
/// `Attr::Default` represents no attribute.
///
/// # Examples
///
/// ```
/// use rustty::Attr;
///
/// // Default attribute.
/// let def = Attr::Default;
///
/// // Base attribute.
/// let base = Attr::Bold;
///
/// // Combination.
/// let comb = Attr::UnderlineReverse;
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Attr {
    Default = 0b000,
    Bold = 0b001,
    Underline = 0b010,
    BoldUnderline = 0b011,
    Reverse = 0b100,
    BoldReverse = 0b101,
    UnderlineReverse = 0b110,
    BoldReverseUnderline = 0b111,
}

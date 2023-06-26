use std::collections::HashMap;
use std::fmt::Display;
use std::vec::Vec;

// left:  '╸'
// up:    '╹'
// right: '╺'
// down:  '╻'
//        '┃'
//        '━'
// ┻ ┣ ┳ ┫

// ┏┓
// ┗┛

type SigilCell = ((i8, i8), char);

const SQUARE: [SigilCell; 4] = [((0, 0), '┏'), ((1, 0), '┓'), ((0, 1), '┗'), ((1, 1), '┛')];

const H_LINE: [SigilCell; 4] = [((0, 0), '╺'), ((1, 0), '━'), ((2, 0), '━'), ((3, 0), '╸')];
const V_LINE: [SigilCell; 4] = [((0, 0), '╻'), ((0, 1), '┃'), ((0, 2), '┃'), ((0, 3), '╹')];

const SH: [SigilCell; 4] = [((-1, 1), '╺'), ((0, 1), '┛'), ((0, 0), '┏'), ((1, 0), '╸')];
const SV: [SigilCell; 4] = [((0, 0), '╻'), ((0, 1), '┗'), ((1, 1), '┓'), ((1, 2), '╹')];

const ZH: [SigilCell; 4] = [((0, 0), '╺'), ((1, 0), '┓'), ((1, 1), '┗'), ((2, 1), '╸')];
const ZV: [SigilCell; 4] = [((0, 0), '╻'), ((0, 1), '┛'), ((-1, 1), '┏'), ((-1, 2), '╹')];

const T_UP: [SigilCell; 4] = [((-1, 1), '╺'), ((0, 1), '┻'), ((0, 0), '╻'), ((1, 1), '╸')];
const T_RIGHT: [SigilCell; 4] = [((0, 0), '╻'), ((0, 1), '┣'), ((1, 1), '╸'), ((0, 2), '╹')];
const T_DOWN: [SigilCell; 4] = [((0, 0), '╺'), ((1, 0), '┳'), ((1, 1), '╹'), ((2, 0), '╸')];
const T_LEFT: [SigilCell; 4] = [((0, 0), '╻'), ((0, 1), '┫'), ((-1, 1), '╺'), ((0, 2), '╹')];

const L_UP: [SigilCell; 4] = [((0, 0), '╻'), ((0, 1), '┃'), ((0, 2), '┗'), ((1, 2), '╸')];
const L_RIGHT: [SigilCell; 4] = [((0, 0), '┏'), ((0, 1), '╹'), ((1, 0), '━'), ((2, 0), '╸')];
const L_DOWN: [SigilCell; 4] = [((0, 0), '╺'), ((1, 0), '┓'), ((1, 1), '┃'), ((1, 2), '╹')];
const L_LEFT: [SigilCell; 4] = [((-2, 1), '╺'), ((-1, 1), '━'), ((0, 1), '┛'), ((0, 0), '╻')];

const J_UP: [SigilCell; 4] = [((0, 0), '╻'), ((0, 1), '┃'), ((0, 2), '┛'), ((-1, 2), '╺')];
const J_RIGHT: [SigilCell; 4] = [((0, 0), '╻'), ((0, 1), '┗'), ((1, 1), '━'), ((2, 1), '╸')];
const J_DOWN: [SigilCell; 4] = [((0, 0), '┏'), ((1, 0), '╸'), ((0, 1), '┃'), ((0, 2), '╹')];
const J_LEFT: [SigilCell; 4] = [((0, 0), '╺'), ((1, 0), '━'), ((2, 0), '┓'), ((2, 1), '╹')];

#[derive(Clone, Copy, Debug)]
pub enum Line {
    Horizontal,
    Vertical,
}

impl Line {
    fn rotate(self) -> Line {
        match self {
            Line::Horizontal => Line::Vertical,
            Line::Vertical => Line::Horizontal,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Tetris {
    O,
    I(Line),
    S(Line),
    Z(Line),
    T(Direction),
    L(Direction),
    J(Direction),
}

impl Tetris {
    pub fn o_default() -> Self {
        Tetris::O
    }

    pub fn i_default() -> Self {
        Tetris::I(Line::Horizontal)
    }
    pub fn s_default() -> Self {
        Tetris::S(Line::Horizontal)
    }

    pub fn z_default() -> Self {
        Tetris::Z(Line::Horizontal)
    }
    pub fn t_default() -> Self {
        Tetris::T(Direction::Down)
    }
    pub fn l_default() -> Self {
        Tetris::L(Direction::Down)
    }
    pub fn j_default() -> Self {
        Tetris::J(Direction::Down)
    }

    pub fn rotate(self) -> Self {
        match self {
            Tetris::O => self,
            Tetris::I(line) => Tetris::I(line.rotate()),
            Tetris::S(line) => Tetris::S(line.rotate()),
            Tetris::Z(line) => Tetris::Z(line.rotate()),
            Tetris::T(dir) => Tetris::T(dir.rotate()),
            Tetris::L(dir) => Tetris::L(dir.rotate()),
            Tetris::J(dir) => Tetris::J(dir.rotate()),
        }
    }
}

#[derive(Debug)]
pub struct Sigil {
    pos: (u8, u8),
    class: Tetris,
}

impl Sigil {
    pub fn new(pos: (u8, u8), tetris: Tetris) -> Self {
        Sigil { pos, class: tetris }
    }

    fn shape(&self) -> &[SigilCell] {
        match self.class {
            Tetris::O => &SQUARE,
            Tetris::I(Line::Horizontal) => &H_LINE,
            Tetris::I(Line::Vertical) => &V_LINE,
            Tetris::S(Line::Horizontal) => &SH,
            Tetris::S(Line::Vertical) => &SV,
            Tetris::Z(Line::Horizontal) => &ZH,
            Tetris::Z(Line::Vertical) => &ZV,
            Tetris::T(Direction::Up) => &T_UP,
            Tetris::T(Direction::Right) => &T_RIGHT,
            Tetris::T(Direction::Down) => &T_DOWN,
            Tetris::T(Direction::Left) => &T_LEFT,
            Tetris::L(Direction::Up) => &L_UP,
            Tetris::L(Direction::Right) => &L_RIGHT,
            Tetris::L(Direction::Down) => &L_DOWN,
            Tetris::L(Direction::Left) => &L_LEFT,
            Tetris::J(Direction::Up) => &J_UP,
            Tetris::J(Direction::Right) => &J_RIGHT,
            Tetris::J(Direction::Down) => &J_DOWN,
            Tetris::J(Direction::Left) => &J_LEFT,
        }
    }

    fn cells(&self) -> impl Iterator<Item = ((i8, i8), &char)> {
        let (x, y) = self.pos;
        self.shape()
            .iter()
            .map(move |(p, s)| ((x as i8 + p.0, y as i8 + p.1), s))
    }

    pub fn n_states(&self) -> i32 {
        match self.class {
            Tetris::O => 1,
            Tetris::I(_) => 2,
            Tetris::S(_) => 2,
            Tetris::Z(_) => 2,
            Tetris::T(_) => 4,
            Tetris::L(_) => 4,
            Tetris::J(_) => 4,
        }
    }

    pub fn rotate(&mut self) {
        self.class = self.class.clone().rotate();
    }
}

pub const EMPTY_CELL: char = '•';

pub struct Board {
    shape: (u8, u8),
    cells: Vec<char>,
    sigils: Vec<Sigil>,
}

impl Board {
    pub fn new(width: u8, height: u8) -> Self {
        Board {
            shape: (width, height),
            cells: vec![EMPTY_CELL; width as usize * height as usize],
            sigils: Vec::new(),
        }
    }

    pub fn sigil_count(&self) -> usize {
        self.sigils.len()
    }

    pub fn capacity(&self) -> usize {
        self.width() as usize * self.height() as usize / 4 - self.sigil_count()
    }

    pub fn width(&self) -> u8 {
        self.shape.0
    }

    pub fn height(&self) -> u8 {
        self.shape.1
    }

    pub fn calc_index(&self, cell: (u8, u8)) -> usize {
        cell.0 as usize + self.width() as usize * cell.1 as usize
    }

    pub fn get(&self, cell: (u8, u8)) -> Option<&char> {
        self.cells.get(self.calc_index(cell))
    }

    pub fn is_cell_avaiable(&self, cell: (i8, i8)) -> bool {
        let (x, y) = cell;
        if x < 0 || y < 0 || x as u8 >= self.width() || y as u8 >= self.height() {
            return false;
        }
        self.cells[self.calc_index((x as u8, y as u8))] == EMPTY_CELL
    }

    pub fn can_sigil_be_placed(&self, sigil: &Sigil) -> bool {
        sigil.cells().all(|(pos, _)| self.is_cell_avaiable(pos))
    }

    pub fn update(&mut self, cell: (u8, u8), item: char) {
        let idx = self.calc_index(cell);
        self.cells[idx] = item;
    }

    pub fn add_sigil(&mut self, sigil: Sigil) -> Result<(), Sigil> {
        if !self.can_sigil_be_placed(&sigil) {
            return Err(sigil);
        }

        for ((x, y), symbol) in sigil.cells() {
            self.update((x as u8, y as u8), *symbol);
        }
        self.sigils.push(sigil);

        Ok(())
    }

    pub fn pop_sigil(&mut self) -> Option<Sigil> {
        if let Some(sigil) = self.sigils.pop() {
            for ((x, y), _) in sigil.cells() {
                self.update((x as u8, y as u8), EMPTY_CELL);
            }
            Some(sigil)
        } else {
            None
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let enlarger = get_enlarger();
        for y in 0..self.height() {
            let mut row_to_display: Vec<&DisplayCell> = Vec::new();
            for x in 0..self.width() {
                if let Some(symbol) = self.get((x, y)) {
                    if let Some(dc) = enlarger.get(symbol) {
                        row_to_display.push(dc);
                    } else {
                        return Err(std::fmt::Error);
                    }
                } else {
                    return Err(std::fmt::Error);
                }
            }

            for dc in row_to_display.iter() {
                write!(f, "{}", dc.0)?;
            }
            write!(f, "\n")?;
            for dc in row_to_display.iter() {
                write!(f, "{}", dc.1)?;
            }
            write!(f, "\n")?;
            for dc in row_to_display.iter() {
                write!(f, "{}", dc.2)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

type DisplayCell = (&'static str, &'static str, &'static str);

// left:  '╸'
// up:    '╹'
// right: '╺'
// down:  '╻'
//        '┃'
//        '━'
// ┻ ┣ ┳ ┫
// ┏┓
// ┗┛

fn get_enlarger() -> HashMap<char, DisplayCell> {
    return [
        (EMPTY_CELL, ("     ", "  •  ", "     ")),
        ('┏', ("┏━━━━", "┃    ", "┃   ┏")),
        ('┓', ("━━━━┓", "    ┃", "┓   ┃")),
        ('┗', ("┃   ┗", "┃    ", "┗━━━━")),
        ('┛', ("┛   ┃", "    ┃", "━━━━┛")),
        ('┻', ("┛   ┗", "     ", "━━━━━")),
        ('┣', ("┃   ┗", "┃    ", "┃   ┏")),
        ('┳', ("━━━━━", "     ", "┓   ┏")),
        ('┫', ("┛   ┃", "    ┃", "┓   ┃")),
        ('┃', ("┃   ┃", "┃   ┃", "┃   ┃")),
        ('━', ("━━━━━", "     ", "━━━━━")),
        ('╻', ("┏━━━┓", "┃   ┃", "┃   ┃")),
        ('╹', ("┃   ┃", "┃   ┃", "┗━━━┛")),
        ('╺', ("┏━━━━", "┃    ", "┗━━━━")),
        ('╸', ("━━━━┓", "    ┃", "━━━━┛")),
    ]
    .into_iter()
    .collect();
}

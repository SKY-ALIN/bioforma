// The biggest part of code from this file is copied from https://github.com/rust-bio/rust-bio,
// because it's hard to re-use it directly and transfer rust generics, traits
// and other rust-specific constructions to python. So, it's copied and adapted.

#[rustfmt::skip]
use bio_types::alignment::{
    Alignment           as _Alignment,
    AlignmentMode       as _AlignmentMode,
    AlignmentOperation  as _AlignmentOperation,
};
use bio::scores::{blosum62, pam120, pam200, pam250, pam40};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::cmp::max;
use std::iter::repeat;
use std::sync::Arc;

use crate::alignment::alignment_type::Alignment;

/// Value to use as a 'negative infinity' score. Should be close to `i32::MIN`,
/// but avoid underflow when used with reasonable scoring parameters or even
/// adding two negative infinities. Use ~ `0.4 * i32::MIN`
const MIN_SCORE: i32 = -858_993_459;

fn make_match_score_function(match_score: i32, mismatch_score: i32) -> impl Fn(u8, u8) -> i32 {
    move |a: u8, b: u8| -> i32 {
        if a == b {
            match_score
        } else {
            mismatch_score
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Scoring {
    pub gap_open: i32,
    pub gap_extend: i32,
    pub match_fn: Arc<dyn Fn(u8, u8) -> i32 + Send + Sync + 'static>,
    pub match_scores: Option<(i32, i32)>,
    pub xclip_prefix: i32,
    pub xclip_suffix: i32,
    pub yclip_prefix: i32,
    pub yclip_suffix: i32,
}

#[pymethods]
impl Scoring {
    #[new]
    pub fn new(gap_open: i32, gap_extend: i32, match_func: &str) -> PyResult<Self> {
        if gap_open > 0 {
            return Err(PyValueError::new_err("gap_open can't be positive"));
        }

        if gap_open > 0 {
            return Err(PyValueError::new_err("gap_extend can't be positive"));
        }

        let func: PyResult<fn(u8, u8) -> i32> = match match_func {
            "blosum62" => Ok(blosum62),
            "pam120" => Ok(pam120),
            "pam200" => Ok(pam200),
            "pam250" => Ok(pam250),
            "pam40" => Ok(pam40),
            _ => Err(PyValueError::new_err("Incorrect mathc_func")),
        };

        Ok(Scoring {
            gap_open,
            gap_extend,
            match_fn: Arc::new(func?),
            match_scores: None,
            xclip_prefix: MIN_SCORE,
            xclip_suffix: MIN_SCORE,
            yclip_prefix: MIN_SCORE,
            yclip_suffix: MIN_SCORE,
        })
    }

    #[classmethod]
    pub fn from_scores(
        _cls: &PyType,
        gap_open: i32,
        gap_extend: i32,
        match_score: i32,
        mismatch_score: i32,
    ) -> PyResult<Self> {
        if gap_open > 0 {
            return Err(PyValueError::new_err("gap_open can't be positive"));
        }

        if gap_open > 0 {
            return Err(PyValueError::new_err("gap_extend can't be positive"));
        }

        if match_score < 0 {
            return Err(PyValueError::new_err("match_score can't be negative"));
        }

        if mismatch_score > 0 {
            return Err(PyValueError::new_err("mismatch_score can't be positive"));
        }

        Ok(Scoring {
            gap_open,
            gap_extend,
            match_fn: Arc::new(make_match_score_function(match_score, mismatch_score)),
            match_scores: Some((match_score, mismatch_score)),
            xclip_prefix: MIN_SCORE,
            xclip_suffix: MIN_SCORE,
            yclip_prefix: MIN_SCORE,
            yclip_suffix: MIN_SCORE,
        })
    }
}

#[allow(non_snake_case)]
#[pyclass]
pub struct PairwiseAligner {
    I: [Vec<i32>; 2],
    D: [Vec<i32>; 2],
    S: [Vec<i32>; 2],
    Lx: Vec<usize>,
    Ly: Vec<usize>,
    Sn: Vec<i32>,
    traceback: Traceback,
    scoring: Scoring,
}

impl PairwiseAligner {
    fn calculate_unwrapped_custom_alignment(&mut self, x: &[u8], y: &[u8]) -> PyResult<_Alignment> {
        let (m, n) = (x.len(), y.len());
        self.traceback.init(m, n);

        // Set the initial conditions
        // We are repeating some work, but that's okay!
        for k in 0..2 {
            self.I[k].clear();
            self.D[k].clear();
            self.S[k].clear();

            self.D[k].extend(repeat(MIN_SCORE).take(m + 1));
            self.I[k].extend(repeat(MIN_SCORE).take(m + 1));
            self.S[k].extend(repeat(MIN_SCORE).take(m + 1));

            self.S[k][0] = 0;

            if k == 0 {
                let mut tb = TracebackCell::new();
                tb.set_all(TB_START);
                self.traceback.set(0, 0, tb);
                self.Lx.clear();
                self.Lx.extend(repeat(0usize).take(n + 1));
                self.Ly.clear();
                self.Ly.extend(repeat(0usize).take(m + 1));
                self.Sn.clear();
                self.Sn.extend(repeat(MIN_SCORE).take(m + 1));
                self.Sn[0] = self.scoring.yclip_suffix;
                self.Ly[0] = n;
            }

            for i in 1..=m {
                let mut tb = TracebackCell::new();
                tb.set_all(TB_START);
                if i == 1 {
                    self.I[k][i] = self.scoring.gap_open + self.scoring.gap_extend;
                    tb.set_i_bits(TB_START);
                } else {
                    // Insert all i characters
                    let i_score = self.scoring.gap_open + self.scoring.gap_extend * (i as i32);
                    let c_score =
                        self.scoring.xclip_prefix + self.scoring.gap_open + self.scoring.gap_extend; // Clip then insert
                    if i_score > c_score {
                        self.I[k][i] = i_score;
                        tb.set_i_bits(TB_INS);
                    } else {
                        self.I[k][i] = c_score;
                        tb.set_i_bits(TB_XCLIP_PREFIX);
                    }
                }

                if i == m {
                    tb.set_s_bits(TB_XCLIP_SUFFIX);
                } else {
                    self.S[k][i] = MIN_SCORE;
                }

                if self.I[k][i] > self.S[k][i] {
                    self.S[k][i] = self.I[k][i];
                    tb.set_s_bits(TB_INS);
                }

                if self.scoring.xclip_prefix > self.S[k][i] {
                    self.S[k][i] = self.scoring.xclip_prefix;
                    tb.set_s_bits(TB_XCLIP_PREFIX);
                }

                // Track the score if we do a suffix clip (x) after this character
                if i != m && self.S[k][i] + self.scoring.xclip_suffix > self.S[k][m] {
                    self.S[k][m] = self.S[k][i] + self.scoring.xclip_suffix;
                    self.Lx[0] = m - i;
                }

                if k == 0 {
                    self.traceback.set(i, 0, tb);
                }
                // Track the score if we do suffix clip (y) from here
                if self.S[k][i] + self.scoring.yclip_suffix > self.Sn[i] {
                    self.Sn[i] = self.S[k][i] + self.scoring.yclip_suffix;
                    self.Ly[i] = n;
                }
            }
        }

        for j in 1..=n {
            let curr = j % 2;
            let prev = 1 - curr;

            {
                // Handle i = 0 case
                let mut tb = TracebackCell::new();
                self.I[curr][0] = MIN_SCORE;

                if j == 1 {
                    self.D[curr][0] = self.scoring.gap_open + self.scoring.gap_extend;
                    tb.set_d_bits(TB_START);
                } else {
                    // Delete all j characters
                    let d_score = self.scoring.gap_open + self.scoring.gap_extend * (j as i32);
                    let c_score =
                        self.scoring.yclip_prefix + self.scoring.gap_open + self.scoring.gap_extend;
                    if d_score > c_score {
                        self.D[curr][0] = d_score;
                        tb.set_d_bits(TB_DEL);
                    } else {
                        self.D[curr][0] = c_score;
                        tb.set_d_bits(TB_YCLIP_PREFIX);
                    }
                }
                if self.D[curr][0] > self.scoring.yclip_prefix {
                    self.S[curr][0] = self.D[curr][0];
                    tb.set_s_bits(TB_DEL);
                } else {
                    self.S[curr][0] = self.scoring.yclip_prefix;
                    tb.set_s_bits(TB_YCLIP_PREFIX);
                }

                if j == n && self.Sn[0] > self.S[curr][0] {
                    // Check if the suffix clip score is better
                    self.S[curr][0] = self.Sn[0];
                    tb.set_s_bits(TB_YCLIP_SUFFIX);
                // Track the score if we do suffix clip (y) from here
                } else if self.S[curr][0] + self.scoring.yclip_suffix > self.Sn[0] {
                    self.Sn[0] = self.S[curr][0] + self.scoring.yclip_suffix;
                    self.Ly[0] = n - j;
                }

                self.traceback.set(0, j, tb);
            }

            for i in 1..=m {
                self.S[curr][i] = MIN_SCORE;
            }

            let q = y[j - 1];
            let xclip_score = self.scoring.xclip_prefix
                + max(
                    self.scoring.yclip_prefix,
                    self.scoring.gap_open + self.scoring.gap_extend * (j as i32),
                );
            for i in 1..m + 1 {
                let p = x[i - 1];
                let mut tb = TracebackCell::new();
                let match_fn = Arc::clone(&self.scoring.match_fn);
                let m_score = self.S[prev][i - 1] + match_fn(p, q);

                let i_score = self.I[curr][i - 1] + self.scoring.gap_extend;
                let s_score = self.S[curr][i - 1] + self.scoring.gap_open + self.scoring.gap_extend;
                let best_i_score;
                if i_score > s_score {
                    best_i_score = i_score;
                    tb.set_i_bits(TB_INS);
                } else {
                    best_i_score = s_score;
                    tb.set_i_bits(self.traceback.get(i - 1, j).get_s_bits());
                }

                let d_score = self.D[prev][i] + self.scoring.gap_extend;
                let s_score = self.S[prev][i] + self.scoring.gap_open + self.scoring.gap_extend;
                let best_d_score;
                if d_score > s_score {
                    best_d_score = d_score;
                    tb.set_d_bits(TB_DEL);
                } else {
                    best_d_score = s_score;
                    tb.set_d_bits(self.traceback.get(i, j - 1).get_s_bits());
                }

                tb.set_s_bits(TB_XCLIP_SUFFIX);
                let mut best_s_score = self.S[curr][i];

                if m_score > best_s_score {
                    best_s_score = m_score;
                    tb.set_s_bits(if p == q { TB_MATCH } else { TB_SUBST });
                }

                if best_i_score > best_s_score {
                    best_s_score = best_i_score;
                    tb.set_s_bits(TB_INS);
                }

                if best_d_score > best_s_score {
                    best_s_score = best_d_score;
                    tb.set_s_bits(TB_DEL);
                }

                if xclip_score > best_s_score {
                    best_s_score = xclip_score;
                    tb.set_s_bits(TB_XCLIP_PREFIX);
                }

                let yclip_score = self.scoring.yclip_prefix
                    + self.scoring.gap_open
                    + self.scoring.gap_extend * (i as i32);
                if yclip_score > best_s_score {
                    best_s_score = yclip_score;
                    tb.set_s_bits(TB_YCLIP_PREFIX);
                }

                self.S[curr][i] = best_s_score;
                self.I[curr][i] = best_i_score;
                self.D[curr][i] = best_d_score;

                // Track the score if we do suffix clip (x) from here
                if self.S[curr][i] + self.scoring.xclip_suffix > self.S[curr][m] {
                    self.S[curr][m] = self.S[curr][i] + self.scoring.xclip_suffix;
                    self.Lx[j] = m - i;
                }

                // Track the score if we do suffix clip (y) from here
                if self.S[curr][i] + self.scoring.yclip_suffix > self.Sn[i] {
                    self.Sn[i] = self.S[curr][i] + self.scoring.yclip_suffix;
                    self.Ly[i] = n - j;
                }

                self.traceback.set(i, j, tb);
            }
        }

        // Handle suffix clipping in the j=n case
        for i in 0..=m {
            let j = n;
            let curr = j % 2;
            if self.Sn[i] > self.S[curr][i] {
                self.S[curr][i] = self.Sn[i];
                self.traceback.get_mut(i, j).set_s_bits(TB_YCLIP_SUFFIX);
            }
            if self.S[curr][i] + self.scoring.xclip_suffix > self.S[curr][m] {
                self.S[curr][m] = self.S[curr][i] + self.scoring.xclip_suffix;
                self.Lx[j] = m - i;
                self.traceback.get_mut(m, j).set_s_bits(TB_XCLIP_SUFFIX);
            }
        }

        // Since there could be a change in the last column of S,
        // recompute the last column of I as this could also change
        for i in 1..=m {
            let j = n;
            let curr = j % 2;
            let s_score = self.S[curr][i - 1] + self.scoring.gap_open + self.scoring.gap_extend;
            if s_score > self.I[curr][i] {
                self.I[curr][i] = s_score;
                let s_bit = self.traceback.get(i - 1, j).get_s_bits();
                self.traceback.get_mut(i, j).set_i_bits(s_bit);
            }
            if s_score > self.S[curr][i] {
                self.S[curr][i] = s_score;
                self.traceback.get_mut(i, j).set_s_bits(TB_INS);
                if self.S[curr][i] + self.scoring.xclip_suffix > self.S[curr][m] {
                    self.S[curr][m] = self.S[curr][i] + self.scoring.xclip_suffix;
                    self.Lx[j] = m - i;
                    self.traceback.get_mut(m, j).set_s_bits(TB_XCLIP_SUFFIX);
                }
            }
        }

        let mut i = m;
        let mut j = n;
        let mut operations = Vec::with_capacity(x.len());
        let mut xstart: usize = 0usize;
        let mut ystart: usize = 0usize;
        let mut xend = m;
        let mut yend = n;

        let mut last_layer = self.traceback.get(i, j).get_s_bits();

        loop {
            let next_layer: u16;
            match last_layer {
                TB_START => break,
                TB_INS => {
                    operations.push(_AlignmentOperation::Ins);
                    next_layer = self.traceback.get(i, j).get_i_bits();
                    i -= 1;
                }
                TB_DEL => {
                    operations.push(_AlignmentOperation::Del);
                    next_layer = self.traceback.get(i, j).get_d_bits();
                    j -= 1;
                }
                TB_MATCH => {
                    operations.push(_AlignmentOperation::Match);
                    next_layer = self.traceback.get(i - 1, j - 1).get_s_bits();
                    i -= 1;
                    j -= 1;
                }
                TB_SUBST => {
                    operations.push(_AlignmentOperation::Subst);
                    next_layer = self.traceback.get(i - 1, j - 1).get_s_bits();
                    i -= 1;
                    j -= 1;
                }
                TB_XCLIP_PREFIX => {
                    operations.push(_AlignmentOperation::Xclip(i));
                    xstart = i;
                    i = 0;
                    next_layer = self.traceback.get(0, j).get_s_bits();
                }
                TB_XCLIP_SUFFIX => {
                    operations.push(_AlignmentOperation::Xclip(self.Lx[j]));
                    i -= self.Lx[j];
                    xend = i;
                    next_layer = self.traceback.get(i, j).get_s_bits();
                }
                TB_YCLIP_PREFIX => {
                    operations.push(_AlignmentOperation::Yclip(j));
                    ystart = j;
                    j = 0;
                    next_layer = self.traceback.get(i, 0).get_s_bits();
                }
                TB_YCLIP_SUFFIX => {
                    operations.push(_AlignmentOperation::Yclip(self.Ly[i]));
                    j -= self.Ly[i];
                    yend = j;
                    next_layer = self.traceback.get(i, j).get_s_bits();
                }
                _ => return Err(PyValueError::new_err("Dint expect this!")),
            }
            last_layer = next_layer;
        }

        operations.reverse();
        Ok(_Alignment {
            score: self.S[n % 2][m],
            ystart,
            xstart,
            yend,
            xend,
            ylen: n,
            xlen: m,
            operations,
            mode: _AlignmentMode::Custom,
        })
    }
}

pub const DEFAULT_ALIGNER_CAPACITY: usize = 200;

#[pymethods]
impl PairwiseAligner {
    #[new]
    #[pyo3(signature = (scoring, m=DEFAULT_ALIGNER_CAPACITY, n=DEFAULT_ALIGNER_CAPACITY))]
    pub fn new(scoring: Scoring, m: usize, n: usize) -> Self {
        PairwiseAligner {
            I: [Vec::with_capacity(m + 1), Vec::with_capacity(m + 1)],
            D: [Vec::with_capacity(m + 1), Vec::with_capacity(m + 1)],
            S: [Vec::with_capacity(m + 1), Vec::with_capacity(m + 1)],
            Lx: Vec::with_capacity(n + 1),
            Ly: Vec::with_capacity(m + 1),
            Sn: Vec::with_capacity(m + 1),
            traceback: Traceback::with_capacity(m, n),
            scoring,
        }
    }

    pub fn custom(&mut self, x: &[u8], y: &[u8]) -> PyResult<Alignment> {
        let unwrapped_alignment = self.calculate_unwrapped_custom_alignment(x, y)?;
        Ok(Alignment(unwrapped_alignment))
    }

    pub fn global(&mut self, x: &[u8], y: &[u8]) -> PyResult<Alignment> {
        // Store the current clip penalties
        let clip_penalties = [
            self.scoring.xclip_prefix,
            self.scoring.xclip_suffix,
            self.scoring.yclip_prefix,
            self.scoring.yclip_suffix,
        ];

        // Temporarily Over-write the clip penalties
        self.scoring.xclip_prefix = MIN_SCORE;
        self.scoring.xclip_suffix = MIN_SCORE;
        self.scoring.yclip_prefix = MIN_SCORE;
        self.scoring.yclip_suffix = MIN_SCORE;

        // Compute the alignment
        let mut alignment = self.calculate_unwrapped_custom_alignment(x, y)?;
        alignment.mode = _AlignmentMode::Global;

        // Set the clip penalties to the original values
        self.scoring.xclip_prefix = clip_penalties[0];
        self.scoring.xclip_suffix = clip_penalties[1];
        self.scoring.yclip_prefix = clip_penalties[2];
        self.scoring.yclip_suffix = clip_penalties[3];

        Ok(Alignment(alignment))
    }

    pub fn semiglobal(&mut self, x: &[u8], y: &[u8]) -> PyResult<Alignment> {
        // Store the current clip penalties
        let clip_penalties = [
            self.scoring.xclip_prefix,
            self.scoring.xclip_suffix,
            self.scoring.yclip_prefix,
            self.scoring.yclip_suffix,
        ];

        // Temporarily Over-write the clip penalties
        self.scoring.xclip_prefix = MIN_SCORE;
        self.scoring.xclip_suffix = MIN_SCORE;
        self.scoring.yclip_prefix = 0;
        self.scoring.yclip_suffix = 0;

        // Compute the alignment
        let mut alignment = self.calculate_unwrapped_custom_alignment(x, y)?;
        alignment.mode = _AlignmentMode::Semiglobal;

        // Filter out Xclip and Yclip from alignment.operations
        alignment.filter_clip_operations();

        // Set the clip penalties to the original values
        self.scoring.xclip_prefix = clip_penalties[0];
        self.scoring.xclip_suffix = clip_penalties[1];
        self.scoring.yclip_prefix = clip_penalties[2];
        self.scoring.yclip_suffix = clip_penalties[3];

        Ok(Alignment(alignment))
    }

    pub fn local(&mut self, x: &[u8], y: &[u8]) -> PyResult<Alignment> {
        // Store the current clip penalties
        let clip_penalties = [
            self.scoring.xclip_prefix,
            self.scoring.xclip_suffix,
            self.scoring.yclip_prefix,
            self.scoring.yclip_suffix,
        ];

        // Temporarily Over-write the clip penalties
        self.scoring.xclip_prefix = 0;
        self.scoring.xclip_suffix = 0;
        self.scoring.yclip_prefix = 0;
        self.scoring.yclip_suffix = 0;

        // Compute the alignment
        let mut alignment = self.calculate_unwrapped_custom_alignment(x, y)?;
        alignment.mode = _AlignmentMode::Local;

        // Filter out Xclip and Yclip from alignment.operations
        alignment.filter_clip_operations();

        // Set the clip penalties to the original values
        self.scoring.xclip_prefix = clip_penalties[0];
        self.scoring.xclip_suffix = clip_penalties[1];
        self.scoring.yclip_prefix = clip_penalties[2];
        self.scoring.yclip_suffix = clip_penalties[3];

        Ok(Alignment(alignment))
    }
}

/// ! Non-changed code ! ///

/// Packed representation of one cell of a Smith-Waterman traceback matrix.
/// Stores the I, D and S traceback matrix values in two bytes.
/// Possible traceback moves include : start, insert, delete, match, substitute,
/// prefix clip and suffix clip for x & y. So we need 4 bits each for matrices I, D, S
/// to keep track of these 9 moves.
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TracebackCell {
    v: u16,
}

// Traceback bit positions (LSB)
const I_POS: u8 = 0; // Meaning bits 0,1,2,3 corresponds to I and so on
const D_POS: u8 = 4;
const S_POS: u8 = 8;

// Traceback moves
const TB_START: u16 = 0b0000;
const TB_INS: u16 = 0b0001;
const TB_DEL: u16 = 0b0010;
const TB_SUBST: u16 = 0b0011;
const TB_MATCH: u16 = 0b0100;

const TB_XCLIP_PREFIX: u16 = 0b0101; // prefix clip of x
const TB_XCLIP_SUFFIX: u16 = 0b0110; // suffix clip of x
const TB_YCLIP_PREFIX: u16 = 0b0111; // prefix clip of y
const TB_YCLIP_SUFFIX: u16 = 0b1000; // suffix clip of y

const TB_MAX: u16 = 0b1000; // Useful in checking that the
                            // TB value we got is a valid one

impl TracebackCell {
    /// Initialize a blank traceback cell
    #[inline(always)]
    pub fn new() -> TracebackCell {
        Default::default()
    }

    /// Sets 4 bits [pos, pos+4) with the 4 LSBs of value
    #[inline(always)]
    fn set_bits(&mut self, pos: u8, value: u16) {
        let bits: u16 = (0b1111) << pos;
        assert!(
            value <= TB_MAX,
            "Expected a value <= TB_MAX while setting traceback bits"
        );
        self.v = (self.v & !bits) // First clear the bits
            | (value << pos) // And set the bits
    }

    #[inline(always)]
    pub fn set_i_bits(&mut self, value: u16) {
        // Traceback corresponding to matrix I
        self.set_bits(I_POS, value);
    }

    #[inline(always)]
    pub fn set_d_bits(&mut self, value: u16) {
        // Traceback corresponding to matrix D
        self.set_bits(D_POS, value);
    }

    #[inline(always)]
    pub fn set_s_bits(&mut self, value: u16) {
        // Traceback corresponding to matrix S
        self.set_bits(S_POS, value);
    }

    // Gets 4 bits [pos, pos+4) of v
    #[inline(always)]
    fn get_bits(self, pos: u8) -> u16 {
        (self.v >> pos) & (0b1111)
    }

    #[inline(always)]
    pub fn get_i_bits(self) -> u16 {
        self.get_bits(I_POS)
    }

    #[inline(always)]
    pub fn get_d_bits(self) -> u16 {
        self.get_bits(D_POS)
    }

    #[inline(always)]
    pub fn get_s_bits(self) -> u16 {
        self.get_bits(S_POS)
    }

    /// Set all matrices to the same value.
    pub fn set_all(&mut self, value: u16) {
        self.set_i_bits(value);
        self.set_d_bits(value);
        self.set_s_bits(value);
    }
}

/// Internal traceback.
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Traceback {
    rows: usize,
    cols: usize,
    matrix: Vec<TracebackCell>,
}

impl Traceback {
    fn with_capacity(m: usize, n: usize) -> Self {
        let rows = m + 1;
        let cols = n + 1;
        Traceback {
            rows,
            cols,
            matrix: Vec::with_capacity(rows * cols),
        }
    }

    fn init(&mut self, m: usize, n: usize) {
        self.matrix.clear();
        let mut start = TracebackCell::new();
        start.set_all(TB_START);
        // set every cell to start
        self.resize(m, n, start);
    }

    #[inline(always)]
    fn set(&mut self, i: usize, j: usize, v: TracebackCell) {
        debug_assert!(i < self.rows);
        debug_assert!(j < self.cols);
        self.matrix[i * self.cols + j] = v;
    }

    #[inline(always)]
    fn get(&self, i: usize, j: usize) -> &TracebackCell {
        debug_assert!(i < self.rows);
        debug_assert!(j < self.cols);
        &self.matrix[i * self.cols + j]
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut TracebackCell {
        debug_assert!(i < self.rows);
        debug_assert!(j < self.cols);
        &mut self.matrix[i * self.cols + j]
    }

    fn resize(&mut self, m: usize, n: usize, v: TracebackCell) {
        self.rows = m + 1;
        self.cols = n + 1;
        self.matrix.resize(self.rows * self.cols, v);
    }
}

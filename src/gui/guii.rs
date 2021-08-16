#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2Di {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size2Di {
    w: i32,
    h: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect2Di {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Point2Di {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn x(&self) -> i32 {
        self.x
    }
    #[inline]
    pub fn y(&self) -> i32 {
        self.y
    }
}

impl Size2Di {
    #[inline]
    pub fn new(w: i32, h: i32) -> Self {
        Self { w, h }
    }
    #[inline]
    pub fn w(&self) -> i32 {
        self.w
    }
    #[inline]
    pub fn h(&self) -> i32 {
        self.h
    }
}

impl Rect2Di {
    #[inline]
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
    #[inline]
    pub fn point(&self) -> Point2Di {
        (self.x, self.y).into()
    }
    #[inline]
    pub fn size(&self) -> Size2Di {
        (self.w, self.h).into()
    }
    #[inline]
    pub fn x(&self) -> i32 {
        self.x
    }
    #[inline]
    pub fn y(&self) -> i32 {
        self.y
    }
    #[inline]
    pub fn w(&self) -> i32 {
        self.w
    }
    #[inline]
    pub fn h(&self) -> i32 {
        self.h
    }
}

impl std::fmt::Display for Point2Di {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "+{}+{}", self.x, self.y)
    }
}

impl std::fmt::Display for Size2Di {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}x{}", self.w, self.h)
    }
}

impl std::fmt::Display for Rect2Di {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}x{}+{}+{}", self.w, self.h, self.x, self.y)
    }
}

impl std::convert::From<(i32, i32)> for Point2Di {
    fn from(a: (i32, i32)) -> Self {
        Point2Di::new(a.0, a.1)
    }
}

impl std::convert::From<(f32, f32)> for Point2Di {
    fn from(a: (f32, f32)) -> Self {
        Point2Di::new(a.0 as i32, a.1 as i32)
    }
}

impl std::convert::From<(f64, f64)> for Point2Di {
    fn from(a: (f64, f64)) -> Self {
        Point2Di::new(a.0 as i32, a.1 as i32)
    }
}

impl std::convert::From<(i32, i32)> for Size2Di {
    fn from(a: (i32, i32)) -> Self {
        Size2Di::new(a.0, a.1)
    }
}

impl std::convert::From<(f32, f32)> for Size2Di {
    fn from(a: (f32, f32)) -> Self {
        Size2Di::new(a.0 as i32, a.1 as i32)
    }
}

impl std::convert::From<(f64, f64)> for Size2Di {
    fn from(a: (f64, f64)) -> Self {
        Size2Di::new(a.0 as i32, a.1 as i32)
    }
}

impl std::convert::From<(i32, i32, i32, i32)> for Rect2Di {
    fn from(a: (i32, i32, i32, i32)) -> Self {
        Rect2Di::new(a.0, a.1, a.2, a.3)
    }
}

impl std::convert::From<(f32, f32, f32, f32)> for Rect2Di {
    fn from(a: (f32, f32, f32, f32)) -> Self {
        Rect2Di::new(a.0 as i32, a.1 as i32, a.2 as i32, a.3 as i32)
    }
}

impl std::convert::From<(f64, f64, f64, f64)> for Rect2Di {
    fn from(a: (f64, f64, f64, f64)) -> Self {
        Rect2Di::new(a.0 as i32, a.1 as i32, a.2 as i32, a.3 as i32)
    }
}

impl std::convert::From<((i32, i32), (i32, i32))> for Rect2Di {
    fn from(a: ((i32, i32), (i32, i32))) -> Self {
        Rect2Di::new(a.0 .0, a.0 .1, a.1 .0, a.1 .1)
    }
}

impl std::convert::From<((f32, f32), (f32, f32))> for Rect2Di {
    fn from(a: ((f32, f32), (f32, f32))) -> Self {
        Rect2Di::new(a.0 .0 as i32, a.0 .1 as i32, a.1 .0 as i32, a.1 .1 as i32)
    }
}

impl std::convert::From<((f64, f64), (f64, f64))> for Rect2Di {
    fn from(a: ((f64, f64), (f64, f64))) -> Self {
        Rect2Di::new(a.0 .0 as i32, a.0 .1 as i32, a.1 .0 as i32, a.1 .1 as i32)
    }
}

impl std::convert::From<(Point2Di, Size2Di)> for Rect2Di {
    fn from(a: (Point2Di, Size2Di)) -> Self {
        Rect2Di::new(a.0.x(), a.0.y(), a.1.w(), a.1.h())
    }
}

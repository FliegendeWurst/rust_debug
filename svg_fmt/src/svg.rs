use std::fmt;

pub trait Number {
    fn to_f32(&self) -> f32;
}

macro_rules! impl_to_number {
    ($t:ty) => {
        impl Number for $t {
            fn to_f32(&self) -> f32 {
                *self as f32
            }
        }
    };
}

impl_to_number!(u8);
impl_to_number!(u16);
impl_to_number!(u32);
impl_to_number!(u64);
impl_to_number!(usize);
impl_to_number!(i8);
impl_to_number!(i16);
impl_to_number!(i32);
impl_to_number!(i64);
impl_to_number!(isize);
impl_to_number!(f32);

pub trait IntoString {
    fn into(self) -> String;
}

impl<T: Into<String>> IntoString for T {
    fn into(self) -> String {
        self.into()
    }
}

#[rustversion::before(1.46)]
impl IntoString for char {
    fn into(self) -> String {
        self.to_string()
    }
}

/// `rgb({r},{g},{b})`
#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.r, self.g, self.b)
    }
}

pub const fn rgb(r: u8, g: u8, b: u8) -> Color { Color { r, g, b } }
pub const fn black() -> Color { rgb(0, 0, 0) }
pub const fn white() -> Color { rgb(255, 255, 255) }
pub const fn red() -> Color { rgb(255, 0, 0) }
pub const fn green() -> Color { rgb(0, 255, 0) }
pub const fn blue() -> Color { rgb(0, 0, 255) }

/// `fill:{self}`
#[derive(Copy, Clone, PartialEq)]
pub enum Fill {
    Color(Color),
    None,
}

/// `stroke:{self}`
#[derive(Copy, Clone, PartialEq)]
pub enum Stroke {
    Color(Color, f32),
    None,
}

/// `fill:{fill};stroke:{stroke};fill-opacity:{opacity};`
#[derive(Copy, Clone, PartialEq)]
pub struct Style {
    pub fill: Fill,
    pub stroke: Stroke,
    pub opacity: f32,
    pub stroke_opacity: f32,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{};{};fill-opacity:{};stroke-opacity:{};",
            self.fill,
            self.stroke,
            self.opacity,
            self.stroke_opacity,
        )
    }
}

impl Style {
    pub fn default() -> Self {
        Style {
            fill: Fill::Color(black()),
            stroke: Stroke::None,
            opacity: 1.0,
            stroke_opacity: 1.0,
        }
    }
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fill::Color(color) => write!(f, "fill:{}", color),
            Fill::None => write!(f, "fill:none"),
        }
    }
}

impl fmt::Display for Stroke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stroke::Color(color, radius) => write!(f, "stroke:{};stroke-width:{}", color, radius),
            Stroke::None => write!(f, "stroke:none"),
        }
    }
}

impl Into<Fill> for Color {
    fn into(self) -> Fill {
        Fill::Color(self)
    }
}

impl Into<Stroke> for Color {
    fn into(self) -> Stroke {
        Stroke::Color(self, 1.0)
    }
}

/// `<rect x="{x}" y="{y}" width="{w}" height="{h}" ... />`,
#[derive(Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub style: Style,
    pub border_radius: f32,
}

pub fn rectangle<T: Number, U: Number>(x: T, y: T, w: U, h: U) -> Rectangle {
    Rectangle {
        x: x.to_f32(), y: y.to_f32(), w: w.to_f32(), h: h.to_f32(),
        style: Style::default(),
        border_radius: 0.0,
    }
}

impl Rectangle {
    pub fn fill<F>(mut self, fill: F) -> Self
    where F: Into<Fill> {
        self.style.fill = fill.into();
        self
    }

    pub fn stroke<S>(mut self, stroke: S) -> Self
    where S: Into<Stroke> {
        self.style.stroke = stroke.into();
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = opacity;
        self
    }

    pub fn stroke_opacity(mut self, opacity: f32) -> Self {
        self.style.stroke_opacity = opacity;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn border_radius(mut self, r: f32) -> Self {
        self.border_radius = r;
        self
    }

    pub fn offset(mut self, dx: f32, dy: f32) -> Self {
        self.x += dx;
        self.y += dy;
        self
    }

    pub fn inflate<T: Number>(mut self, dx: T, dy: T) -> Self {
        let dx = dx.to_f32();
        let dy = dy.to_f32();
        self.x -= dx;
        self.y -= dy;
        self.w += 2.0 * dx;
        self.h += 2.0 * dy;
        self
    }

    /// Up, Down, Left, Right
    pub fn sides(&self) -> [LineSegment; 4] {
        let Rectangle { x, y, w, h, .. } = *self;
        [
            line_segment(x, y, x+w, y),
            line_segment(x, y+h, x+w, y+h),
            line_segment(x, y, x, y+h),
            line_segment(x+w, y, x+w, y+h)
        ]
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            r#"<rect x="{}" y="{}" width="{}" height="{}" ry="{}" style="{}" />""#,
            self.x, self.y, self.w, self.h,
            self.border_radius,
            self.style,
        )
    }
}

/// `<circle cx="{x}" cy="{y}" r="{radius}" .../>`
#[derive(Copy, Clone, PartialEq)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub style: Style,
}

impl Circle {
    pub fn fill<F>(mut self, fill: F) -> Self
    where F: Into<Fill> {
        self.style.fill = fill.into();
        self
    }

    pub fn stroke<S>(mut self, stroke: S) -> Self
    where S: Into<Stroke> {
        self.style.stroke = stroke.into();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = opacity;
        self
    }

    pub fn stroke_opacity(mut self, opacity: f32) -> Self {
        self.style.stroke_opacity = opacity;
        self
    }

    pub fn offset(mut self, dx: f32, dy: f32) -> Self {
        self.x += dx;
        self.y += dy;
        self
    }

    pub fn inflate(mut self, by: f32) -> Self {
        self.radius += by;
        self
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            r#"<circle cx="{}" cy="{}" r="{}" style="{}" />""#,
            self.x, self.y, self.radius,
            self.style,
        )
    }
}

/// `<path d="..." style="..."/>`
#[derive(Clone, PartialEq)]
pub struct Polygon {
    pub points: Vec<[f32; 2]>,
    pub closed: bool,
    pub style: Style,
}

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"<path d="#)?;
        if self.points.len() > 0 {
            write!(f, "M {} {} ", self.points[0][0], self.points[0][1])?;
            for &p in &self.points[1..] {
                write!(f, "L {} {} ", p[0], p[1])?;
            }
            if self.closed {
                write!(f, "Z")?;
            }
        }
        write!(f, r#"" style="{}"/>"#, self.style)
    }
}

pub fn polygon<T: Copy + Into<[f32; 2]>>(pts: &[T]) ->  Polygon {
    let mut points = Vec::with_capacity(pts.len());
    for p in pts {
        points.push((*p).into());
    }
    Polygon {
        points,
        closed: true,
        style: Style::default(),
    }
}

pub fn triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> Polygon {
    polygon(&[[x1, y1], [x2, y2], [x3, y3]])
}

impl Polygon {
    pub fn open(mut self) -> Self {
        self.closed = false;
        self
    }

    pub fn fill<F>(mut self, fill: F) -> Self
    where F: Into<Fill> {
        self.style.fill = fill.into();
        self
    }

    pub fn stroke<S>(mut self, stroke: S) -> Self
    where S: Into<Stroke> {
        self.style.stroke = stroke.into();
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = opacity;
        self
    }

    pub fn stroke_opacity(mut self, opacity: f32) -> Self {
        self.style.stroke_opacity = opacity;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

/// `<path d="M {x1} {y1} L {x2} {y2}" ... />`
#[derive(Copy, Clone, PartialEq)]
pub struct LineSegment {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
    pub color: Color,
    pub width: f32,
    pub opacity: f32,
}

impl fmt::Display for LineSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            r#"<path d="M {} {} L {} {}" style="stroke:{};stroke-width:{};stroke-opacity:{}"/>"#,
            self.x1, self.y1,
            self.x2, self.y2,
            self.color,
            self.width,
            self.opacity,
        )
    }
}

pub fn line_segment<T: Number, U: Number>(x1: T, y1: U, x2: T, y2: U) -> LineSegment {
    LineSegment {
        x1: x1.to_f32(), y1: y1.to_f32(), x2: x2.to_f32(), y2: y2.to_f32(),
        color: black(),
        width: 1.0,
        opacity: 1.0,
    }
}

impl LineSegment {
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn offset<T: Number, U: Number>(mut self, dx: T, dy: U) -> Self {
        let dx = dx.to_f32();
        let dy = dy.to_f32();
        self.x1 += dx;
        self.y1 += dy;
        self.x2 += dx;
        self.y2 += dy;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn flip_to<T: Number, U: Number>(mut self, new_x: T, new_y: U) -> Self {
        let new_x = new_x.to_f32();
        let new_y = new_y.to_f32();
        if new_x != 0.0 {
            if new_x > self.x1 {
                self.x1 = self.x2;
                self.x2 = new_x;
            } else if new_x < self.x1 {
                self.x2 = self.x1;
                self.x1 = new_x;
            }
        }
        if new_y != 0.0 {
            if new_y > self.y1 {
                self.y1 = self.y2;
                self.y2 = new_y;
            } else if new_y < self.y1 {
                self.y2 = self.y1;
                self.y1 = new_y;
            }
        }
        self
    }
}

/// `<path d="..." />`
#[derive(Clone, PartialEq)]
pub struct Path {
    pub ops: Vec<PathOp>,
    pub style: Style,
}

/// `M {} {} L {} {} ...`
#[derive(Copy, Clone, PartialEq)]
pub enum PathOp {
    MoveTo { x: f32, y: f32 },
    LineTo { x: f32, y: f32 },
    QuadraticTo { ctrl_x: f32, ctrl_y: f32, x: f32, y: f32 },
    CubicTo { ctrl1_x: f32, ctrl1_y: f32, ctrl2_x: f32, ctrl2_y: f32, x: f32, y: f32 },
    Close,
}
impl fmt::Display for PathOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PathOp::MoveTo { x, y } => write!(f, "M {} {} ", x, y),
            PathOp::LineTo { x, y } => write!(f, "L {} {} ", x, y),
            PathOp::QuadraticTo { ctrl_x, ctrl_y, x, y } => write!(f, "Q {} {} {} {} ", ctrl_x, ctrl_y, x, y),
            PathOp::CubicTo { ctrl1_x, ctrl1_y, ctrl2_x, ctrl2_y, x, y } => write!(f, "C {} {} {} {} {} {} ", ctrl1_x, ctrl1_y, ctrl2_x, ctrl2_y, x, y),
            PathOp::Close => write!(f, "Z "),
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"<path d=""#)?;
        for op in &self.ops {
            op.fmt(f)?;
        }
        write!(f, r#"" style="{}" />"#, self.style)
    }
}

impl Path {
    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.ops.push(PathOp::MoveTo { x, y });
        self
    }

    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        self.ops.push(PathOp::LineTo { x, y });
        self
    }

    pub fn quadratic_bezier_to(
        mut self,
        ctrl_x: f32, ctrl_y: f32,
        x: f32, y: f32,
    ) -> Self {
        self.ops.push(PathOp::QuadraticTo { ctrl_x, ctrl_y, x, y });
        self
    }

    pub fn cubic_bezier_to(
        mut self,
        ctrl1_x: f32, ctrl1_y: f32,
        ctrl2_x: f32, ctrl2_y: f32,
        x: f32, y: f32,
    ) -> Self {
        self.ops.push(PathOp::CubicTo { ctrl1_x, ctrl1_y, ctrl2_x, ctrl2_y, x, y });
        self
    }

    pub fn close(mut self) -> Self {
        self.ops.push(PathOp::Close);
        self
    }

    pub fn fill<F>(mut self, fill: F) -> Self
    where F: Into<Fill> {
        self.style.fill = fill.into();
        self
    }

    pub fn stroke<S>(mut self, stroke: S) -> Self
    where S: Into<Stroke> {
        self.style.stroke = stroke.into();
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = opacity;
        self
    }

    pub fn stroke_opacity(mut self, opacity: f32) -> Self {
        self.style.stroke_opacity = opacity;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

pub fn path() -> Path {
    Path {
        ops: Vec::new(),
        style: Style::default(),
    }
}

/// `<text x="{x}" y="{y}" ... > {text} </text>`
#[derive(Clone, PartialEq)]
pub struct Text {
    pub x: f32, pub y: f32,
    pub text: String,
    pub color: Color,
    pub align: Align,
    pub size: f32,
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            r#"<text x="{}" y="{}" style="font-size:{}px;fill:{};{}"> {} </text>"#,
            self.x, self.y,
            self.size,
            self.color,
            self.align,
            self.text,
        )
    }
}

pub fn text<T: Number, U: Number, S: IntoString>(x: T, y: U, txt: S) -> Text {
    Text {
        x: x.to_f32(), y: y.to_f32(),
        text: txt.into(),
        color: black(),
        align: Align::Left,
        size: 10.0,
    }
}

impl Text {
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn offset(mut self, dx: f32, dy: f32) -> Self {
        self.x += dx;
        self.y += dy;
        self
    }
}

pub struct Comment {
    pub text: String,
}

pub fn comment<T: Into<String>>(text: T) -> Comment {
    Comment {
        text: text.into()
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<!-- {} -->", self.text)
    }
}

/// `text-align:{self}`
#[derive(Copy, Clone, PartialEq)]
pub enum Align {
    Left, Right, Center
}

impl fmt::Display for Align {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Align::Left => write!(f, "text-anchor:start;text-align:left;"),
            Align::Right => write!(f, "text-anchor:end;text-align:right;"),
            Align::Center => write!(f, "text-anchor:middle;text-align:center;"),
        }
    }
}

/// `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {y}">`
#[derive(Copy, Clone, PartialEq)]
pub struct BeginSvg<T = f32, U = f32> {
    pub x: U,
    pub y: U,
    pub w: T,
    pub h: T,
}

impl Default for BeginSvg {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        }
    }
}

impl<T: Number> fmt::Display for BeginSvg<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}">"#,
            self.x.to_f32(),
            self.y.to_f32(),
            self.w.to_f32(),
            self.h.to_f32(),
        )
    }
}


/// `</svg>`
#[derive(Copy, Clone, PartialEq)]
pub struct EndSvg;

impl fmt::Display for EndSvg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "</svg>")
    }
}

/// `"    "`
pub struct Indentation {
    pub n: u32,
}

pub fn indent(n: u32) -> Indentation {
    Indentation { n }
}

impl Indentation {
    pub fn push(&mut self) {
        self.n += 1;
    }

    pub fn pop(&mut self) {
        self.n -= 1;
    }
}

impl fmt::Display for Indentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.n {
            write!(f, "    ")?;
        }
        Ok(())
    }
}

#[test]
fn foo() {
    println!("{}", BeginSvg { w: 800.0, h: 600.0, ..Default::default() });
    println!("    {}",
        rectangle(20.0, 50.0, 200.0, 100.0)
            .fill(red())
            .stroke(Stroke::Color(black(), 3.0))
            .border_radius(5.0)
    );
    println!("    {}", text(25.0, 100.0, "Foo!").size(42.0).color(white()));
    println!("{}", EndSvg);
}

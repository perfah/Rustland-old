use wlc:: {Point, Size, Geometry};

pub trait PointExt {
    fn new(x: i32, y: i32) -> Point;
    fn origin() -> Point;
}

impl PointExt for Point {
    fn new(x: i32, y: i32) -> Point {
        Point{
            x, y
        }
    }

    fn origin() -> Point{
        Point{
            x: 0,
            y: 0
        }
    }
}

pub trait SizeExt {
    fn new(w: u32, h: u32) -> Size;
    fn zero() -> Size;
}

impl SizeExt for Size {
    fn new(w: u32, h: u32) -> Size {
        Size{
            w, h
        }
    }

    fn zero() -> Size{
        Size{
            w: 0,
            h: 0
        }
    }
}

pub trait GeometryExt {
    fn new(origin: Point, size: Size) -> Geometry;
    fn zero() -> Geometry;
    fn overlaps_geometry(&self, other_geom: Geometry) -> bool;
    fn contains_point(&self, point: Point) -> bool;
}

impl GeometryExt for Geometry {
    fn new(origin: Point, size: Size) -> Geometry {
        Geometry{
            origin: origin,
            size: size
        }
    }

    fn zero() -> Geometry{
        Geometry::new(Point::origin(), Size::zero())
    }    

    fn overlaps_geometry(&self, other_geom: Geometry) -> bool {
        self.contains_point(other_geom.origin) || self.contains_point( 
            Point{
                x: other_geom.origin.x + other_geom.size.w as i32,
                y: other_geom.origin.y + other_geom.size.h as i32
            }
        )
    }

    fn contains_point(&self, point: Point) -> bool {
        self.origin.x <= point.x && point.x <= self.origin.x + (self.size.w as i32) &&
        self.origin.y <= point.y && point.y <= self.origin.y + (self.size.h as i32)
    }
}
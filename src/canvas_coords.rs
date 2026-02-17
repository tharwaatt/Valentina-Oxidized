#[derive(Clone, Debug, PartialEq)]
pub struct SvgViewBox {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AspectRatioMode {
    /// xMidYMid meet — letterbox (bars on sides/top)
    Meet,
    /// xMidYMid slice — crop to fill
    Slice,
    /// none — stretch, no mapping offset needed
    None,
}

pub struct CoordMapper {
    pub viewbox: SvgViewBox,
    pub preserve_aspect_ratio: AspectRatioMode,
}

impl CoordMapper {
    /// يحول إحداثيات البكسل من الشاشة إلى إحداثيات الـ SVG viewBox
    pub fn to_svg_space(&self, pixel_x: f64, pixel_y: f64, elem_w: f64, elem_h: f64) -> (f64, f64) {
        let vb = &self.viewbox;
        let (scale, offset_x, offset_y) = match self.preserve_aspect_ratio {
            AspectRatioMode::None => {
                let sx = elem_w / vb.width;
                let sy = elem_h / vb.height;
                return (pixel_x / sx + vb.min_x, pixel_y / sy + vb.min_y);
            }
            AspectRatioMode::Meet => {
                let scale = f64::min(elem_w / vb.width, elem_h / vb.height);
                let ox = (elem_w - vb.width * scale) / 2.0;
                let oy = (elem_h - vb.height * scale) / 2.0;
                (scale, ox, oy)
            }
            AspectRatioMode::Slice => {
                let scale = f64::max(elem_w / vb.width, elem_h / vb.height);
                let ox = (elem_w - vb.width * scale) / 2.0;
                let oy = (elem_h - vb.height * scale) / 2.0;
                (scale, ox, oy)
            }
        };

        (
            (pixel_x - offset_x) / scale + vb.min_x,
            (pixel_y - offset_y) / scale + vb.min_y,
        )
    }
}

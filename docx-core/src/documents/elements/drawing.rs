use super::*;
use serde::{ser::*, Serialize};
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Drawing {
    #[serde(flatten)]
    pub data: Option<DrawingData>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawingData {
    Pic(Pic),
    TextBox(TextBox),
}

impl Serialize for DrawingData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DrawingData::Pic(ref pic) => {
                let mut t = serializer.serialize_struct("Pic", 2)?;
                t.serialize_field("type", "pic")?;
                t.serialize_field("data", pic)?;
                t.end()
            }
            DrawingData::TextBox(ref text_box) => {
                let mut t = serializer.serialize_struct("TextBox", 2)?;
                t.serialize_field("type", "textBox")?;
                t.serialize_field("data", text_box)?;
                t.end()
            }
        }
    }
}

impl Drawing {
    pub fn new() -> Drawing {
        Default::default()
    }

    pub fn pic(mut self, pic: Pic) -> Drawing {
        self.data = Some(DrawingData::Pic(pic));
        self
    }

    pub fn text_box(mut self, t: TextBox) -> Drawing {
        self.data = Some(DrawingData::TextBox(t));
        self
    }
}

impl BuildXML for Drawing {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        let b = XMLBuilder::from(stream);
        let mut b = b.open_drawing()?;

        match &self.data {
            Some(DrawingData::Pic(p)) => {
                if let DrawingPositionType::Inline { .. } = p.position_type {
                    b = b.open_wp_inline(
                        &format!("{}", p.dist_t),
                        &format!("{}", p.dist_b),
                        &format!("{}", p.dist_l),
                        &format!("{}", p.dist_r),
                    )?
                } else {
                    b = b
                        .open_wp_anchor(
                            &format!("{}", p.dist_t),
                            &format!("{}", p.dist_b),
                            &format!("{}", p.dist_l),
                            &format!("{}", p.dist_r),
                            "0",
                            if p.simple_pos { "1" } else { "0" },
                            "0",
                            "0",
                            if p.layout_in_cell { "1" } else { "0" },
                            &format!("{}", p.relative_height),
                        )?
                        .simple_pos(
                            &format!("{}", p.simple_pos_x),
                            &format!("{}", p.simple_pos_y),
                        )?
                        .open_position_h(&format!("{}", p.relative_from_h))?;

                    match p.position_h {
                        DrawingPosition::Offset(x) => {
                            let x = format!("{}", x as u32);
                            b = b.pos_offset(&x)?.close()?;
                        }
                        DrawingPosition::Align(x) => {
                            b = b.align(&x.to_string())?.close()?;
                        }
                    }

                    b = b.open_position_v(&format!("{}", p.relative_from_v))?;

                    match p.position_v {
                        DrawingPosition::Offset(y) => {
                            let y = format!("{}", y as u32);
                            b = b.pos_offset(&y)?.close()?;
                        }
                        DrawingPosition::Align(a) => {
                            b = b.align(&a.to_string())?.close()?;
                        }
                    }
                }

                let w = format!("{}", p.size.0);
                let h = format!("{}", p.size.1);
                b = b
                    // Please see 20.4.2.7 extent (Drawing Object Size)
                    // One inch equates to 914400 EMUs and a centimeter is 360000
                    .wp_extent(&w, &h)?
                    .wp_effect_extent("0", "0", "0", "0")?;
                if p.allow_overlap {
                    b = b.wrap_none()?;
                } else if p.position_type == DrawingPositionType::Anchor {
                    b = b.wrap_square("bothSides")?;
                }
                b = b
                    .wp_doc_pr("1", "Figure")?
                    .open_wp_c_nv_graphic_frame_pr()?
                    .a_graphic_frame_locks(
                        "http://schemas.openxmlformats.org/drawingml/2006/main",
                        "1",
                    )?
                    .close()?
                    .open_a_graphic("http://schemas.openxmlformats.org/drawingml/2006/main")?
                    .open_a_graphic_data(
                        "http://schemas.openxmlformats.org/drawingml/2006/picture",
                    )?
                    .add_child(&p.clone())?
                    .close()?
                    .close()?;
            }
            Some(DrawingData::TextBox(t)) => {
                if let DrawingPositionType::Inline { .. } = t.position_type {
                    b = b.open_wp_inline(
                        &format!("{}", t.dist_t),
                        &format!("{}", t.dist_b),
                        &format!("{}", t.dist_l),
                        &format!("{}", t.dist_r),
                    )?
                } else {
                    b = b
                        .open_wp_anchor(
                            &format!("{}", t.dist_t),
                            &format!("{}", t.dist_b),
                            &format!("{}", t.dist_l),
                            &format!("{}", t.dist_r),
                            "0",
                            if t.simple_pos { "1" } else { "0" },
                            "0",
                            "0",
                            if t.layout_in_cell { "1" } else { "0" },
                            &format!("{}", t.relative_height),
                        )?
                        .simple_pos(
                            &format!("{}", t.simple_pos_x),
                            &format!("{}", t.simple_pos_y),
                        )?
                        .open_position_h(&format!("{}", t.relative_from_h))?;

                    match t.position_h {
                        DrawingPosition::Offset(x) => {
                            let x = format!("{}", x as u32);
                            b = b.pos_offset(&x)?.close()?;
                        }
                        DrawingPosition::Align(x) => {
                            b = b.align(&x.to_string())?.close()?;
                        }
                    }

                    b = b.open_position_v(&format!("{}", t.relative_from_v))?;

                    match t.position_v {
                        DrawingPosition::Offset(y) => {
                            let y = format!("{}", y as u32);
                            b = b.pos_offset(&y)?.close()?;
                        }
                        DrawingPosition::Align(a) => {
                            b = b.align(&a.to_string())?.close()?;
                        }
                    }
                }

                let w = format!("{}", t.size.0);
                let h = format!("{}", t.size.1);
                b = b.wp_extent(&w, &h)?.wp_effect_extent("0", "0", "0", "0")?;
                if t.allow_overlap {
                    b = b.wrap_none()?;
                } else if t.position_type == DrawingPositionType::Anchor {
                    b = b.wrap_square("bothSides")?;
                }

                let content = TextBoxContent {
                    children: t.children.clone(),
                    has_numbering: false,
                };
                let wps_text_box = WpsTextBox::new().add_content(content);

                b = b
                    .wp_doc_pr("1", "Text Box")?
                    .open_wp_c_nv_graphic_frame_pr()?
                    .a_graphic_frame_locks(
                        "http://schemas.openxmlformats.org/drawingml/2006/main",
                        "1",
                    )?
                    .close()?
                    .open_a_graphic("http://schemas.openxmlformats.org/drawingml/2006/main")?
                    .open_a_graphic_data(
                        "http://schemas.microsoft.com/office/word/2010/wordprocessingShape",
                    )?
                    .open_wp_shape()?
                    .add_child(&wps_text_box)?
                    .wps_body_pr()?
                    .close()?
                    .close()?
                    .close()?;
            }
            None => {
                unimplemented!()
            }
        }
        b.close()?.close()?.into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_drawing_build_with_pic() {
        let pic = Pic::new_with_dimensions(Vec::new(), 320, 240);
        let d = Drawing::new().pic(pic).build();
        assert_eq!(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing><wp:inline distT="0" distB="0" distL="0" distR="0"><wp:extent cx="3048000" cy="2286000" /><wp:effectExtent b="0" l="0" r="0" t="0" /><wp:docPr id="1" name="Figure" /><wp:cNvGraphicFramePr><a:graphicFrameLocks xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" noChangeAspect="1" /></wp:cNvGraphicFramePr><a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:nvPicPr><pic:cNvPr id="0" name="" /><pic:cNvPicPr><a:picLocks noChangeAspect="1" noChangeArrowheads="1" /></pic:cNvPicPr></pic:nvPicPr><pic:blipFill><a:blip r:embed="rIdImage123" /><a:srcRect /><a:stretch><a:fillRect /></a:stretch></pic:blipFill><pic:spPr bwMode="auto"><a:xfrm rot="0"><a:off x="0" y="0" /><a:ext cx="3048000" cy="2286000" /></a:xfrm><a:prstGeom prst="rect"><a:avLst /></a:prstGeom></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing>"#
        );
    }

    #[test]
    fn test_drawing_build_with_pic_overlap() {
        let pic = Pic::new_with_dimensions(Vec::new(), 320, 240).overlapping();
        let d = Drawing::new().pic(pic).build();
        assert_eq!(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing><wp:inline distT="0" distB="0" distL="0" distR="0"><wp:extent cx="3048000" cy="2286000" /><wp:effectExtent b="0" l="0" r="0" t="0" /><wp:wrapNone /><wp:docPr id="1" name="Figure" /><wp:cNvGraphicFramePr><a:graphicFrameLocks xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" noChangeAspect="1" /></wp:cNvGraphicFramePr><a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:nvPicPr><pic:cNvPr id="0" name="" /><pic:cNvPicPr><a:picLocks noChangeAspect="1" noChangeArrowheads="1" /></pic:cNvPicPr></pic:nvPicPr><pic:blipFill><a:blip r:embed="rIdImage123" /><a:srcRect /><a:stretch><a:fillRect /></a:stretch></pic:blipFill><pic:spPr bwMode="auto"><a:xfrm rot="0"><a:off x="0" y="0" /><a:ext cx="3048000" cy="2286000" /></a:xfrm><a:prstGeom prst="rect"><a:avLst /></a:prstGeom></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing>"#
        );
    }

    #[test]
    fn test_drawing_build_with_pic_align_right() {
        let mut pic = Pic::new_with_dimensions(Vec::new(), 320, 240).floating();
        pic = pic.relative_from_h(RelativeFromHType::Column);
        pic = pic.relative_from_v(RelativeFromVType::Paragraph);
        pic = pic.position_h(DrawingPosition::Align(PicAlign::Right));
        let d = Drawing::new().pic(pic).build();
        assert_eq!(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing><wp:anchor distT="0" distB="0" distL="0" distR="0" simplePos="0" allowOverlap="0" behindDoc="0" locked="0" layoutInCell="0" relativeHeight="190500"><wp:simplePos x="0" y="0" /><wp:positionH relativeFrom="column"><wp:align>right</wp:align></wp:positionH><wp:positionV relativeFrom="paragraph"><wp:posOffset>0</wp:posOffset></wp:positionV><wp:extent cx="3048000" cy="2286000" /><wp:effectExtent b="0" l="0" r="0" t="0" /><wp:wrapSquare wrapText="bothSides" /><wp:docPr id="1" name="Figure" /><wp:cNvGraphicFramePr><a:graphicFrameLocks xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" noChangeAspect="1" /></wp:cNvGraphicFramePr><a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:nvPicPr><pic:cNvPr id="0" name="" /><pic:cNvPicPr><a:picLocks noChangeAspect="1" noChangeArrowheads="1" /></pic:cNvPicPr></pic:nvPicPr><pic:blipFill><a:blip r:embed="rIdImage123" /><a:srcRect /><a:stretch><a:fillRect /></a:stretch></pic:blipFill><pic:spPr bwMode="auto"><a:xfrm rot="0"><a:off x="0" y="0" /><a:ext cx="3048000" cy="2286000" /></a:xfrm><a:prstGeom prst="rect"><a:avLst /></a:prstGeom></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:anchor></w:drawing>"#
        );
    }

    #[test]
    fn test_issue686() {
        let pic = Pic::new_with_dimensions(Vec::new(), 320, 240)
            .size(320 * 9525, 240 * 9525)
            .floating()
            .offset_x(300 * 9525)
            .offset_y(400 * 9525);

        let d = Drawing::new().pic(pic).build();
        assert_eq!(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing><wp:anchor distT="0" distB="0" distL="0" distR="0" simplePos="0" allowOverlap="0" behindDoc="0" locked="0" layoutInCell="0" relativeHeight="190500"><wp:simplePos x="0" y="0" /><wp:positionH relativeFrom="margin"><wp:posOffset>2857500</wp:posOffset></wp:positionH><wp:positionV relativeFrom="margin"><wp:posOffset>3810000</wp:posOffset></wp:positionV><wp:extent cx="3048000" cy="2286000" /><wp:effectExtent b="0" l="0" r="0" t="0" /><wp:wrapSquare wrapText="bothSides" /><wp:docPr id="1" name="Figure" /><wp:cNvGraphicFramePr><a:graphicFrameLocks xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" noChangeAspect="1" /></wp:cNvGraphicFramePr><a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:nvPicPr><pic:cNvPr id="0" name="" /><pic:cNvPicPr><a:picLocks noChangeAspect="1" noChangeArrowheads="1" /></pic:cNvPicPr></pic:nvPicPr><pic:blipFill><a:blip r:embed="rIdImage123" /><a:srcRect /><a:stretch><a:fillRect /></a:stretch></pic:blipFill><pic:spPr bwMode="auto"><a:xfrm rot="0"><a:off x="0" y="0" /><a:ext cx="3048000" cy="2286000" /></a:xfrm><a:prstGeom prst="rect"><a:avLst /></a:prstGeom></pic:spPr></pic:pic></a:graphicData></a:graphic></wp:anchor></w:drawing>"#
        );
    }

    #[test]
    fn test_drawing_build_with_textbox() {
        let text_box = TextBox::new();
        let d = Drawing::new().text_box(text_box).build();
        let xml = str::from_utf8(&d).unwrap();
        assert!(xml.contains(r#"<w:drawing>"#));
        assert!(xml.contains(r#"wp:inline"#));
        assert!(xml.contains(r#"<wp:extent cx="952500" cy="952500"#));
        assert!(xml.contains(
            r#"uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape""#
        ));
        assert!(xml.contains(r#"<wps:wsp>"#));
        assert!(xml.contains(r#"<wps:txbx>"#));
        assert!(xml.contains(r#"txbxContent"#));
        assert!(xml.contains(r#"<wps:bodyPr />"#));
        assert!(xml.contains(r#"<wp:docPr id="1" name="Text Box"#));
    }

    #[test]
    fn test_drawing_build_with_textbox_with_paragraph() {
        let text_box = TextBox::new();
        let mut t = text_box;
        t.children
            .push(TextBoxContentChild::Paragraph(Box::new(Paragraph::new())));
        let d = Drawing::new().text_box(t).build();
        let xml = str::from_utf8(&d).unwrap();
        assert!(xml.contains(r#"<w:txbxContent>"#));
        assert!(xml.contains(r#"<w:p "#));
        assert!(xml.contains(r#"<wps:wsp>"#));
        assert!(xml.contains(r#"<wps:txbx>"#));
        assert!(xml.contains(r#"<wps:bodyPr />"#));
    }
}

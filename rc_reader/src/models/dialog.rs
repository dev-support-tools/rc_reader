use serde::{Deserialize, Serialize};
use crate::models::control::Control;
use crate::models::rect::Rect;

use svg::Document;
use svg::node::element::{Path, SVG};
use svg::node::element::path::Data;

use super::font::Font;
use super::resource_file::CodeInfo;
use super::string_table::StringTable;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dialog {
    pub id: String,
    pub styles: Vec<String>,
    pub text: String,
    pub font: Font,
    pub rect: Rect,
    pub controls: Vec<Control>,
    pub code_infos: Vec<CodeInfo>,
    pub header_files: Vec<String>,
    pub reference_string_table_ids: Vec<StringTable>,
}

impl Dialog {
    pub fn create(id: String, styles: Vec<String>, text: String, font: Font, rect: Rect, controls: Vec<Control>, code_info: Vec<CodeInfo>, header_files: Vec<String>) -> Dialog {
        Dialog {
            id: id,
            styles: styles,
            text: text,
            font: font,
            rect: rect,
            controls: controls,
            code_infos: code_info,
            header_files: header_files,
            reference_string_table_ids: Vec::new(),
        }
    }
    pub fn default() -> Dialog {
        Dialog {
            id: String::new(),
            styles: Vec::new(),
            text: String::new(),
            font: Font::default(),
            rect: Rect::default(),
            controls: Vec::new(),
            code_infos: Vec::new(),
            header_files: Vec::new(),
            reference_string_table_ids: Vec::new(),
        }
    }
    
    pub(crate) fn create_svg(&self) -> SVG {
        
        // DialogのSVGを作成
        let mut data = Data::new();
        data = data.move_to((self.rect.x, self.rect.y));
        data = data.line_to((self.rect.x + self.rect.width, self.rect.y));
        data = data.line_to((self.rect.x + self.rect.width, self.rect.y + self.rect.height));
        data = data.line_to((self.rect.x, self.rect.y + self.rect.height));
        data = data.close();
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", data);

        let mut svg = Document::new()
            .set("width", self.rect.width)
            .set("height", self.rect.height)
            .add(path);

        svg // Return the svg variable
    }
}
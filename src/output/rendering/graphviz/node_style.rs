/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/



use strum_macros::IntoStaticStr;

use crate::output::rendering::graphviz::common::{DotTranslatable,GraphvizColor};




#[derive(IntoStaticStr)]
pub enum GvNodeStyleKind {
    Solid,
    Dashed,
    Dotted,
    Bold,
    Rounded,
    Diagonals,
    Filled,
    Striped,
    Wedged
}

impl DotTranslatable for GvNodeStyleKind {
    fn to_dot_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        return as_static_str.to_string().to_lowercase();
    }
}

pub type GvNodeStyle = Vec<GvNodeStyleKind>;

impl DotTranslatable for GvNodeStyle {
    fn to_dot_string(&self) -> String {
        let mut dot_str = "\"".to_string();
        let mut rem_styles = self.len();
        for sty_kind in self {
            dot_str.push_str( &sty_kind.to_dot_string() );
            rem_styles = rem_styles -1;
            if rem_styles > 0 {
                dot_str.push_str( "," );
            }
        }
        dot_str.push_str( "\"" );
        return dot_str;
    }
}


#[derive(IntoStaticStr)]
pub enum GvNodeShape {
    Ellipse,
    Circle,
    Triangle,
    Diamond,
    Trapezium,
    Parallelogram,
    House,
    Pentagon,
    Hexagon,
    Septagon,
    Octagon,
    Rectangle,
    Square,
    InvTriangle,
    InvTrapezium,
    InvHouse,
    Star,
    PlainText
}

impl DotTranslatable for GvNodeShape {
    fn to_dot_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        return as_static_str.to_string().to_lowercase();
    }
}





pub enum GraphvizNodeStyleItem {
    Style(GvNodeStyle),
    Shape(GvNodeShape),
    Label(String),
    Image(String),
    Color(GraphvizColor),
    FillColor(GraphvizColor),
    FontColor(GraphvizColor),
    FontSize(u32),
    FontName(String)
}

impl DotTranslatable for GraphvizNodeStyleItem {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        match self {
            GraphvizNodeStyleItem::Style(node_style) => {
                res.push_str("style=");
                res.push_str(&(node_style.to_dot_string()));
            },
            GraphvizNodeStyleItem::Shape(node_shape) => {
                res.push_str("shape=");
                res.push_str(&(node_shape.to_dot_string()));
            },
            GraphvizNodeStyleItem::Label(label) => {
                res.push_str(&format!("label=\"{}\"",label));
            },
            GraphvizNodeStyleItem::Image(imgpath) => {
                res.push_str(&format!("image=\"{}\"",imgpath));
            },
            GraphvizNodeStyleItem::Color(graphviz_color) => {
                res.push_str("color=");
                res.push_str(&(graphviz_color.to_dot_string()));
            },
            GraphvizNodeStyleItem::FillColor(graphviz_color) => {
                res.push_str("style=filled;fillcolor=");
                res.push_str(&(graphviz_color.to_dot_string()));
            },
            GraphvizNodeStyleItem::FontColor(graphviz_color) => {
                res.push_str("fontcolor=");
                res.push_str(&(graphviz_color.to_dot_string()));
            },
            GraphvizNodeStyleItem::FontSize(size) => {
                res.push_str("fontsize=");
                res.push_str(&(size.to_string()));
            },GraphvizNodeStyleItem::FontName(fname) => {
                res.push_str(&format!("fontname=\"{}\"",fname));
            }
        }
        return res;
    }
}

pub type GraphvizNodeStyle = Vec<GraphvizNodeStyleItem>;

impl DotTranslatable for GraphvizNodeStyle {
    fn to_dot_string(&self) -> String {
        if self.len()==0 {
            return "".to_string();
        }
        let mut res = String::new();
        let mut first : bool = true;
        res.push_str("[");
        for item in self {
            if first {
                first = false;
            } else {
                res.push_str(",");
            }
            res.push_str(&(item.to_dot_string()) );
        }
        res.push_str("]");
        return res;
    }
}

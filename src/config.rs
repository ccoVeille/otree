use std::str::FromStr;

use anyhow::{Context, Result};
use ratatui::style::{Color, Style, Stylize};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_icons")]
    pub icons: Icons,

    #[serde(default = "default_colors")]
    pub colors: Colors,

    #[serde(skip)]
    pub tree_platte: TreePalette,
}

impl Config {
    pub fn default() -> Self {
        Self {
            icons: default_icons(),
            colors: default_colors(),
            tree_platte: TreePalette::default(),
        }
    }

    pub fn validate(&mut self) -> Result<()> {
        let tp = TreePalette {
            name: self
                .colors
                .tree_name
                .parse()
                .context("parse color tree_name")?,
            icon: self
                .colors
                .tree_icon
                .parse()
                .context("parse color tree_icon")?,
            value: self
                .colors
                .tree_value
                .parse()
                .context("parse color tree_value")?,
            null: self
                .colors
                .tree_null
                .parse()
                .context("parse color tree_null")?,
            focus: self
                .colors
                .tree_focus
                .parse()
                .context("parse color tree_null")?,
            border: self
                .colors
                .tree_border
                .parse()
                .context("parse color tree_border")?,
        };
        self.tree_platte = tp;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Icons {
    #[serde(default = "default_icons_str")]
    pub str: String,
    #[serde(default = "default_icons_null")]
    pub null: String,
    #[serde(default = "default_icons_number")]
    pub number: String,
    #[serde(default = "default_icons_object")]
    pub object: String,
    #[serde(default = "default_icons_array")]
    pub array: String,
    #[serde(default = "default_icons_bool")]
    pub bool: String,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    #[serde(default = "default_colors_tree_name")]
    pub tree_name: StyleConfig,
    #[serde(default = "default_colors_tree_icon")]
    pub tree_icon: StyleConfig,
    #[serde(default = "default_colors_tree_value")]
    pub tree_value: StyleConfig,
    #[serde(default = "default_colors_tree_null")]
    pub tree_null: StyleConfig,
    #[serde(default = "default_colors_tree_focus")]
    pub tree_focus: StyleConfig,
    #[serde(default = "default_colors_tree_border")]
    pub tree_border: StyleConfig,
}

#[derive(Debug, Deserialize)]
pub struct StyleConfig {
    pub fg: String,
    pub bg: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

#[derive(Debug, Default)]
pub struct TreePalette {
    pub name: Style,
    pub icon: Style,
    pub value: Style,
    pub null: Style,
    pub focus: Style,
    pub border: Style,
}

impl StyleConfig {
    fn new(fg: &str, bg: &str, bold: bool, italic: bool, underline: bool) -> Self {
        Self {
            fg: String::from(fg),
            bg: String::from(bg),
            bold,
            italic,
            underline,
        }
    }

    fn parse(&self) -> Result<Style> {
        let mut style = Style::default();
        if !self.fg.is_empty() {
            let fg = Color::from_str(&self.fg)?;
            style.fg = Some(fg);
        }
        if !self.bg.is_empty() {
            let bg = Color::from_str(&self.bg)?;
            style.bg = Some(bg);
        }

        if self.bold {
            style = style.bold();
        }
        if self.italic {
            style = style.italic();
        }
        if self.underline {
            style = style.underlined();
        }

        Ok(style)
    }
}

fn default_icons() -> Icons {
    Icons {
        str: default_icons_str(),
        array: default_icons_array(),
        bool: default_icons_bool(),
        null: default_icons_null(),
        number: default_icons_number(),
        object: default_icons_object(),
    }
}

fn default_icons_str() -> String {
    String::from("str")
}

fn default_icons_null() -> String {
    String::from("null")
}

fn default_icons_number() -> String {
    String::from("num")
}

fn default_icons_object() -> String {
    String::from("obj")
}

fn default_icons_array() -> String {
    String::from("arr")
}

fn default_icons_bool() -> String {
    String::from("bool")
}

fn default_colors() -> Colors {
    Colors {
        tree_name: default_colors_tree_name(),
        tree_icon: default_colors_tree_icon(),
        tree_value: default_colors_tree_value(),
        tree_null: default_colors_tree_null(),
        tree_focus: default_colors_tree_focus(),
        tree_border: default_colors_tree_border(),
    }
}

fn default_colors_tree_name() -> StyleConfig {
    StyleConfig::new("", "", false, false, false)
}

fn default_colors_tree_icon() -> StyleConfig {
    StyleConfig::new("cyan", "", true, true, false)
}

fn default_colors_tree_value() -> StyleConfig {
    StyleConfig::new("dark_gray", "", false, false, false)
}

fn default_colors_tree_null() -> StyleConfig {
    StyleConfig::new("dark_gray", "", true, false, false)
}

fn default_colors_tree_focus() -> StyleConfig {
    StyleConfig::new("black", "light_green", true, false, false)
}

fn default_colors_tree_border() -> StyleConfig {
    StyleConfig::new("", "", false, false, false)
}

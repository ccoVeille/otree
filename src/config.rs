use ratatui::style::Style;

pub struct Config {
    pub icons: Icons,
    pub tree_platte: TreePalette,
}

pub struct Icons {
    pub str: String,
    pub null: String,
    pub number: String,
    pub object: String,
    pub array: String,
    pub bool: String,
}

struct Colors {}

pub struct TreePalette {
    pub name: Style,
    pub icon: Style,
    pub value: Style,
    pub null: Style,
}



use bevy::prelude::*;




pub fn main_style() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }
}

pub fn side_panel_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(20.0),
        height: Val::Percent(100.0),
        ..default()
    }
}

pub fn shells_label_style() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.0),
        height: Val::Percent(10.0),
        ..default()
    }
}

pub fn upgrade_item_style() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.0),
        height: Val::Percent(20.0),
        ..default()
    }
}

pub fn upgrade_button_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }
}
















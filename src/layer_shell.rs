use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Window};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};

pub trait LayerShell {
    fn init_layer_shell(&self);
    fn set_layer(&self, layer: Layer);
    fn set_anchor(&self, edge: Edge, anchor_to_edge: bool);
    fn set_margin(&self, edge: Edge, margin_size: i32);
    fn set_keyboard_mode(&self, mode: KeyboardMode);
}

impl<T> LayerShell for T where
    T: IsA<Window>
{
    fn init_layer_shell(&self) where Self: IsA<Window> {
        gtk4_layer_shell::init_for_window(self);
    }
    fn set_layer(&self, layer: Layer) where Self: IsA<Window> {
        gtk4_layer_shell::set_layer(self, layer);
    }
    fn set_anchor(&self, edge: Edge, anchor_to_edge: bool) where Self: IsA<Window> {
        gtk4_layer_shell::set_anchor(self, edge, anchor_to_edge);
    }
    fn set_margin(&self, edge: Edge, margin_size: i32) where Self: IsA<Window> {
        gtk4_layer_shell::set_margin(self, edge, margin_size);
    }
    fn set_keyboard_mode(&self, mode: KeyboardMode) where Self: IsA<Window> {
        gtk4_layer_shell::set_keyboard_mode(self, mode);
    }
}

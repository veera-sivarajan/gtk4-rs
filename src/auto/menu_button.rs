// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Actionable;
use Align;
use ArrowType;
use Bin;
use Buildable;
use Button;
use Container;
use LayoutManager;
use Menu;
use Overflow;
use Popover;
use ReliefStyle;
use ToggleButton;
use Widget;

glib_wrapper! {
    pub struct MenuButton(Object<gtk_sys::GtkMenuButton, gtk_sys::GtkMenuButtonClass, MenuButtonClass>) @extends ToggleButton, Button, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || gtk_sys::gtk_menu_button_get_type(),
    }
}

impl MenuButton {
    pub fn new() -> MenuButton {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_menu_button_new()).unsafe_cast() }
    }
}

impl Default for MenuButton {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct MenuButtonBuilder {
    align_widget: Option<Container>,
    direction: Option<ArrowType>,
    menu_model: Option<gio::MenuModel>,
    popover: Option<Popover>,
    popup: Option<Menu>,
    use_popover: Option<bool>,
    active: Option<bool>,
    icon_name: Option<String>,
    label: Option<String>,
    relief: Option<ReliefStyle>,
    use_underline: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    action_name: Option<String>,
}

impl MenuButtonBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> MenuButton {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref align_widget) = self.align_widget {
            properties.push(("align-widget", align_widget));
        }
        if let Some(ref direction) = self.direction {
            properties.push(("direction", direction));
        }
        if let Some(ref menu_model) = self.menu_model {
            properties.push(("menu-model", menu_model));
        }
        if let Some(ref popover) = self.popover {
            properties.push(("popover", popover));
        }
        if let Some(ref popup) = self.popup {
            properties.push(("popup", popup));
        }
        if let Some(ref use_popover) = self.use_popover {
            properties.push(("use-popover", use_popover));
        }
        if let Some(ref active) = self.active {
            properties.push(("active", active));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref relief) = self.relief {
            properties.push(("relief", relief));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref action_name) = self.action_name {
            properties.push(("action-name", action_name));
        }
        glib::Object::new(MenuButton::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn align_widget<P: IsA<Container>>(mut self, align_widget: &P) -> Self {
        self.align_widget = Some(align_widget.clone().upcast());
        self
    }

    pub fn direction(mut self, direction: ArrowType) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn menu_model<P: IsA<gio::MenuModel>>(mut self, menu_model: &P) -> Self {
        self.menu_model = Some(menu_model.clone().upcast());
        self
    }

    pub fn popover<P: IsA<Popover>>(mut self, popover: &P) -> Self {
        self.popover = Some(popover.clone().upcast());
        self
    }

    pub fn popup<P: IsA<Menu>>(mut self, popup: &P) -> Self {
        self.popup = Some(popup.clone().upcast());
        self
    }

    pub fn use_popover(mut self, use_popover: bool) -> Self {
        self.use_popover = Some(use_popover);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn relief(mut self, relief: ReliefStyle) -> Self {
        self.relief = Some(relief);
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn action_name(mut self, action_name: &str) -> Self {
        self.action_name = Some(action_name.to_string());
        self
    }
}

pub const NONE_MENU_BUTTON: Option<&MenuButton> = None;

pub trait MenuButtonExt: 'static {
    fn get_align_widget(&self) -> Option<Widget>;

    fn get_direction(&self) -> ArrowType;

    fn get_menu_model(&self) -> Option<gio::MenuModel>;

    fn get_popover(&self) -> Option<Popover>;

    fn get_popup(&self) -> Option<Menu>;

    fn get_use_popover(&self) -> bool;

    fn set_align_widget<P: IsA<Widget>>(&self, align_widget: Option<&P>);

    fn set_direction(&self, direction: ArrowType);

    fn set_menu_model<P: IsA<gio::MenuModel>>(&self, menu_model: Option<&P>);

    fn set_popover<P: IsA<Widget>>(&self, popover: Option<&P>);

    fn set_popup<P: IsA<Widget>>(&self, menu: Option<&P>);

    fn set_use_popover(&self, use_popover: bool);

    fn connect_property_align_widget_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MenuButton>> MenuButtonExt for O {
    fn get_align_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_button_get_align_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_direction(&self) -> ArrowType {
        unsafe {
            from_glib(gtk_sys::gtk_menu_button_get_direction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_menu_model(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_button_get_menu_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_popover(&self) -> Option<Popover> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_button_get_popover(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_popup(&self) -> Option<Menu> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_button_get_popup(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_use_popover(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_menu_button_get_use_popover(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_align_widget<P: IsA<Widget>>(&self, align_widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_menu_button_set_align_widget(
                self.as_ref().to_glib_none().0,
                align_widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_direction(&self, direction: ArrowType) {
        unsafe {
            gtk_sys::gtk_menu_button_set_direction(
                self.as_ref().to_glib_none().0,
                direction.to_glib(),
            );
        }
    }

    fn set_menu_model<P: IsA<gio::MenuModel>>(&self, menu_model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_menu_button_set_menu_model(
                self.as_ref().to_glib_none().0,
                menu_model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_popover<P: IsA<Widget>>(&self, popover: Option<&P>) {
        unsafe {
            gtk_sys::gtk_menu_button_set_popover(
                self.as_ref().to_glib_none().0,
                popover.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_popup<P: IsA<Widget>>(&self, menu: Option<&P>) {
        unsafe {
            gtk_sys::gtk_menu_button_set_popup(
                self.as_ref().to_glib_none().0,
                menu.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_use_popover(&self, use_popover: bool) {
        unsafe {
            gtk_sys::gtk_menu_button_set_use_popover(
                self.as_ref().to_glib_none().0,
                use_popover.to_glib(),
            );
        }
    }

    fn connect_property_align_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_align_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuButton>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::align-widget\0".as_ptr() as *const _,
                Some(transmute(
                    notify_align_widget_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuButton>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::direction\0".as_ptr() as *const _,
                Some(transmute(notify_direction_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_model_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuButton>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::menu-model\0".as_ptr() as *const _,
                Some(transmute(notify_menu_model_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popover_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuButton>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::popover\0".as_ptr() as *const _,
                Some(transmute(notify_popover_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuButton>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::popup\0".as_ptr() as *const _,
                Some(transmute(notify_popup_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_popover_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMenuButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MenuButton>,
        {
            let f: &F = &*(f as *const F);
            f(&MenuButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-popover\0".as_ptr() as *const _,
                Some(transmute(notify_use_popover_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MenuButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MenuButton")
    }
}

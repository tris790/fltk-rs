/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
pub type custom_handler_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn Fl_Widget_callback_with_captures(
        arg1: *mut Fl_Widget,
        cb: Fl_Callback,
        arg2: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Text_Display {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Text_Display_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Text_Display;
}
extern "C" {
    pub fn Fl_Text_Display_x(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_y(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_width(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_height(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_label(arg1: *mut Fl_Text_Display) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Text_Display_set_label(
        arg1: *mut Fl_Text_Display,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Text_Display_redraw(arg1: *mut Fl_Text_Display);
}
extern "C" {
    pub fn Fl_Text_Display_show(arg1: *mut Fl_Text_Display);
}
extern "C" {
    pub fn Fl_Text_Display_hide(arg1: *mut Fl_Text_Display);
}
extern "C" {
    pub fn Fl_Text_Display_activate(arg1: *mut Fl_Text_Display);
}
extern "C" {
    pub fn Fl_Text_Display_deactivate(arg1: *mut Fl_Text_Display);
}
extern "C" {
    pub fn Fl_Text_Display_redraw_label(arg1: *mut Fl_Text_Display);
}
extern "C" {
    pub fn Fl_Text_Display_resize(
        arg1: *mut Fl_Text_Display,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Display_tooltip(arg1: *mut Fl_Text_Display) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Text_Display_set_tooltip(
        arg1: *mut Fl_Text_Display,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Text_Display_get_type(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_type(arg1: *mut Fl_Text_Display, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_color(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_color(arg1: *mut Fl_Text_Display, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_label_color(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_label_color(
        arg1: *mut Fl_Text_Display,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Display_label_font(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_label_font(arg1: *mut Fl_Text_Display, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_label_size(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_label_size(arg1: *mut Fl_Text_Display, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_label_type(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_label_type(arg1: *mut Fl_Text_Display, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_box(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_box(arg1: *mut Fl_Text_Display, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_changed(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_changed(arg1: *mut Fl_Text_Display);
}
extern "C" {
    pub fn Fl_Text_Display_clear_changed(arg1: *mut Fl_Text_Display);
}
extern "C" {
    pub fn Fl_Text_Display_align(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_align(arg1: *mut Fl_Text_Display, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_delete(arg1: *mut Fl_Text_Display);
}
extern "C" {
    pub fn Fl_Text_Display_set_image(arg1: *mut Fl_Text_Display, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Text_Display_set_handler(
        self_: *mut *mut Fl_Text_Display,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Text_Display_set_trigger(arg1: *mut Fl_Text_Display, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_init(arg1: *mut Fl_Text_Display);
}
extern "C" {
    pub fn Fl_Text_Display_text_font(arg1: *const Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_text_font(arg1: *mut Fl_Text_Display, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_text_size(arg1: *const Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_text_size(arg1: *mut Fl_Text_Display, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_text_color(arg1: *const Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_set_text_color(arg1: *mut Fl_Text_Display, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_text(arg1: *mut Fl_Text_Display) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Text_Display_set_text(
        arg1: *mut Fl_Text_Display,
        arg2: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Text_Display_append(arg1: *mut Fl_Text_Display, arg2: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Text_Display_buffer_length(arg1: *const Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_scroll(
        arg1: *mut Fl_Text_Display,
        topLineNum: ::std::os::raw::c_int,
        horizOffset: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Display_insert(arg1: *mut Fl_Text_Display, text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Text_Display_set_insert_position(
        arg1: *mut Fl_Text_Display,
        newPos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Display_insert_position(arg1: *const Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_count_lines(
        arg1: *const Fl_Text_Display,
        start: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
        start_pos_is_line_start: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_move_right(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_move_left(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_move_up(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_move_down(arg1: *mut Fl_Text_Display) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Display_remove(
        self_: *mut Fl_Text_Display,
        start: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Display_show_cursor(arg1: *mut Fl_Text_Display, boolean: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Display_set_style_table_entry(
        self_: *mut Fl_Text_Display,
        color: *mut ::std::os::raw::c_uint,
        font: *mut ::std::os::raw::c_int,
        fontsz: *mut ::std::os::raw::c_int,
        sz: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Display_set_cursor_style(
        arg1: *mut Fl_Text_Display,
        style: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Display_set_cursor_color(
        arg1: *mut Fl_Text_Display,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Display_set_scrollbar_width(
        arg1: *mut Fl_Text_Display,
        width: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Display_set_scrollbar_size(
        arg1: *mut Fl_Text_Display,
        newSize: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Display_set_scrollbar_align(
        arg1: *mut Fl_Text_Display,
        align: ::std::os::raw::c_int,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Text_Editor {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Text_Editor_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Text_Editor;
}
extern "C" {
    pub fn Fl_Text_Editor_x(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_y(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_width(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_height(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_label(arg1: *mut Fl_Text_Editor) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Text_Editor_set_label(
        arg1: *mut Fl_Text_Editor,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Text_Editor_redraw(arg1: *mut Fl_Text_Editor);
}
extern "C" {
    pub fn Fl_Text_Editor_show(arg1: *mut Fl_Text_Editor);
}
extern "C" {
    pub fn Fl_Text_Editor_hide(arg1: *mut Fl_Text_Editor);
}
extern "C" {
    pub fn Fl_Text_Editor_activate(arg1: *mut Fl_Text_Editor);
}
extern "C" {
    pub fn Fl_Text_Editor_deactivate(arg1: *mut Fl_Text_Editor);
}
extern "C" {
    pub fn Fl_Text_Editor_redraw_label(arg1: *mut Fl_Text_Editor);
}
extern "C" {
    pub fn Fl_Text_Editor_resize(
        arg1: *mut Fl_Text_Editor,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Editor_tooltip(arg1: *mut Fl_Text_Editor) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Text_Editor_set_tooltip(
        arg1: *mut Fl_Text_Editor,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Text_Editor_get_type(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_type(arg1: *mut Fl_Text_Editor, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_color(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_color(arg1: *mut Fl_Text_Editor, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_label_color(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_label_color(arg1: *mut Fl_Text_Editor, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_label_font(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_label_font(arg1: *mut Fl_Text_Editor, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_label_size(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_label_size(arg1: *mut Fl_Text_Editor, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_label_type(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_label_type(arg1: *mut Fl_Text_Editor, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_box(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_box(arg1: *mut Fl_Text_Editor, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_changed(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_changed(arg1: *mut Fl_Text_Editor);
}
extern "C" {
    pub fn Fl_Text_Editor_clear_changed(arg1: *mut Fl_Text_Editor);
}
extern "C" {
    pub fn Fl_Text_Editor_align(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_align(arg1: *mut Fl_Text_Editor, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_delete(arg1: *mut Fl_Text_Editor);
}
extern "C" {
    pub fn Fl_Text_Editor_set_image(arg1: *mut Fl_Text_Editor, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Text_Editor_set_handler(
        self_: *mut *mut Fl_Text_Editor,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Text_Editor_set_trigger(arg1: *mut Fl_Text_Editor, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_init(arg1: *mut Fl_Text_Editor);
}
extern "C" {
    pub fn Fl_Text_Editor_text_font(arg1: *const Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_text_font(arg1: *mut Fl_Text_Editor, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_text_size(arg1: *const Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_text_size(arg1: *mut Fl_Text_Editor, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_text_color(arg1: *const Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_set_text_color(arg1: *mut Fl_Text_Editor, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_text(arg1: *mut Fl_Text_Editor) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Text_Editor_set_text(arg1: *mut Fl_Text_Editor, arg2: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Text_Editor_append(arg1: *mut Fl_Text_Editor, arg2: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Text_Editor_buffer_length(arg1: *const Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_scroll(
        arg1: *mut Fl_Text_Editor,
        topLineNum: ::std::os::raw::c_int,
        horizOffset: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Editor_insert(arg1: *mut Fl_Text_Editor, text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Text_Editor_set_insert_position(
        arg1: *mut Fl_Text_Editor,
        newPos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Editor_insert_position(arg1: *const Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_count_lines(
        arg1: *const Fl_Text_Editor,
        start: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
        start_pos_is_line_start: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_move_right(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_move_left(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_move_up(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_move_down(arg1: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Text_Editor_remove(
        self_: *mut Fl_Text_Editor,
        start: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Editor_show_cursor(arg1: *mut Fl_Text_Editor, boolean: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_set_style_table_entry(
        self_: *mut Fl_Text_Editor,
        color: *mut ::std::os::raw::c_uint,
        font: *mut ::std::os::raw::c_int,
        fontsz: *mut ::std::os::raw::c_int,
        sz: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Editor_set_cursor_style(arg1: *mut Fl_Text_Editor, style: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_set_cursor_color(arg1: *mut Fl_Text_Editor, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Text_Editor_set_scrollbar_width(
        arg1: *mut Fl_Text_Editor,
        width: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Editor_set_scrollbar_size(
        arg1: *mut Fl_Text_Editor,
        newSize: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Text_Editor_set_scrollbar_align(
        arg1: *mut Fl_Text_Editor,
        align: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn kf_copy(e: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn kf_cut(e: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn kf_paste(e: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn kf_undo(e: *mut Fl_Text_Editor) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Simple_Terminal {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Simple_Terminal_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Simple_Terminal;
}
extern "C" {
    pub fn Fl_Simple_Terminal_x(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_y(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_width(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_height(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_label(arg1: *mut Fl_Simple_Terminal)
        -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_label(
        arg1: *mut Fl_Simple_Terminal,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_redraw(arg1: *mut Fl_Simple_Terminal);
}
extern "C" {
    pub fn Fl_Simple_Terminal_show(arg1: *mut Fl_Simple_Terminal);
}
extern "C" {
    pub fn Fl_Simple_Terminal_hide(arg1: *mut Fl_Simple_Terminal);
}
extern "C" {
    pub fn Fl_Simple_Terminal_activate(arg1: *mut Fl_Simple_Terminal);
}
extern "C" {
    pub fn Fl_Simple_Terminal_deactivate(arg1: *mut Fl_Simple_Terminal);
}
extern "C" {
    pub fn Fl_Simple_Terminal_redraw_label(arg1: *mut Fl_Simple_Terminal);
}
extern "C" {
    pub fn Fl_Simple_Terminal_resize(
        arg1: *mut Fl_Simple_Terminal,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_tooltip(
        arg1: *mut Fl_Simple_Terminal,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_tooltip(
        arg1: *mut Fl_Simple_Terminal,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_get_type(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_type(arg1: *mut Fl_Simple_Terminal, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Simple_Terminal_color(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_color(
        arg1: *mut Fl_Simple_Terminal,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_label_color(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_label_color(
        arg1: *mut Fl_Simple_Terminal,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_label_font(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_label_font(
        arg1: *mut Fl_Simple_Terminal,
        font: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_label_size(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_label_size(
        arg1: *mut Fl_Simple_Terminal,
        sz: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_label_type(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_label_type(
        arg1: *mut Fl_Simple_Terminal,
        typ: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_box(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_box(arg1: *mut Fl_Simple_Terminal, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Simple_Terminal_changed(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_changed(arg1: *mut Fl_Simple_Terminal);
}
extern "C" {
    pub fn Fl_Simple_Terminal_clear_changed(arg1: *mut Fl_Simple_Terminal);
}
extern "C" {
    pub fn Fl_Simple_Terminal_align(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_align(arg1: *mut Fl_Simple_Terminal, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Simple_Terminal_delete(arg1: *mut Fl_Simple_Terminal);
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_image(
        arg1: *mut Fl_Simple_Terminal,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_handler(
        self_: *mut *mut Fl_Simple_Terminal,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_trigger(
        arg1: *mut Fl_Simple_Terminal,
        arg2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_init(arg1: *mut Fl_Simple_Terminal);
}
extern "C" {
    pub fn Fl_Simple_Terminal_text_font(arg1: *const Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_text_font(
        arg1: *mut Fl_Simple_Terminal,
        s: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_text_size(arg1: *const Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_text_size(
        arg1: *mut Fl_Simple_Terminal,
        s: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_text_color(arg1: *const Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_text_color(
        arg1: *mut Fl_Simple_Terminal,
        n: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_text(arg1: *mut Fl_Simple_Terminal) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_text(
        arg1: *mut Fl_Simple_Terminal,
        arg2: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_append(
        arg1: *mut Fl_Simple_Terminal,
        arg2: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_buffer_length(
        arg1: *const Fl_Simple_Terminal,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_scroll(
        arg1: *mut Fl_Simple_Terminal,
        topLineNum: ::std::os::raw::c_int,
        horizOffset: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_insert(
        arg1: *mut Fl_Simple_Terminal,
        text: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_insert_position(
        arg1: *mut Fl_Simple_Terminal,
        newPos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_insert_position(
        arg1: *const Fl_Simple_Terminal,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_count_lines(
        arg1: *const Fl_Simple_Terminal,
        start: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
        start_pos_is_line_start: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_move_right(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_move_left(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_move_up(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_move_down(arg1: *mut Fl_Simple_Terminal) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Simple_Terminal_remove(
        self_: *mut Fl_Simple_Terminal,
        start: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_show_cursor(
        arg1: *mut Fl_Simple_Terminal,
        boolean: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_style_table_entry(
        self_: *mut Fl_Simple_Terminal,
        color: *mut ::std::os::raw::c_uint,
        font: *mut ::std::os::raw::c_int,
        fontsz: *mut ::std::os::raw::c_int,
        sz: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_cursor_style(
        arg1: *mut Fl_Simple_Terminal,
        style: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_cursor_color(
        arg1: *mut Fl_Simple_Terminal,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_scrollbar_width(
        arg1: *mut Fl_Simple_Terminal,
        width: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_scrollbar_size(
        arg1: *mut Fl_Simple_Terminal,
        newSize: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Simple_Terminal_set_scrollbar_align(
        arg1: *mut Fl_Simple_Terminal,
        align: ::std::os::raw::c_int,
    );
}

pub extern crate harfbuzz_sys as ffi;
extern crate libc;

use ffi::*;
use libc::{c_int, c_char};
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem;

pub mod blob;

pub use blob::Blob;

pub struct Buffer(*mut hb_buffer_t);

impl Buffer {
    pub fn new() -> Buffer {
        unsafe { Buffer(hb_buffer_create()) }
    }

    pub fn empty() -> Buffer {
        unsafe { Buffer(hb_buffer_get_empty()) }
    }

// {{{
//    pub fn reference(&mut self) -> *mut Buffer {
//        unsafe { hb_buffer_reference(buffer) }
//    }
//
//    pub fn set_user_data(&mut self, key: *mut UserDataKey, data: *mut ::libc::c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t {
//        unsafe { hb_buffer_set_user_data(buffer, key, data, destroy, replace) }
//    }
//
//    pub fn get_user_data(&mut self, key: *mut UserDataKey) -> *mut ::libc::c_void {
//        unsafe { hb_buffer_get_user_data(buffer, key) }
//    }
//
//    pub fn set_content_type(&mut self, content_type: BufferContentType) -> () {
//        unsafe { hb_buffer_set_content_type(buffer, content_type) }
//    }
//
//    pub fn get_content_type(&mut self) -> BufferContentType {
//        unsafe { hb_buffer_get_content_type(buffer) }
//    }
//
//    pub fn set_unicode_funcs(&mut self, unicode_funcs: *mut UnicodeFuncs) -> () {
//        unsafe { hb_buffer_set_unicode_funcs(buffer, unicode_funcs) }
//    }
//
//    pub fn get_unicode_funcs(&mut self) -> *mut UnicodeFuncs {
//        unsafe { hb_buffer_get_unicode_funcs(buffer) }
//    }
//
//    pub fn set_direction(&mut self, direction: Direction) -> () {
//        unsafe { hb_buffer_set_direction(buffer, direction) }
//    }
//
//    pub fn get_direction(&mut self) -> Direction {
//        unsafe { hb_buffer_get_direction(buffer) }
//    }
//
//    pub fn set_script(&mut self, script: Script) -> () {
//        unsafe { hb_buffer_set_script(buffer, script) }
//    }
//
//    pub fn get_script(&mut self) -> Script {
//        unsafe { hb_buffer_get_script(buffer) }
//    }
//
//    pub fn set_language(&mut self, language: Language) -> () {
//        unsafe { hb_buffer_set_language(buffer, language) }
//    }
//
//    pub fn get_language(&mut self) -> Language {
//        unsafe { hb_buffer_get_language(buffer) }
//    }
//
//    pub fn set_segment_properties(&mut self, props: *const SegmentProperties) -> () {
//        unsafe { hb_buffer_set_segment_properties(buffer, props) }
//    }
//
//    pub fn get_segment_properties(&mut self, props: *mut SegmentProperties) -> () {
//        unsafe { hb_buffer_get_segment_properties(buffer, props) }
//    }
//
//    pub fn guess_segment_properties(&mut self) -> () {
//        unsafe { hb_buffer_guess_segment_properties(buffer) }
//    }
//
//    pub fn set_flags(&mut self, flags: BufferFlags) -> () {
//        unsafe { hb_buffer_set_flags(buffer, flags) }
//    }
//
//    pub fn get_flags(&mut self) -> BufferFlags {
//        unsafe { hb_buffer_get_flags(buffer) }
//    }
//
//    pub fn set_cluster_level(&mut self, cluster_level: BufferClusterLevel) -> () {
//        unsafe { hb_buffer_set_cluster_level(buffer, cluster_level) }
//    }
//
//    pub fn get_cluster_level(&mut self) -> BufferClusterLevel {
//        unsafe { hb_buffer_get_cluster_level(buffer) }
//    }
//
//    pub fn set_replacement_codepoint(&mut self, replacement: Codepoint) -> () {
//        unsafe { hb_buffer_set_replacement_codepoint(buffer, replacement) }
//    }
//
//    pub fn get_replacement_codepoint(&mut self) -> Codepoint {
//        unsafe { hb_buffer_get_replacement_codepoint(buffer) }
//    }
//
//    pub fn reset(&mut self) -> () {
//        unsafe { hb_buffer_reset(buffer) }
//    }
//
//    pub fn clear_contents(&mut self) -> () {
//        unsafe { hb_buffer_clear_contents(buffer) }
//    }
//
//    pub fn pre_allocate(&mut self, size: c_uint) -> hb_bool_t {
//        unsafe { hb_buffer_pre_allocate(buffer, size) }
//    }
//
//    pub fn allocation_successful(&mut self) -> hb_bool_t {
//        unsafe { hb_buffer_allocation_successful(buffer) }
//    }
//
//    pub fn reverse(&mut self) -> () {
//        unsafe { hb_buffer_reverse(buffer) }
//    }
//
//    pub fn reverse_range(&mut self, start: c_uint, end: c_uint) -> () {
//        unsafe { hb_buffer_reverse_range(buffer, start, end) }
//    }
//
//    pub fn reverse_clusters(&mut self) -> () {
//        unsafe { hb_buffer_reverse_clusters(buffer) }
//    }
//
//    pub fn add(&mut self, codepoint: Codepoint, cluster: c_uint) -> () {
//        unsafe { hb_buffer_add(buffer, codepoint, cluster) }
//    }
//
//    pub fn add_codepoints(&mut self, text: *const Codepoint, text_length: ::libc::c_int, item_offset: c_uint, item_length: ::libc::c_int) -> () {
//        unsafe { hb_buffer_add_codepoints(buffer, text, text_length, item_offset, item_length) }
//    }
//
//    pub fn set_length(&mut self, length: c_uint) -> hb_bool_t {
//        unsafe { hb_buffer_set_length(buffer, length) }
//    }
//
//    pub fn get_length(&mut self) -> c_uint {
//        unsafe { hb_buffer_get_length(buffer) }
//    }
//
//    pub fn get_glyph_infos(&mut self, length: *mut c_uint) -> *mut GlyphInfo {
//        unsafe { hb_buffer_get_glyph_infos(buffer, length) }
//    }
//
//    pub fn get_glyph_positions(&mut self, length: *mut c_uint) -> *mut GlyphPosition {
//        unsafe { hb_buffer_get_glyph_positions(buffer, length) }
//    }
//
//    pub fn normalize_glyphs(&mut self) -> () {
//        unsafe { hb_buffer_normalize_glyphs(buffer) }
//    }
//
//    pub fn serialize_list_formats() -> *mut *const ::libc::c_char {
//        unsafe { hb_buffer_serialize_list_formats() }
//    }
//
//    pub fn serialize_glyphs(&mut self, start: c_uint, end: c_uint, buf: *mut ::libc::c_char, buf_size: c_uint, buf_consumed: *mut c_uint, font: *mut Font, format: BufferSerializeFormat, flags: BufferSerializeFlags) -> c_uint {
//        unsafe { hb_buffer_serialize_glyphs(buffer, start, end, buf, buf_size, buf_consumed, font, format, flags) }
//    }
//
//    pub fn deserialize_glyphs(&mut self, buf: *const ::libc::c_char, buf_len: ::libc::c_int, end_ptr: *mut *const ::libc::c_char, font: *mut Font, format: BufferSerializeFormat) -> hb_bool_t {
//        unsafe { hb_buffer_deserialize_glyphs(buffer, buf, buf_len, end_ptr, font, format) }
//    }
// }}}

    pub fn raw(&self) -> *mut hb_buffer_t {
        self.0
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { hb_buffer_destroy(self.0) };
    }
}

pub struct Font<'a> {
    inner: *mut hb_font_t,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Font<'a> {
    pub fn new<'f>(face: &Face<'f>) -> Font<'f> {
        Font {
            inner: unsafe { hb_font_create(face.raw()) },
            _phantom: PhantomData,
        }
    }

// {{{
//    pub fn create_sub_font(parent: *mut Font) -> *mut Font {
//        unsafe { hb_font_create_sub_font(parent) }
//    }
//
//    pub fn empty() -> *mut Font {
//        unsafe { hb_font_get_empty() }
//    }
//
//    pub fn reference(font: *mut Font) -> *mut Font {
//        unsafe { hb_font_reference(font) }
//    }
//
//    pub fn get_glyph(font: *mut Font, unicode: Codepoint, variation_selector: Codepoint, glyph: *mut Codepoint) -> hb_bool_t {
//        unsafe { hb_font_get_glyph(font, unicode, variation_selector, glyph) }
//    }
//
//    pub fn get_glyph_h_advance(font: *mut Font, glyph: Codepoint) -> Position {
//        unsafe { hb_font_get_glyph_h_advance(font, glyph) }
//    }
//
//    pub fn get_glyph_v_advance(font: *mut Font, glyph: Codepoint) -> Position {
//        unsafe { hb_font_get_glyph_v_advance(font, glyph) }
//    }
//
//    pub fn get_glyph_h_origin(font: *mut Font, glyph: Codepoint, x: *mut Position, y: *mut Position) -> hb_bool_t {
//        unsafe { hb_font_get_glyph_h_origin(font, glyph, x, y) }
//    }
//
//    pub fn get_glyph_v_origin(font: *mut Font, glyph: Codepoint, x: *mut Position, y: *mut Position) -> hb_bool_t {
//        unsafe { hb_font_get_glyph_v_origin(font, glyph, x, y) }
//    }
//
//    pub fn get_glyph_h_kerning(font: *mut Font, left_glyph: Codepoint, right_glyph: Codepoint) -> Position {
//        unsafe { hb_font_get_glyph_h_kerning(font, left_glyph, right_glyph) }
//    }
//
//    pub fn get_glyph_v_kerning(font: *mut Font, top_glyph: Codepoint, bottom_glyph: Codepoint) -> Position {
//        unsafe { hb_font_get_glyph_v_kerning(font, top_glyph, bottom_glyph) }
//    }
//
//    pub fn get_glyph_extents(font: *mut Font, glyph: Codepoint, extents: *mut GlyphExtents) -> hb_bool_t {
//        unsafe { hb_font_get_glyph_extents(font, glyph, extents) }
//    }
//
//    pub fn get_glyph_contour_point(font: *mut Font, glyph: Codepoint, point_index: c_uint, x: *mut Position, y: *mut Position) -> hb_bool_t {
//        unsafe { hb_font_get_glyph_contour_point(font, glyph, point_index, x, y) }
//    }
//
//    pub fn get_glyph_name(font: *mut Font, glyph: Codepoint, name: *mut ::libc::c_char, size: c_uint) -> hb_bool_t {
//        unsafe { hb_font_get_glyph_name(font, glyph, name, size) }
//    }
//
//    pub fn get_glyph_from_name(font: *mut Font, name: *const ::libc::c_char, len: ::libc::c_int, glyph: *mut Codepoint) -> hb_bool_t {
//        unsafe { hb_font_get_glyph_from_name(font, name, len, glyph) }
//    }
//
//    pub fn get_glyph_advance_for_direction(font: *mut Font, glyph: Codepoint, direction: Direction, x: *mut Position, y: *mut Position) -> () {
//        unsafe { hb_font_get_glyph_advance_for_direction(font, glyph, direction, x, y) }
//    }
//
//    pub fn get_glyph_origin_for_direction(font: *mut Font, glyph: Codepoint, direction: Direction, x: *mut Position, y: *mut Position) -> () {
//        unsafe { hb_font_get_glyph_origin_for_direction(font, glyph, direction, x, y) }
//    }
//
//    pub fn add_glyph_origin_for_direction(font: *mut Font, glyph: Codepoint, direction: Direction, x: *mut Position, y: *mut Position) -> () {
//        unsafe { hb_font_add_glyph_origin_for_direction(font, glyph, direction, x, y) }
//    }
//
//    pub fn subtract_glyph_origin_for_direction(font: *mut Font, glyph: Codepoint, direction: Direction, x: *mut Position, y: *mut Position) -> () {
//        unsafe { hb_font_subtract_glyph_origin_for_direction(font, glyph, direction, x, y) }
//    }
//
//    pub fn get_glyph_kerning_for_direction(font: *mut Font, first_glyph: Codepoint, second_glyph: Codepoint, direction: Direction, x: *mut Position, y: *mut Position) -> () {
//        unsafe { hb_font_get_glyph_kerning_for_direction(font, first_glyph, second_glyph, direction, x, y) }
//    }
//
//    pub fn get_glyph_extents_for_origin(font: *mut Font, glyph: Codepoint, direction: Direction, extents: *mut GlyphExtents) -> hb_bool_t {
//        unsafe { hb_font_get_glyph_extents_for_origin(font, glyph, direction, extents) }
//    }
//
//    pub fn get_glyph_contour_point_for_origin(font: *mut Font, glyph: Codepoint, point_index: c_uint, direction: Direction, x: *mut Position, y: *mut Position) -> hb_bool_t {
//        unsafe { hb_font_get_glyph_contour_point_for_origin(font, glyph, point_index, direction, x, y) }
//    }
//
//    pub fn glyph_to_string(font: *mut Font, glyph: Codepoint, s: *mut ::libc::c_char, size: c_uint) -> () {
//        unsafe { hb_font_glyph_to_string(font, glyph, s, size) }
//    }
//
//    pub fn glyph_from_string(font: *mut Font, s: *const ::libc::c_char, len: ::libc::c_int, glyph: *mut Codepoint) -> hb_bool_t {
//        unsafe { hb_font_glyph_from_string(font, s, len, glyph) }
//    }
//
//    pub fn set_user_data(font: *mut Font, key: *mut UserDataKey, data: *mut ::libc::c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t {
//        unsafe { hb_font_set_user_data(font, key, data, destroy, replace) }
//    }
//
//    pub fn get_user_data(font: *mut Font, key: *mut UserDataKey) -> *mut ::libc::c_void {
//        unsafe { hb_font_get_user_data(font, key) }
//    }
//
//    pub fn make_immutable(font: *mut Font) -> () {
//        unsafe { hb_font_make_immutable(font) }
//    }
//
//    pub fn is_immutable(font: *mut Font) -> hb_bool_t {
//        unsafe { hb_font_is_immutable(font) }
//    }
//
//    pub fn get_parent(font: *mut Font) -> *mut Font {
//        unsafe { hb_font_get_parent(font) }
//    }
//
//    pub fn get_face(font: *mut Font) -> *mut Face {
//        unsafe { hb_font_get_face(font) }
//    }
//
//    pub fn set_funcs(font: *mut Font, klass: *mut FontFuncs, font_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_set_funcs(font, klass, font_data, destroy) }
//    }
//
//    pub fn set_funcs_data(font: *mut Font, font_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_set_funcs_data(font, font_data, destroy) }
//    }
//
//    pub fn set_scale(font: *mut Font, x_scale: ::libc::c_int, y_scale: ::libc::c_int) -> () {
//        unsafe { hb_font_set_scale(font, x_scale, y_scale) }
//    }
//
//    pub fn get_scale(font: *mut Font, x_scale: *mut ::libc::c_int, y_scale: *mut ::libc::c_int) -> () {
//        unsafe { hb_font_get_scale(font, x_scale, y_scale) }
//    }
//
//    pub fn set_ppem(font: *mut Font, x_ppem: c_uint, y_ppem: c_uint) -> () {
//        unsafe { hb_font_set_ppem(font, x_ppem, y_ppem) }
//    }
//
//    pub fn get_ppem(font: *mut Font, x_ppem: *mut c_uint, y_ppem: *mut c_uint) -> () {
//        unsafe { hb_font_get_ppem(font, x_ppem, y_ppem) }
//    }
// }}}

    pub fn raw(&self) -> *mut hb_font_t {
        self.inner
    }
}

impl<'a> Drop for Font<'a> {
    fn drop(&mut self) {
        unsafe { hb_font_destroy(self.inner) }
    }
}

pub fn shape(font: &mut Font, buffer: &mut Buffer, features: &[Feature]) {
    unsafe { hb_shape(font.inner, buffer.0, mem::transmute(&features), features.len() as u32) }
}

//pub fn shape_full(font: *mut Font, buffer: *mut Buffer, features: &[Feature], shaper_list: *const *const ::libc::c_char) -> bool;
//pub fn hb_shape_list_shapers() -> *mut *const ::libc::c_char;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Feature(hb_feature_t);

impl Feature {
    // impl TryFrom<String>
    pub fn from_string(s: &str) -> Result<Feature, ()> {
        let mut feature = Feature(Struct_hb_feature_t::default());
        let ok = unsafe {
            hb_feature_from_string(s.as_ptr() as *const c_char,
                                   s.len() as c_int,
                                   mem::transmute(&mut feature))
        };
        if ok == 0 {
            Err(())
        } else {
            Ok(feature)
        }
    }

    pub fn raw(&self) -> hb_feature_t {
        self.0
    }
}

impl ToString for Feature {
    fn to_string(&self) -> String {
        unsafe {
            // 128 bytes is documented to be "more than enough"
            // src: https://github.com/behdad/harfbuzz/blob/70e72e5f/src/hb-shape.cc#L247
            let mut bytes = vec![0u8; 128];
            hb_feature_to_string(mem::transmute(self),
                                 bytes.as_mut_ptr() as *mut c_char,
                                 128);
            CString::from_vec_unchecked(bytes).into_string().unwrap()
        }
    }
}

pub struct Face<'a> {
    inner: *mut hb_face_t,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Face<'a> {
    pub fn new<'b>(blob: &Blob<'b>, index: u32) -> Face<'b> {
        Face {
            inner: unsafe { hb_face_create(blob.raw(), index) },
            _phantom: PhantomData,
        }
    }

// {{{
//    pub fn create_for_tables(reference_table_func: hb_reference_table_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> *mut Face {
//        unsafe { hb_face_create_for_tables(reference_table_func, user_data, destroy) }
//    }
//
//    pub fn empty() -> *mut Face {
//        unsafe { hb_face_get_empty() }
//    }
//
//    pub fn reference(face: *mut Face) -> *mut Face {
//        unsafe { hb_face_reference(face) }
//    }
//
//    pub fn set_user_data(face: *mut Face, key: *mut UserDataKey, data: *mut ::libc::c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t {
//        unsafe { hb_face_set_user_data(face, key, data, destroy, replace) }
//    }
//
//    pub fn get_user_data(face: *mut Face, key: *mut UserDataKey) -> *mut ::libc::c_void {
//        unsafe { hb_face_get_user_data(face, key) }
//    }
//
//    pub fn make_immutable(face: *mut Face) -> () {
//        unsafe { hb_face_make_immutable(face) }
//    }
//
//    pub fn is_immutable(face: *mut Face) -> hb_bool_t {
//        unsafe { hb_face_is_immutable(face) }
//    }
//
//    pub fn reference_table(face: *mut Face, tag: Tag) -> *mut Blob {
//        unsafe { hb_face_reference_table(face, tag) }
//    }
//
//    pub fn reference_blob(face: *mut Face) -> *mut Blob {
//        unsafe { hb_face_reference_blob(face) }
//    }
//
//    pub fn set_index(face: *mut Face, index: c_uint) -> () {
//        unsafe { hb_face_set_index(face, index) }
//    }
//
//    pub fn get_index(face: *mut Face) -> c_uint {
//        unsafe { hb_face_get_index(face) }
//    }
//
//    pub fn set_upem(face: *mut Face, upem: c_uint) -> () {
//        unsafe { hb_face_set_upem(face, upem) }
//    }
//
//    pub fn get_upem(face: *mut Face) -> c_uint {
//        unsafe { hb_face_get_upem(face) }
//    }
//
//    pub fn set_glyph_count(face: *mut Face, glyph_count: c_uint) -> () {
//        unsafe { hb_face_set_glyph_count(face, glyph_count) }
//    }
//
//    pub fn get_glyph_count(face: *mut Face) -> c_uint {
//        unsafe { hb_face_get_glyph_count(face) }
//    }
// }}}

    pub fn raw(&self) -> *mut hb_face_t {
        self.inner
    }
}

impl<'a> Drop for Face<'a> {
    fn drop(&mut self) {
        unsafe { hb_face_destroy(self.inner) }
    }
}

#[derive(Clone, Copy)]
pub struct Tag(u32);

impl Tag {
    pub fn raw(&self) -> u32 {
        self.0
    }
}

impl<'a> From<&'a str> for Tag {
    fn from(s: &str) -> Tag {
        unsafe { Tag(hb_tag_from_string(s.as_ptr() as *const c_char, s.len() as c_int)) }
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        unsafe {
            let mut bytes = vec![0u8; 4];
            hb_tag_to_string(self.0, bytes.as_mut_ptr() as *mut c_char);
            String::from_utf8_unchecked(bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;

    #[test]
    fn it_works() {
        let _ = Buffer::new();
    }
}

//pub type Codepoint = uint32_t;
//pub type Position = int32_t;
//pub type Mask = uint32_t;
//
//enum Direction {
//    //INVALID = 0,
//    LTR = 4,
//    RTL = 5,
//    TTB = 6,
//    BTT = 7,
//}
//
//impl Direction {
//    pub fn from_string(s: &str) -> Direction {
//        unsafe { hb_direction_from_string(s, s.len()) }
//    }
//
//    pub fn to_string(direction: Direction) -> &str {
//        unsafe { hb_direction_to_string(direction) }
//    }
//}
//
//enum Script {
//    Common = 1517910393,
//    Inherited = 1516858984,
//    Unknown = 1517976186,
//    Arabic = 1098015074,
//    Armenian = 1098018158,
//    Bengali = 1113943655,
//    Cyrillic = 1132032620,
//    Devanagari = 1147500129,
//    Georgian = 1197830002,
//    Greek = 1198679403,
//    Gujarati = 1198877298,
//    Gurmukhi = 1198879349,
//    Hangul = 1214344807,
//    Han = 1214344809,
//    Hebrew = 1214603890,
//    Hiragana = 1214870113,
//    Kannada = 1265525857,
//    Katakana = 1264676449,
//    Lao = 1281453935,
//    Latin = 1281455214,
//    Malayalam = 1298954605,
//    Oriya = 1332902241,
//    Tamil = 1415671148,
//    Telugu = 1415933045,
//    Thai = 1416126825,
//    Tibetan = 1416192628,
//    Bopomofo = 1114599535,
//    Braille = 1114792297,
//    CanadianSyllabics = 1130458739,
//    Cherokee = 1130915186,
//    Ethiopic = 1165256809,
//    Khmer = 1265134962,
//    Mongolian = 1299148391,
//    Myanmar = 1299803506,
//    Ogham = 1332175213,
//    Runic = 1383427698,
//    Sinhala = 1399418472,
//    Syriac = 1400468067,
//    Thaana = 1416126817,
//    Yi = 1500080489,
//    Deseret = 1148416628,
//    Gothic = 1198486632,
//    OldItalic = 1232363884,
//    Buhid = 1114990692,
//    Hanunoo = 1214344815,
//    Tagalog = 1416064103,
//    Tagbanwa = 1415669602,
//    Cypriot = 1131442804,
//    Limbu = 1281977698,
//    LinearB = 1281977954,
//    Osmanya = 1332964705,
//    Shavian = 1399349623,
//    TaiLe = 1415670885,
//    Ugaritic = 1432838514,
//    Buginese = 1114990441,
//    Coptic = 1131376756,
//    Glagolitic = 1198285159,
//    Kharoshthi = 1265131890,
//    NewTaiLue = 1415670901,
//    OldPersian = 1483761007,
//    SylotiNagri = 1400466543,
//    Tifinagh = 1415999079,
//    Balinese = 1113681001,
//    Cuneiform = 1483961720,
//    Nko = 1315663727,
//    PhagsPa = 1349017959,
//    Phoenician = 1349021304,
//    Carian = 1130459753,
//    Cham = 1130914157,
//    KayahLi = 1264675945,
//    Lepcha = 1281716323,
//    Lycian = 1283023721,
//    Lydian = 1283023977,
//    OlChiki = 1332503403,
//    Rejang = 1382706791,
//    Saurashtra = 1398895986,
//    Sundanese = 1400204900,
//    Vai = 1449224553,
//    Avestan = 1098281844,
//    Bamum = 1113681269,
//    EgyptianHieroglyphs = 1164409200,
//    ImperialAramaic = 1098018153,
//    InscriptionalPahlavi = 1349020777,
//    InscriptionalParthian = 1349678185,
//    Javanese = 1247901281,
//    Kaithi = 1265920105,
//    Lisu = 1281979253,
//    MeeteiMayek = 1299473769,
//    OldSouthArabian = 1398895202,
//    OldTurkic = 1332898664,
//    Samaritan = 1398893938,
//    TaiTham = 1281453665,
//    TaiViet = 1415673460,
//    Batak = 1113683051,
//    Brahmi = 1114792296,
//    Mandaic = 1298230884,
//    Chakma = 1130457965,
//    MeroiticCursive = 1298494051,
//    MeroiticHieroglyphs = 1298494063,
//    Miao = 1349284452,
//    Sharada = 1399353956,
//    SoraSompeng = 1399812705,
//    Takri = 1415670642,
//    BassaVah = 1113682803,
//    CaucasianAlbanian = 1097295970,
//    Duployan = 1148547180,
//    Elbasan = 1164730977,
//    Grantha = 1198678382,
//    Khojki = 1265135466,
//    Khudawadi = 1399418468,
//    LinearA = 1281977953,
//    Mahajani = 1298229354,
//    Manichaean = 1298230889,
//    MendeKikakui = 1298493028,
//    Modi = 1299145833,
//    Mro = 1299345263,
//    Nabataean = 1315070324,
//    OldNorthArabian = 1315009122,
//    OldPermic = 1348825709,
//    PahawhHmong = 1215131239,
//    Palmyrene = 1348562029,
//    PauCinHau = 1348564323,
//    PsalterPahlavi = 1349020784,
//    Siddham = 1399415908,
//    Tirhuta = 1416196712,
//    WarangCiti = 1466004065,
//    Ahom = 1097363309,
//    AnatolianHieroglyphs = 1215067511,
//    Hatran = 1214346354,
//    Multani = 1299541108,
//    OldHungarian = 1215655527,
//    Signwriting = 1399287415,
//    Invalid = 0,
//}
//
//impl Script {
//    pub fn from_string(s: *const ::libc::c_char, len: ::libc::c_int) -> Script {
//        unsafe { hb_script_from_string(s, len) }
//    }
//
//    pub fn get_horizontal_direction(script: Script) -> Direction {
//        unsafe { hb_script_get_horizontal_direction(script) }
//    }
//}
//
//enum UnicodeGeneralCategory {
//    Control = 0,
//    Format = 1,
//    Unassigned = 2,
//    PrivateUse = 3,
//    Surrogate = 4,
//    LowercaseLetter = 5,
//    ModifierLetter = 6,
//    OtherLetter = 7,
//    TitlecaseLetter = 8,
//    UppercaseLetter = 9,
//    SpacingMark = 10,
//    EnclosingMark = 11,
//    NonSpacingMark = 12,
//    DecimalNumber = 13,
//    LetterNumber = 14,
//    OtherNumber = 15,
//    ConnectPunctuation = 16,
//    DashPunctuation = 17,
//    ClosePunctuation = 18,
//    FinalPunctuation = 19,
//    InitialPunctuation = 20,
//    OtherPunctuation = 21,
//    OpenPunctuation = 22,
//    CurrencySymbol = 23,
//    ModifierSymbol = 24,
//    MathSymbol = 25,
//    OtherSymbol = 26,
//    LineSeparator = 27,
//    ParagraphSeparator = 28,
//    SpaceSeparator = 29,
//}
//
//impl UnicodeGeneralCategory {
//    pub fn (ufuncs: *mut UnicodeFuncs, unicode: Codepoint) -> UnicodeGeneralCategory {
//        unsafe { hb_unicode_general_category_(ufuncs, unicode) }
//    }
//}
//
//enum UnicodeCombiningClass {
//    NotReordered = 0,
//    Overlay = 1,
//    Nukta = 7,
//    KanaVoicing = 8,
//    Virama = 9,
//    AttachedBelowLeft = 200,
//    AttachedBelow = 202,
//    AttachedAbove = 214,
//    AttachedAboveRight = 216,
//    BelowLeft = 218,
//    Below = 220,
//    BelowRight = 222,
//    Left = 224,
//    Right = 226,
//    AboveLeft = 228,
//    Above = 230,
//    AboveRight = 232,
//    DoubleBelow = 233,
//    DoubleAbove = 234,
//    IotaSubscript = 240,
//    Invalid = 255,
//}
//
//impl UnicodeCombiningClass {
//    pub fn (ufuncs: *mut UnicodeFuncs, unicode: Codepoint) -> UnicodeCombiningClass {
//        unsafe { hb_unicode_combining_class_(ufuncs, unicode) }
//    }
//}
//
//enum BufferContentType {
//    Invalid = 0,
//    Unicode = 1,
//    Glyphs = 2,
//}
//
//enum BufferClusterLevel {
//    MonotoneGraphemes = 0,
//    MonotoneCharacters = 1,
//    Characters = 2,
//    Default = 0,
//}
//
//enum BufferSerializeFormat {
//    Text = 1413830740,
//    Json = 1246973774,
//    Invalid = 0,
//}
//
//impl BufferSerializeFormat {
//    pub fn from_string(str: *const ::libc::c_char, len: ::libc::c_int) -> BufferSerializeFormat {
//        unsafe { hb_buffer_serialize_format_from_string(str, len) }
//    }
//
//    pub fn to_string(format: BufferSerializeFormat) -> *const ::libc::c_char {
//        unsafe { hb_buffer_serialize_format_to_string(format) }
//    }
//}
//
//flags BufferFlags: u32 {
//    const DEFAULT = 0,
//    const BOT = 1,
//    const EOT = 2,
//    const PRESERVE_DEFAULT_IGNORABLES = 4,
//}
//
//flags BufferSerializeFlags: u32 {
//    const DEFAULT = 0,
//    const NO_CLUSTERS = 1,
//    const NO_POSITIONS = 2,
//    const NO_GLYPH_NAMES = 4,
//    const GLYPH_EXTENTS = 8,
//}
//
//pub struct VarInt(*mut hb_var_int_t);
//
//pub struct Language(*mut hb_language_t);
//
//impl Language {
//    pub fn from_string(str: *const ::libc::c_char, len: ::libc::c_int) -> Language {
//        unsafe { hb_language_from_string(str, len) }
//    }
//
//    pub fn to_string(language: Language) -> *const ::libc::c_char {
//        unsafe { hb_language_to_string(language) }
//    }
//
//    pub fn get_default() -> Language {
//        unsafe { hb_language_get_default() }
//    }
//}
//
//pub struct UserDataKey(*mut hb_user_data_key_t);
//
//pub struct UnicodeFuncs(*mut hb_unicode_funcs_t);
//
//impl UnicodeFuncs {
//    pub fn get_default() -> *mut UnicodeFuncs {
//        unsafe { hb_unicode_funcs_get_default() }
//    }
//
//    pub fn create(parent: *mut UnicodeFuncs) -> *mut UnicodeFuncs {
//        unsafe { hb_unicode_funcs_create(parent) }
//    }
//
//    pub fn get_empty() -> *mut UnicodeFuncs {
//        unsafe { hb_unicode_funcs_get_empty() }
//    }
//
//    pub fn reference(ufuncs: *mut UnicodeFuncs) -> *mut UnicodeFuncs {
//        unsafe { hb_unicode_funcs_reference(ufuncs) }
//    }
//
//    pub fn destroy(ufuncs: *mut UnicodeFuncs) -> () {
//        unsafe { hb_unicode_funcs_destroy(ufuncs) }
//    }
//
//    pub fn set_user_data(ufuncs: *mut UnicodeFuncs, key: *mut UserDataKey, data: *mut ::libc::c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t {
//        unsafe { hb_unicode_funcs_set_user_data(ufuncs, key, data, destroy, replace) }
//    }
//
//    pub fn get_user_data(ufuncs: *mut UnicodeFuncs, key: *mut UserDataKey) -> *mut ::libc::c_void {
//        unsafe { hb_unicode_funcs_get_user_data(ufuncs, key) }
//    }
//
//    pub fn make_immutable(ufuncs: *mut UnicodeFuncs) -> () {
//        unsafe { hb_unicode_funcs_make_immutable(ufuncs) }
//    }
//
//    pub fn is_immutable(ufuncs: *mut UnicodeFuncs) -> hb_bool_t {
//        unsafe { hb_unicode_funcs_is_immutable(ufuncs) }
//    }
//
//    pub fn get_parent(ufuncs: *mut UnicodeFuncs) -> *mut UnicodeFuncs {
//        unsafe { hb_unicode_funcs_get_parent(ufuncs) }
//    }
//
//    pub fn set_combining_class_func(ufuncs: *mut UnicodeFuncs, func: hb_unicode_combining_class_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_unicode_funcs_set_combining_class_func(ufuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_eastasian_width_func(ufuncs: *mut UnicodeFuncs, func: hb_unicode_eastasian_width_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_unicode_funcs_set_eastasian_width_func(ufuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_general_category_func(ufuncs: *mut UnicodeFuncs, func: hb_unicode_general_category_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_unicode_funcs_set_general_category_func(ufuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_mirroring_func(ufuncs: *mut UnicodeFuncs, func: hb_unicode_mirroring_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_unicode_funcs_set_mirroring_func(ufuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_script_func(ufuncs: *mut UnicodeFuncs, func: hb_unicode_script_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_unicode_funcs_set_script_func(ufuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_compose_func(ufuncs: *mut UnicodeFuncs, func: hb_unicode_compose_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_unicode_funcs_set_compose_func(ufuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_decompose_func(ufuncs: *mut UnicodeFuncs, func: hb_unicode_decompose_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_unicode_funcs_set_decompose_func(ufuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_decompose_compatibility_func(ufuncs: *mut UnicodeFuncs, func: hb_unicode_decompose_compatibility_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_unicode_funcs_set_decompose_compatibility_func(ufuncs, func, user_data, destroy) }
//    }
//}
//
//pub struct FontFuncs(*mut hb_font_funcs_t);
//
//impl FontFuncs {
//    pub fn create() -> *mut FontFuncs {
//        unsafe { hb_font_funcs_create() }
//    }
//
//    pub fn get_empty() -> *mut FontFuncs {
//        unsafe { hb_font_funcs_get_empty() }
//    }
//
//    pub fn reference(ffuncs: *mut FontFuncs) -> *mut FontFuncs {
//        unsafe { hb_font_funcs_reference(ffuncs) }
//    }
//
//    pub fn destroy(ffuncs: *mut FontFuncs) -> () {
//        unsafe { hb_font_funcs_destroy(ffuncs) }
//    }
//
//    pub fn set_user_data(ffuncs: *mut FontFuncs, key: *mut UserDataKey, data: *mut ::libc::c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t {
//        unsafe { hb_font_funcs_set_user_data(ffuncs, key, data, destroy, replace) }
//    }
//
//    pub fn get_user_data(ffuncs: *mut FontFuncs, key: *mut UserDataKey) -> *mut ::libc::c_void {
//        unsafe { hb_font_funcs_get_user_data(ffuncs, key) }
//    }
//
//    pub fn make_immutable(ffuncs: *mut FontFuncs) -> () {
//        unsafe { hb_font_funcs_make_immutable(ffuncs) }
//    }
//
//    pub fn is_immutable(ffuncs: *mut FontFuncs) -> hb_bool_t {
//        unsafe { hb_font_funcs_is_immutable(ffuncs) }
//    }
//
//    pub fn set_glyph_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_func(ffuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_glyph_h_advance_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_h_advance_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_h_advance_func(ffuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_glyph_v_advance_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_v_advance_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_v_advance_func(ffuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_glyph_h_origin_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_h_origin_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_h_origin_func(ffuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_glyph_v_origin_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_v_origin_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_v_origin_func(ffuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_glyph_h_kerning_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_h_kerning_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_h_kerning_func(ffuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_glyph_v_kerning_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_v_kerning_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_v_kerning_func(ffuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_glyph_extents_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_extents_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_extents_func(ffuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_glyph_contour_point_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_contour_point_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_contour_point_func(ffuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_glyph_name_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_name_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_name_func(ffuncs, func, user_data, destroy) }
//    }
//
//    pub fn set_glyph_from_name_func(ffuncs: *mut FontFuncs, func: hb_font_get_glyph_from_name_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> () {
//        unsafe { hb_font_funcs_set_glyph_from_name_func(ffuncs, func, user_data, destroy) }
//    }
//}
//
//pub struct GlyphExtents(*mut hb_glyph_extents_t);
//
//pub struct GlyphInfo(*mut hb_glyph_info_t);
//
//pub struct GlyphPosition(*mut hb_glyph_position_t);
//
//pub struct SegmentProperties(*mut hb_segment_properties_t);
//
//impl SegmentProperties {
//    pub fn equal(a: *const SegmentProperties, b: *const SegmentProperties) -> hb_bool_t {
//        unsafe { hb_segment_properties_equal(a, b) }
//    }
//
//    pub fn hash(p: *const SegmentProperties) -> c_uint {
//        unsafe { hb_segment_properties_hash(p) }
//    }
//}
//
//pub struct Buffer(*mut hb_buffer_t);
//
//impl Buffer {
//}
//
//pub struct Set(*mut hb_set_t);
//
//impl Set {
//    pub fn create() -> *mut Set {
//        unsafe { hb_set_create() }
//    }
//
//    pub fn get_empty() -> *mut Set {
//        unsafe { hb_set_get_empty() }
//    }
//
//    pub fn reference(set: *mut Set) -> *mut Set {
//        unsafe { hb_set_reference(set) }
//    }
//
//    pub fn destroy(set: *mut Set) -> () {
//        unsafe { hb_set_destroy(set) }
//    }
//
//    pub fn set_user_data(set: *mut Set, key: *mut UserDataKey, data: *mut ::libc::c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t {
//        unsafe { hb_set_set_user_data(set, key, data, destroy, replace) }
//    }
//
//    pub fn get_user_data(set: *mut Set, key: *mut UserDataKey) -> *mut ::libc::c_void {
//        unsafe { hb_set_get_user_data(set, key) }
//    }
//
//    pub fn allocation_successful(set: *const Set) -> hb_bool_t {
//        unsafe { hb_set_allocation_successful(set) }
//    }
//
//    pub fn clear(set: *mut Set) -> () {
//        unsafe { hb_set_clear(set) }
//    }
//
//    pub fn is_empty(set: *const Set) -> hb_bool_t {
//        unsafe { hb_set_is_empty(set) }
//    }
//
//    pub fn has(set: *const Set, codepoint: Codepoint) -> hb_bool_t {
//        unsafe { hb_set_has(set, codepoint) }
//    }
//
//    pub fn add(set: *mut Set, codepoint: Codepoint) -> () {
//        unsafe { hb_set_add(set, codepoint) }
//    }
//
//    pub fn add_range(set: *mut Set, first: Codepoint, last: Codepoint) -> () {
//        unsafe { hb_set_add_range(set, first, last) }
//    }
//
//    pub fn del(set: *mut Set, codepoint: Codepoint) -> () {
//        unsafe { hb_set_del(set, codepoint) }
//    }
//
//    pub fn del_range(set: *mut Set, first: Codepoint, last: Codepoint) -> () {
//        unsafe { hb_set_del_range(set, first, last) }
//    }
//
//    pub fn is_equal(set: *const Set, other: *const Set) -> hb_bool_t {
//        unsafe { hb_set_is_equal(set, other) }
//    }
//
//    pub fn set(set: *mut Set, other: *const Set) -> () {
//        unsafe { hb_set_set(set, other) }
//    }
//
//    pub fn union(set: *mut Set, other: *const Set) -> () {
//        unsafe { hb_set_union(set, other) }
//    }
//
//    pub fn intersect(set: *mut Set, other: *const Set) -> () {
//        unsafe { hb_set_intersect(set, other) }
//    }
//
//    pub fn subtract(set: *mut Set, other: *const Set) -> () {
//        unsafe { hb_set_subtract(set, other) }
//    }
//
//    pub fn symmetric_difference(set: *mut Set, other: *const Set) -> () {
//        unsafe { hb_set_symmetric_difference(set, other) }
//    }
//
//    pub fn invert(set: *mut Set) -> () {
//        unsafe { hb_set_invert(set) }
//    }
//
//    pub fn get_population(set: *const Set) -> c_uint {
//        unsafe { hb_set_get_population(set) }
//    }
//
//    pub fn get_min(set: *const Set) -> Codepoint {
//        unsafe { hb_set_get_min(set) }
//    }
//
//    pub fn get_max(set: *const Set) -> Codepoint {
//        unsafe { hb_set_get_max(set) }
//    }
//
//    pub fn next(set: *const Set, codepoint: *mut Codepoint) -> hb_bool_t {
//        unsafe { hb_set_next(set, codepoint) }
//    }
//
//    pub fn next_range(set: *const Set, first: *mut Codepoint, last: *mut Codepoint) -> hb_bool_t {
//        unsafe { hb_set_next_range(set, first, last) }
//    }
//}
//
//pub struct ShapePlan(*mut hb_shape_plan_t);
//
//impl ShapePlan {
//    pub fn create(face: *mut Face, props: *const SegmentProperties, user_features: *const Feature, num_user_features: c_uint, shaper_list: *const *const ::libc::c_char) -> *mut ShapePlan {
//        unsafe { hb_shape_plan_create(face, props, user_features, num_user_features, shaper_list) }
//    }
//
//    pub fn create_cached(face: *mut Face, props: *const SegmentProperties, user_features: *const Feature, num_user_features: c_uint, shaper_list: *const *const ::libc::c_char) -> *mut ShapePlan {
//        unsafe { hb_shape_plan_create_cached(face, props, user_features, num_user_features, shaper_list) }
//    }
//
//    pub fn get_empty() -> *mut ShapePlan {
//        unsafe { hb_shape_plan_get_empty() }
//    }
//
//    pub fn reference(shape_plan: *mut ShapePlan) -> *mut ShapePlan {
//        unsafe { hb_shape_plan_reference(shape_plan) }
//    }
//
//    pub fn destroy(shape_plan: *mut ShapePlan) -> () {
//        unsafe { hb_shape_plan_destroy(shape_plan) }
//    }
//
//    pub fn set_user_data(shape_plan: *mut ShapePlan, key: *mut UserDataKey, data: *mut ::libc::c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t {
//        unsafe { hb_shape_plan_set_user_data(shape_plan, key, data, destroy, replace) }
//    }
//
//    pub fn get_user_data(shape_plan: *mut ShapePlan, key: *mut UserDataKey) -> *mut ::libc::c_void {
//        unsafe { hb_shape_plan_get_user_data(shape_plan, key) }
//    }
//
//    pub fn execute(shape_plan: *mut ShapePlan, font: *mut Font, buffer: *mut Buffer, features: *const Feature, num_features: c_uint) -> hb_bool_t {
//        unsafe { hb_shape_plan_execute(shape_plan, font, buffer, features, num_features) }
//    }
//
//    pub fn get_shaper(shape_plan: *mut ShapePlan) -> *const ::libc::c_char {
//        unsafe { hb_shape_plan_get_shaper(shape_plan) }
//    }
//}

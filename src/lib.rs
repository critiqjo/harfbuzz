extern crate harfbuzz_sys;

use harfbuzz_sys::*;

pub struct Buffer(*mut hb_buffer_t);

impl Buffer {
    pub fn new() -> Buffer {
        unsafe { Buffer(hb_buffer_create()) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { hb_buffer_destroy(self.0) };
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
//pub type Tag = uint32_t;
//
//impl Tag {
//    pub fn from_string(str: *const ::libc::c_char, len: ::libc::c_int) -> Tag {
//        unsafe { hb_tag_from_string(str, len) }
//    }
//
//    pub fn to_string(tag: Tag, buf: *mut ::libc::c_char) -> () {
//        unsafe { hb_tag_to_string(tag, buf) }
//    }
//}
//
//enum Direction {
//    INVALID = 0,
//    LTR = 4,
//    RTL = 5,
//    TTB = 6,
//    BTT = 7,
//}
//
//impl Direction {
//    pub fn from_string(str: *const ::libc::c_char, len: ::libc::c_int) -> Direction {
//        unsafe { hb_direction_from_string(str, len) }
//    }
//
//    pub fn to_string(direction: Direction) -> *const ::libc::c_char {
//        unsafe { hb_direction_to_string(direction) }
//    }
//}
//
//enum Script {
//    COMMON = 1517910393,
//    INHERITED = 1516858984,
//    UNKNOWN = 1517976186,
//    ARABIC = 1098015074,
//    ARMENIAN = 1098018158,
//    BENGALI = 1113943655,
//    CYRILLIC = 1132032620,
//    DEVANAGARI = 1147500129,
//    GEORGIAN = 1197830002,
//    GREEK = 1198679403,
//    GUJARATI = 1198877298,
//    GURMUKHI = 1198879349,
//    HANGUL = 1214344807,
//    HAN = 1214344809,
//    HEBREW = 1214603890,
//    HIRAGANA = 1214870113,
//    KANNADA = 1265525857,
//    KATAKANA = 1264676449,
//    LAO = 1281453935,
//    LATIN = 1281455214,
//    MALAYALAM = 1298954605,
//    ORIYA = 1332902241,
//    TAMIL = 1415671148,
//    TELUGU = 1415933045,
//    THAI = 1416126825,
//    TIBETAN = 1416192628,
//    BOPOMOFO = 1114599535,
//    BRAILLE = 1114792297,
//    CANADIAN_SYLLABICS = 1130458739,
//    CHEROKEE = 1130915186,
//    ETHIOPIC = 1165256809,
//    KHMER = 1265134962,
//    MONGOLIAN = 1299148391,
//    MYANMAR = 1299803506,
//    OGHAM = 1332175213,
//    RUNIC = 1383427698,
//    SINHALA = 1399418472,
//    SYRIAC = 1400468067,
//    THAANA = 1416126817,
//    YI = 1500080489,
//    DESERET = 1148416628,
//    GOTHIC = 1198486632,
//    OLD_ITALIC = 1232363884,
//    BUHID = 1114990692,
//    HANUNOO = 1214344815,
//    TAGALOG = 1416064103,
//    TAGBANWA = 1415669602,
//    CYPRIOT = 1131442804,
//    LIMBU = 1281977698,
//    LINEAR_B = 1281977954,
//    OSMANYA = 1332964705,
//    SHAVIAN = 1399349623,
//    TAI_LE = 1415670885,
//    UGARITIC = 1432838514,
//    BUGINESE = 1114990441,
//    COPTIC = 1131376756,
//    GLAGOLITIC = 1198285159,
//    KHAROSHTHI = 1265131890,
//    NEW_TAI_LUE = 1415670901,
//    OLD_PERSIAN = 1483761007,
//    SYLOTI_NAGRI = 1400466543,
//    TIFINAGH = 1415999079,
//    BALINESE = 1113681001,
//    CUNEIFORM = 1483961720,
//    NKO = 1315663727,
//    PHAGS_PA = 1349017959,
//    PHOENICIAN = 1349021304,
//    CARIAN = 1130459753,
//    CHAM = 1130914157,
//    KAYAH_LI = 1264675945,
//    LEPCHA = 1281716323,
//    LYCIAN = 1283023721,
//    LYDIAN = 1283023977,
//    OL_CHIKI = 1332503403,
//    REJANG = 1382706791,
//    SAURASHTRA = 1398895986,
//    SUNDANESE = 1400204900,
//    VAI = 1449224553,
//    AVESTAN = 1098281844,
//    BAMUM = 1113681269,
//    EGYPTIAN_HIEROGLYPHS = 1164409200,
//    IMPERIAL_ARAMAIC = 1098018153,
//    INSCRIPTIONAL_PAHLAVI = 1349020777,
//    INSCRIPTIONAL_PARTHIAN = 1349678185,
//    JAVANESE = 1247901281,
//    KAITHI = 1265920105,
//    LISU = 1281979253,
//    MEETEI_MAYEK = 1299473769,
//    OLD_SOUTH_ARABIAN = 1398895202,
//    OLD_TURKIC = 1332898664,
//    SAMARITAN = 1398893938,
//    TAI_THAM = 1281453665,
//    TAI_VIET = 1415673460,
//    BATAK = 1113683051,
//    BRAHMI = 1114792296,
//    MANDAIC = 1298230884,
//    CHAKMA = 1130457965,
//    MEROITIC_CURSIVE = 1298494051,
//    MEROITIC_HIEROGLYPHS = 1298494063,
//    MIAO = 1349284452,
//    SHARADA = 1399353956,
//    SORA_SOMPENG = 1399812705,
//    TAKRI = 1415670642,
//    BASSA_VAH = 1113682803,
//    CAUCASIAN_ALBANIAN = 1097295970,
//    DUPLOYAN = 1148547180,
//    ELBASAN = 1164730977,
//    GRANTHA = 1198678382,
//    KHOJKI = 1265135466,
//    KHUDAWADI = 1399418468,
//    LINEAR_A = 1281977953,
//    MAHAJANI = 1298229354,
//    MANICHAEAN = 1298230889,
//    MENDE_KIKAKUI = 1298493028,
//    MODI = 1299145833,
//    MRO = 1299345263,
//    NABATAEAN = 1315070324,
//    OLD_NORTH_ARABIAN = 1315009122,
//    OLD_PERMIC = 1348825709,
//    PAHAWH_HMONG = 1215131239,
//    PALMYRENE = 1348562029,
//    PAU_CIN_HAU = 1348564323,
//    PSALTER_PAHLAVI = 1349020784,
//    SIDDHAM = 1399415908,
//    TIRHUTA = 1416196712,
//    WARANG_CITI = 1466004065,
//    AHOM = 1097363309,
//    ANATOLIAN_HIEROGLYPHS = 1215067511,
//    HATRAN = 1214346354,
//    MULTANI = 1299541108,
//    OLD_HUNGARIAN = 1215655527,
//    SIGNWRITING = 1399287415,
//    INVALID = 0,
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
//enum MemoryMode {
//    DUPLICATE = 0,
//    READONLY = 1,
//    WRITABLE = 2,
//    READONLY_MAY_MAKE_WRITABLE = 3,
//}
//
//enum UnicodeGeneralCategory {
//    CONTROL = 0,
//    FORMAT = 1,
//    UNASSIGNED = 2,
//    PRIVATE_USE = 3,
//    SURROGATE = 4,
//    LOWERCASE_LETTER = 5,
//    MODIFIER_LETTER = 6,
//    OTHER_LETTER = 7,
//    TITLECASE_LETTER = 8,
//    UPPERCASE_LETTER = 9,
//    SPACING_MARK = 10,
//    ENCLOSING_MARK = 11,
//    NON_SPACING_MARK = 12,
//    DECIMAL_NUMBER = 13,
//    LETTER_NUMBER = 14,
//    OTHER_NUMBER = 15,
//    CONNECT_PUNCTUATION = 16,
//    DASH_PUNCTUATION = 17,
//    CLOSE_PUNCTUATION = 18,
//    FINAL_PUNCTUATION = 19,
//    INITIAL_PUNCTUATION = 20,
//    OTHER_PUNCTUATION = 21,
//    OPEN_PUNCTUATION = 22,
//    CURRENCY_SYMBOL = 23,
//    MODIFIER_SYMBOL = 24,
//    MATH_SYMBOL = 25,
//    OTHER_SYMBOL = 26,
//    LINE_SEPARATOR = 27,
//    PARAGRAPH_SEPARATOR = 28,
//    SPACE_SEPARATOR = 29,
//}
//
//impl UnicodeGeneralCategory {
//    pub fn (ufuncs: *mut UnicodeFuncs, unicode: Codepoint) -> UnicodeGeneralCategory {
//        unsafe { hb_unicode_general_category_(ufuncs, unicode) }
//    }
//}
//
//enum UnicodeCombiningClass {
//    NOT_REORDERED = 0,
//    OVERLAY = 1,
//    NUKTA = 7,
//    KANA_VOICING = 8,
//    VIRAMA = 9,
//    ATTACHED_BELOW_LEFT = 200,
//    ATTACHED_BELOW = 202,
//    ATTACHED_ABOVE = 214,
//    ATTACHED_ABOVE_RIGHT = 216,
//    BELOW_LEFT = 218,
//    BELOW = 220,
//    BELOW_RIGHT = 222,
//    LEFT = 224,
//    RIGHT = 226,
//    ABOVE_LEFT = 228,
//    ABOVE = 230,
//    ABOVE_RIGHT = 232,
//    DOUBLE_BELOW = 233,
//    DOUBLE_ABOVE = 234,
//    IOTA_SUBSCRIPT = 240,
//    INVALID = 255,
//}
//
//impl UnicodeCombiningClass {
//    pub fn (ufuncs: *mut UnicodeFuncs, unicode: Codepoint) -> UnicodeCombiningClass {
//        unsafe { hb_unicode_combining_class_(ufuncs, unicode) }
//    }
//}
//
//enum BufferContentType {
//    INVALID = 0,
//    UNICODE = 1,
//    GLYPHS = 2,
//}
//
//enum BufferClusterLevel {
//    MONOTONE_GRAPHEMES = 0,
//    MONOTONE_CHARACTERS = 1,
//    CHARACTERS = 2,
//    DEFAULT = 0,
//}
//
//enum BufferSerializeFormat {
//    TEXT = 1413830740,
//    JSON = 1246973774,
//    INVALID = 0,
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
//pub struct Blob(*mut hb_blob_t);
//
//impl Blob {
//    pub fn create(data: *const ::libc::c_char, length: ::libc::c_uint, mode: MemoryMode, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> *mut Blob {
//        unsafe { hb_blob_create(data, length, mode, user_data, destroy) }
//    }
//
//    pub fn create_sub_blob(parent: *mut Blob, offset: ::libc::c_uint, length: ::libc::c_uint) -> *mut Blob {
//        unsafe { hb_blob_create_sub_blob(parent, offset, length) }
//    }
//
//    pub fn get_empty() -> *mut Blob {
//        unsafe { hb_blob_get_empty() }
//    }
//
//    pub fn reference(blob: *mut Blob) -> *mut Blob {
//        unsafe { hb_blob_reference(blob) }
//    }
//
//    pub fn destroy(blob: *mut Blob) -> () {
//        unsafe { hb_blob_destroy(blob) }
//    }
//
//    pub fn set_user_data(blob: *mut Blob, key: *mut UserDataKey, data: *mut ::libc::c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t {
//        unsafe { hb_blob_set_user_data(blob, key, data, destroy, replace) }
//    }
//
//    pub fn get_user_data(blob: *mut Blob, key: *mut UserDataKey) -> *mut ::libc::c_void {
//        unsafe { hb_blob_get_user_data(blob, key) }
//    }
//
//    pub fn make_immutable(blob: *mut Blob) -> () {
//        unsafe { hb_blob_make_immutable(blob) }
//    }
//
//    pub fn is_immutable(blob: *mut Blob) -> hb_bool_t {
//        unsafe { hb_blob_is_immutable(blob) }
//    }
//
//    pub fn get_length(blob: *mut Blob) -> ::libc::c_uint {
//        unsafe { hb_blob_get_length(blob) }
//    }
//
//    pub fn get_data(blob: *mut Blob, length: *mut ::libc::c_uint) -> *const ::libc::c_char {
//        unsafe { hb_blob_get_data(blob, length) }
//    }
//
//    pub fn get_data_writable(blob: *mut Blob, length: *mut ::libc::c_uint) -> *mut ::libc::c_char {
//        unsafe { hb_blob_get_data_writable(blob, length) }
//    }
//}
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
//pub struct Face(*mut hb_face_t);
//
//impl Face {
//    pub fn create(blob: *mut Blob, index: ::libc::c_uint) -> *mut Face {
//        unsafe { hb_face_create(blob, index) }
//    }
//
//    pub fn create_for_tables(reference_table_func: hb_reference_table_func_t, user_data: *mut ::libc::c_void, destroy: hb_destroy_func_t) -> *mut Face {
//        unsafe { hb_face_create_for_tables(reference_table_func, user_data, destroy) }
//    }
//
//    pub fn get_empty() -> *mut Face {
//        unsafe { hb_face_get_empty() }
//    }
//
//    pub fn reference(face: *mut Face) -> *mut Face {
//        unsafe { hb_face_reference(face) }
//    }
//
//    pub fn destroy(face: *mut Face) -> () {
//        unsafe { hb_face_destroy(face) }
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
//    pub fn set_index(face: *mut Face, index: ::libc::c_uint) -> () {
//        unsafe { hb_face_set_index(face, index) }
//    }
//
//    pub fn get_index(face: *mut Face) -> ::libc::c_uint {
//        unsafe { hb_face_get_index(face) }
//    }
//
//    pub fn set_upem(face: *mut Face, upem: ::libc::c_uint) -> () {
//        unsafe { hb_face_set_upem(face, upem) }
//    }
//
//    pub fn get_upem(face: *mut Face) -> ::libc::c_uint {
//        unsafe { hb_face_get_upem(face) }
//    }
//
//    pub fn set_glyph_count(face: *mut Face, glyph_count: ::libc::c_uint) -> () {
//        unsafe { hb_face_set_glyph_count(face, glyph_count) }
//    }
//
//    pub fn get_glyph_count(face: *mut Face) -> ::libc::c_uint {
//        unsafe { hb_face_get_glyph_count(face) }
//    }
//}
//
//pub struct Font(*mut hb_font_t);
//
//impl Font {
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
//    pub fn get_glyph_contour_point(font: *mut Font, glyph: Codepoint, point_index: ::libc::c_uint, x: *mut Position, y: *mut Position) -> hb_bool_t {
//        unsafe { hb_font_get_glyph_contour_point(font, glyph, point_index, x, y) }
//    }
//
//    pub fn get_glyph_name(font: *mut Font, glyph: Codepoint, name: *mut ::libc::c_char, size: ::libc::c_uint) -> hb_bool_t {
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
//    pub fn get_glyph_contour_point_for_origin(font: *mut Font, glyph: Codepoint, point_index: ::libc::c_uint, direction: Direction, x: *mut Position, y: *mut Position) -> hb_bool_t {
//        unsafe { hb_font_get_glyph_contour_point_for_origin(font, glyph, point_index, direction, x, y) }
//    }
//
//    pub fn glyph_to_string(font: *mut Font, glyph: Codepoint, s: *mut ::libc::c_char, size: ::libc::c_uint) -> () {
//        unsafe { hb_font_glyph_to_string(font, glyph, s, size) }
//    }
//
//    pub fn glyph_from_string(font: *mut Font, s: *const ::libc::c_char, len: ::libc::c_int, glyph: *mut Codepoint) -> hb_bool_t {
//        unsafe { hb_font_glyph_from_string(font, s, len, glyph) }
//    }
//
//    pub fn create(face: *mut Face) -> *mut Font {
//        unsafe { hb_font_create(face) }
//    }
//
//    pub fn create_sub_font(parent: *mut Font) -> *mut Font {
//        unsafe { hb_font_create_sub_font(parent) }
//    }
//
//    pub fn get_empty() -> *mut Font {
//        unsafe { hb_font_get_empty() }
//    }
//
//    pub fn reference(font: *mut Font) -> *mut Font {
//        unsafe { hb_font_reference(font) }
//    }
//
//    pub fn destroy(font: *mut Font) -> () {
//        unsafe { hb_font_destroy(font) }
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
//    pub fn set_ppem(font: *mut Font, x_ppem: ::libc::c_uint, y_ppem: ::libc::c_uint) -> () {
//        unsafe { hb_font_set_ppem(font, x_ppem, y_ppem) }
//    }
//
//    pub fn get_ppem(font: *mut Font, x_ppem: *mut ::libc::c_uint, y_ppem: *mut ::libc::c_uint) -> () {
//        unsafe { hb_font_get_ppem(font, x_ppem, y_ppem) }
//    }
//}
//
//pub struct FontFuncs(*mut hb_font_funcs_t);
//
//impl FontFuncs {
//
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
//    pub fn hash(p: *const SegmentProperties) -> ::libc::c_uint {
//        unsafe { hb_segment_properties_hash(p) }
//    }
//}
//
//pub struct Buffer(*mut hb_buffer_t);
//
//impl Buffer {
//    pub fn create() -> *mut Buffer {
//        unsafe { hb_buffer_create() }
//    }
//
//    pub fn get_empty() -> *mut Buffer {
//        unsafe { hb_buffer_get_empty() }
//    }
//
//    pub fn reference(buffer: *mut Buffer) -> *mut Buffer {
//        unsafe { hb_buffer_reference(buffer) }
//    }
//
//    pub fn destroy(buffer: *mut Buffer) -> () {
//        unsafe { hb_buffer_destroy(buffer) }
//    }
//
//    pub fn set_user_data(buffer: *mut Buffer, key: *mut UserDataKey, data: *mut ::libc::c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t {
//        unsafe { hb_buffer_set_user_data(buffer, key, data, destroy, replace) }
//    }
//
//    pub fn get_user_data(buffer: *mut Buffer, key: *mut UserDataKey) -> *mut ::libc::c_void {
//        unsafe { hb_buffer_get_user_data(buffer, key) }
//    }
//
//    pub fn set_content_type(buffer: *mut Buffer, content_type: BufferContentType) -> () {
//        unsafe { hb_buffer_set_content_type(buffer, content_type) }
//    }
//
//    pub fn get_content_type(buffer: *mut Buffer) -> BufferContentType {
//        unsafe { hb_buffer_get_content_type(buffer) }
//    }
//
//    pub fn set_unicode_funcs(buffer: *mut Buffer, unicode_funcs: *mut UnicodeFuncs) -> () {
//        unsafe { hb_buffer_set_unicode_funcs(buffer, unicode_funcs) }
//    }
//
//    pub fn get_unicode_funcs(buffer: *mut Buffer) -> *mut UnicodeFuncs {
//        unsafe { hb_buffer_get_unicode_funcs(buffer) }
//    }
//
//    pub fn set_direction(buffer: *mut Buffer, direction: Direction) -> () {
//        unsafe { hb_buffer_set_direction(buffer, direction) }
//    }
//
//    pub fn get_direction(buffer: *mut Buffer) -> Direction {
//        unsafe { hb_buffer_get_direction(buffer) }
//    }
//
//    pub fn set_script(buffer: *mut Buffer, script: Script) -> () {
//        unsafe { hb_buffer_set_script(buffer, script) }
//    }
//
//    pub fn get_script(buffer: *mut Buffer) -> Script {
//        unsafe { hb_buffer_get_script(buffer) }
//    }
//
//    pub fn set_language(buffer: *mut Buffer, language: Language) -> () {
//        unsafe { hb_buffer_set_language(buffer, language) }
//    }
//
//    pub fn get_language(buffer: *mut Buffer) -> Language {
//        unsafe { hb_buffer_get_language(buffer) }
//    }
//
//    pub fn set_segment_properties(buffer: *mut Buffer, props: *const SegmentProperties) -> () {
//        unsafe { hb_buffer_set_segment_properties(buffer, props) }
//    }
//
//    pub fn get_segment_properties(buffer: *mut Buffer, props: *mut SegmentProperties) -> () {
//        unsafe { hb_buffer_get_segment_properties(buffer, props) }
//    }
//
//    pub fn guess_segment_properties(buffer: *mut Buffer) -> () {
//        unsafe { hb_buffer_guess_segment_properties(buffer) }
//    }
//
//    pub fn set_flags(buffer: *mut Buffer, flags: BufferFlags) -> () {
//        unsafe { hb_buffer_set_flags(buffer, flags) }
//    }
//
//    pub fn get_flags(buffer: *mut Buffer) -> BufferFlags {
//        unsafe { hb_buffer_get_flags(buffer) }
//    }
//
//    pub fn set_cluster_level(buffer: *mut Buffer, cluster_level: BufferClusterLevel) -> () {
//        unsafe { hb_buffer_set_cluster_level(buffer, cluster_level) }
//    }
//
//    pub fn get_cluster_level(buffer: *mut Buffer) -> BufferClusterLevel {
//        unsafe { hb_buffer_get_cluster_level(buffer) }
//    }
//
//    pub fn set_replacement_codepoint(buffer: *mut Buffer, replacement: Codepoint) -> () {
//        unsafe { hb_buffer_set_replacement_codepoint(buffer, replacement) }
//    }
//
//    pub fn get_replacement_codepoint(buffer: *mut Buffer) -> Codepoint {
//        unsafe { hb_buffer_get_replacement_codepoint(buffer) }
//    }
//
//    pub fn reset(buffer: *mut Buffer) -> () {
//        unsafe { hb_buffer_reset(buffer) }
//    }
//
//    pub fn clear_contents(buffer: *mut Buffer) -> () {
//        unsafe { hb_buffer_clear_contents(buffer) }
//    }
//
//    pub fn pre_allocate(buffer: *mut Buffer, size: ::libc::c_uint) -> hb_bool_t {
//        unsafe { hb_buffer_pre_allocate(buffer, size) }
//    }
//
//    pub fn allocation_successful(buffer: *mut Buffer) -> hb_bool_t {
//        unsafe { hb_buffer_allocation_successful(buffer) }
//    }
//
//    pub fn reverse(buffer: *mut Buffer) -> () {
//        unsafe { hb_buffer_reverse(buffer) }
//    }
//
//    pub fn reverse_range(buffer: *mut Buffer, start: ::libc::c_uint, end: ::libc::c_uint) -> () {
//        unsafe { hb_buffer_reverse_range(buffer, start, end) }
//    }
//
//    pub fn reverse_clusters(buffer: *mut Buffer) -> () {
//        unsafe { hb_buffer_reverse_clusters(buffer) }
//    }
//
//    pub fn add(buffer: *mut Buffer, codepoint: Codepoint, cluster: ::libc::c_uint) -> () {
//        unsafe { hb_buffer_add(buffer, codepoint, cluster) }
//    }
//
//    pub fn add_codepoints(buffer: *mut Buffer, text: *const Codepoint, text_length: ::libc::c_int, item_offset: ::libc::c_uint, item_length: ::libc::c_int) -> () {
//        unsafe { hb_buffer_add_codepoints(buffer, text, text_length, item_offset, item_length) }
//    }
//
//    pub fn set_length(buffer: *mut Buffer, length: ::libc::c_uint) -> hb_bool_t {
//        unsafe { hb_buffer_set_length(buffer, length) }
//    }
//
//    pub fn get_length(buffer: *mut Buffer) -> ::libc::c_uint {
//        unsafe { hb_buffer_get_length(buffer) }
//    }
//
//    pub fn get_glyph_infos(buffer: *mut Buffer, length: *mut ::libc::c_uint) -> *mut GlyphInfo {
//        unsafe { hb_buffer_get_glyph_infos(buffer, length) }
//    }
//
//    pub fn get_glyph_positions(buffer: *mut Buffer, length: *mut ::libc::c_uint) -> *mut GlyphPosition {
//        unsafe { hb_buffer_get_glyph_positions(buffer, length) }
//    }
//
//    pub fn normalize_glyphs(buffer: *mut Buffer) -> () {
//        unsafe { hb_buffer_normalize_glyphs(buffer) }
//    }
//
//    pub fn serialize_list_formats() -> *mut *const ::libc::c_char {
//        unsafe { hb_buffer_serialize_list_formats() }
//    }
//
//    pub fn serialize_glyphs(buffer: *mut Buffer, start: ::libc::c_uint, end: ::libc::c_uint, buf: *mut ::libc::c_char, buf_size: ::libc::c_uint, buf_consumed: *mut ::libc::c_uint, font: *mut Font, format: BufferSerializeFormat, flags: BufferSerializeFlags) -> ::libc::c_uint {
//        unsafe { hb_buffer_serialize_glyphs(buffer, start, end, buf, buf_size, buf_consumed, font, format, flags) }
//    }
//
//    pub fn deserialize_glyphs(buffer: *mut Buffer, buf: *const ::libc::c_char, buf_len: ::libc::c_int, end_ptr: *mut *const ::libc::c_char, font: *mut Font, format: BufferSerializeFormat) -> hb_bool_t {
//        unsafe { hb_buffer_deserialize_glyphs(buffer, buf, buf_len, end_ptr, font, format) }
//    }
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
//    pub fn get_population(set: *const Set) -> ::libc::c_uint {
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
//pub struct Feature(*mut hb_feature_t);
//
//impl Feature {
//    pub fn from_string(str: *const ::libc::c_char, len: ::libc::c_int, feature: *mut Feature) -> hb_bool_t {
//        unsafe { hb_feature_from_string(str, len, feature) }
//    }
//
//    pub fn to_string(feature: *mut Feature, buf: *mut ::libc::c_char, size: ::libc::c_uint) -> () {
//        unsafe { hb_feature_to_string(feature, buf, size) }
//    }
//}
//
//pub struct ShapePlan(*mut hb_shape_plan_t);
//
//impl ShapePlan {
//    pub fn create(face: *mut Face, props: *const SegmentProperties, user_features: *const Feature, num_user_features: ::libc::c_uint, shaper_list: *const *const ::libc::c_char) -> *mut ShapePlan {
//        unsafe { hb_shape_plan_create(face, props, user_features, num_user_features, shaper_list) }
//    }
//
//    pub fn create_cached(face: *mut Face, props: *const SegmentProperties, user_features: *const Feature, num_user_features: ::libc::c_uint, shaper_list: *const *const ::libc::c_char) -> *mut ShapePlan {
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
//    pub fn execute(shape_plan: *mut ShapePlan, font: *mut Font, buffer: *mut Buffer, features: *const Feature, num_features: ::libc::c_uint) -> hb_bool_t {
//        unsafe { hb_shape_plan_execute(shape_plan, font, buffer, features, num_features) }
//    }
//
//    pub fn get_shaper(shape_plan: *mut ShapePlan) -> *const ::libc::c_char {
//        unsafe { hb_shape_plan_get_shaper(shape_plan) }
//    }
//}

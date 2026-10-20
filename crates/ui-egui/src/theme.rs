//! Design tokens and fonts for the UI.
//!
//! Two appearances: the house theme (dark, steel-blue accent; the default), filled from the
//! shared `craft-tokens` crate, and the light, PowerPoint-like theme as the alternate.

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, Stroke, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Brightness {
    Light,
    /// The house theme (`Tokens::house`).
    #[default]
    Dark,
}

/// Every colour the chrome uses.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub dark: bool,
    /// Window background (title bar, tab row, side panes).
    pub chrome: Color32,
    /// The ribbon card and popovers.
    pub card: Color32,
    pub card_border: Color32,
    /// Canvas around the slide.
    pub canvas: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_faint: Color32,
    pub border: Color32,
    pub separator: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub selected: Color32,
    /// DeckCraft's accent (tabs underline, selected thumbnail, primary buttons).
    pub accent: Color32,
    pub accent_text: Color32,
    pub contextual: Color32,
    pub selection: Color32,
    pub handle_fill: Color32,
    pub handle_stroke: Color32,
    pub guide: Color32,
    pub caret: Color32,
    pub text_selection: Color32,
    pub field_bg: Color32,
    pub shadow: Color32,
    /// "You are here": the current slide's thumbnail. Gold in the house theme, used for nothing else.
    pub current: Color32,
}

impl Tokens {
    pub fn light() -> Self {
        Tokens {
            dark: false,
            chrome: Color32::from_rgb(0xF5, 0xF5, 0xF5),
            card: Color32::WHITE,
            card_border: Color32::from_rgb(0xE3, 0xE3, 0xE3),
            canvas: Color32::from_rgb(0xF0, 0xF0, 0xF0),
            text: Color32::from_rgb(0x24, 0x24, 0x24),
            text_dim: Color32::from_rgb(0x5C, 0x5C, 0x5C),
            text_faint: Color32::from_rgb(0x99, 0x99, 0x99),
            border: Color32::from_rgb(0xD6, 0xD6, 0xD6),
            separator: Color32::from_rgb(0xE1, 0xE1, 0xE1),
            hover: Color32::from_rgb(0xEB, 0xEB, 0xEB),
            pressed: Color32::from_rgb(0xDE, 0xDE, 0xDE),
            selected: Color32::from_rgb(0xE6, 0xE6, 0xE6),
            accent: Color32::from_rgb(0x3D, 0x5A, 0x73),
            accent_text: Color32::WHITE,
            contextual: Color32::from_rgb(0x2F, 0x4A, 0x60),
            selection: Color32::from_rgb(0x7A, 0x7A, 0x7A),
            handle_fill: Color32::WHITE,
            handle_stroke: Color32::from_rgb(0x6E, 0x6E, 0x6E),
            guide: Color32::from_rgb(0x4F, 0x8F, 0xD9),
            caret: Color32::BLACK,
            text_selection: Color32::from_rgba_unmultiplied(0x5A, 0x8F, 0xE0, 90),
            field_bg: Color32::WHITE,
            shadow: Color32::from_black_alpha(28),
            current: Color32::from_rgb(0x3D, 0x5A, 0x73),
        }
    }
    /// The house theme: the shared design tokens (`craft-tokens`), mapped as the design system's
    /// ADOPTION guide says. Translucent web surfaces are flattened onto the opaque card, since egui
    /// can't blur what is behind a panel. The canvas round the slide stays the neutral page colour.
    pub fn house() -> Self {
        use craft_tokens::color as c;
        let card = c::SURFACE_PANEL.over(c::BG_BASE);
        let on_card = |x: craft_tokens::Rgba| Color32::from(x.over(card));
        Tokens {
            dark: true,
            chrome: c::BG_SHEET.into(),
            card: card.into(),
            card_border: on_card(c::BORDER_LINE),
            canvas: c::BG_BASE.into(),
            text: c::TEXT_INK.into(),
            text_dim: c::TEXT_MUTED.into(),
            text_faint: c::TEXT_FAINT.into(),
            border: on_card(c::BORDER_LINE),
            separator: on_card(c::BORDER_LINE_SOFT),
            hover: on_card(c::SURFACE_FILL_2),
            pressed: on_card(c::SURFACE_FILL_3),
            selected: on_card(c::ACCENT_BASE.with_alpha(0.14)),
            accent: c::ACCENT_BASE.into(),
            accent_text: c::TEXT_ON_ACCENT.into(),
            contextual: c::ACCENT_BRIGHT.into(),
            selection: c::ACCENT_BASE.into(),
            handle_fill: c::ACCENT_HIGH.into(),
            handle_stroke: c::ACCENT_LOW.into(),
            guide: c::ACCENT_BRIGHT.into(),
            caret: Color32::BLACK,
            text_selection: c::ACCENT_BASE.with_alpha(0.35).into(),
            field_bg: on_card(c::SURFACE_FILL_1),
            shadow: Color32::from_black_alpha(178),
            current: c::ACCENT_GOLD.into(),
        }
    }
    /// The dark appearance: the house theme.
    pub fn dark() -> Self {
        Self::house()
    }
    pub fn for_brightness(b: Brightness) -> Self {
        match b {
            Brightness::Light => Self::light(),
            Brightness::Dark => Self::dark(),
        }
    }
    pub fn get(ctx: &egui::Context) -> Self {
        if ctx.global_style().visuals.dark_mode { Self::dark() } else { Self::light() }
    }
}

/// Apply tokens to egui's style.
pub fn apply(ctx: &egui::Context, t: &Tokens) {
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.chrome;
    v.window_fill = t.card;
    v.extreme_bg_color = t.field_bg;
    v.faint_bg_color = t.hover;
    v.window_stroke = Stroke::new(1.0, t.card_border);
    v.window_corner_radius = CornerRadius::same(if t.dark { craft_tokens::radius::XL as u8 } else { 10 });
    v.menu_corner_radius = CornerRadius::same(if t.dark { craft_tokens::radius::LG as u8 } else { 8 });
    if t.dark {
        // House: accent at 30% behind selected text, the accent as its stroke.
        v.selection.bg_fill =
            craft_tokens::color::ACCENT_BASE.with_alpha(0.30).over(craft_tokens::color::SURFACE_PANEL.over(craft_tokens::color::BG_BASE)).into();
        v.selection.stroke = Stroke::new(1.0, t.accent);
    } else {
        v.selection.bg_fill = t.accent.gamma_multiply(0.85);
        v.selection.stroke = Stroke::new(1.0, t.accent_text);
    }
    v.hyperlink_color = t.accent;
    v.override_text_color = Some(t.text);
    for w in [&mut v.widgets.noninteractive, &mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open] {
        w.corner_radius = CornerRadius::same(5);
    }
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.separator);
    v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    v.widgets.inactive.bg_fill = t.field_bg;
    v.widgets.inactive.bg_stroke = Stroke::NONE;
    v.widgets.hovered.weak_bg_fill = t.hover;
    v.widgets.hovered.bg_fill = t.hover;
    v.widgets.hovered.bg_stroke = Stroke::NONE;
    v.widgets.active.weak_bg_fill = t.pressed;
    v.widgets.active.bg_fill = t.pressed;
    v.widgets.open.weak_bg_fill = t.pressed;
    v.window_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 18, spread: 0, color: t.shadow };
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 3], blur: 12, spread: 0, color: t.shadow };
    ctx.set_visuals(v);
    let reduce_motion = std::env::var_os("DECKCRAFT_REDUCE_MOTION").is_some();
    ctx.global_style_mut(|s| {
        // House motion: 200 ms, none under reduce-motion.
        s.animation_time = if reduce_motion { 0.0 } else { craft_tokens::motion::DURATION_HOVER.as_secs_f32() };
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(6.0, 3.0);
        s.spacing.interact_size.y = 22.0;
        s.text_styles.insert(egui::TextStyle::Body, egui::FontId::proportional(13.0));
        s.text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(13.0));
        s.text_styles.insert(egui::TextStyle::Small, egui::FontId::proportional(11.0));
        s.text_styles.insert(egui::TextStyle::Heading, egui::FontId::proportional(18.0));
    });
}

/// UI fonts: Inter (from craft-fonts) when embedded, a Japanese UI face for CJK, then egui's;
/// JetBrains Mono (from craft-fonts) heads the monospace family when embedded.
pub fn install_fonts(ctx: &egui::Context) {
    install_fonts_with(ctx, &[]);
}

/// [`install_fonts`], with `extra` faces from the font database after every interface font: the
/// installed or embedded faces that draw text the interface faces lack (Hangul, Simplified
/// Chinese, Devanagari…, #101). See [`crate::uifonts`].
pub fn install_fonts_with(ctx: &egui::Context, extra: &[std::sync::Arc<deckcraft_fonts::FontFace>]) {
    ctx.set_fonts(font_definitions(extra));
}

/// The interface font definitions; see [`install_fonts_with`].
pub fn font_definitions(extra: &[std::sync::Arc<deckcraft_fonts::FontFace>]) -> FontDefinitions {
    let mut defs = FontDefinitions::default();
    let mut names = vec![];
    for (i, f) in deckcraft_fonts::CRAFT_FONTS.iter().enumerate() {
        let wanted = (f.family == "Inter" && (f.style == "Regular" || f.style == "SemiBold" || f.style == "Bold"))
            || (f.family == "BIZ UDPGothic" && f.style == "Regular")
            || (f.family == "Noto Sans Arabic")
            || (f.family == "JetBrains Mono" && f.style == "Regular");
        if !wanted {
            continue;
        }
        let name = format!("craft-{i}-{}-{}", f.family, f.style);
        defs.font_data.insert(name.clone(), std::sync::Arc::new(FontData::from_static(f.bytes)));
        names.push((f.family, f.style, name));
    }
    let regular: Vec<String> =
        names.iter().filter(|(fam, st, _)| !(*fam == "Inter" && *st != "Regular") && *fam != "JetBrains Mono").map(|(_, _, n)| n.clone()).collect();
    // The house monospace face first; egui's Hack and the proportional faces stay as fallbacks.
    let mono: Vec<String> = names.iter().filter(|(fam, _, _)| *fam == "JetBrains Mono").map(|(_, _, n)| n.clone()).collect();
    if let Some(list) = defs.families.get_mut(&FontFamily::Monospace) {
        for (k, n) in mono.iter().enumerate() {
            list.insert(k, n.clone());
        }
    }
    if let Some(list) = defs.families.get_mut(&FontFamily::Proportional) {
        for (k, n) in regular.iter().enumerate() {
            list.insert(k, n.clone());
        }
    }
    // A bold family for headings and selected tabs.
    let mut bold: Vec<String> = names.iter().filter(|(fam, st, _)| *fam == "Inter" && *st == "SemiBold").map(|(_, _, n)| n.clone()).collect();
    bold.extend(defs.families.get(&FontFamily::Proportional).cloned().unwrap_or_default());
    defs.families.insert(FontFamily::Name("bold".into()), bold);
    add_fallback_faces(&mut defs, extra);
    // The Japanese interface face (the first one when Inter isn't embedded) hints its en and em
    // dashes, bars thinner than a pixel, down to nothing, so they drew as gaps in interface text;
    // unhinted, they draw.
    for (name, d) in defs.font_data.iter_mut() {
        if name.contains("BIZ UDPGothic") {
            let mut data = (**d).clone();
            data.tweak.hinting = Some(false);
            *d = std::sync::Arc::new(data);
        }
    }
    defs
}

/// Add `faces` after every other font of each interface family (the last fallbacks).
pub fn add_fallback_faces(defs: &mut FontDefinitions, faces: &[std::sync::Arc<deckcraft_fonts::FontFace>]) {
    for f in faces {
        let name = format!("fallback {} {} {}", f.family, f.style, f.id());
        if defs.font_data.contains_key(&name) {
            continue;
        }
        let mut data = FontData::from_owned(f.data().to_vec());
        data.index = f.index();
        defs.font_data.insert(name.clone(), std::sync::Arc::new(data));
        for list in defs.families.values_mut() {
            list.push(name.clone());
        }
    }
}

pub fn bold(size: f32) -> egui::FontId {
    egui::FontId::new(size, FontFamily::Name("bold".into()))
}

pub fn font(size: f32) -> egui::FontId {
    egui::FontId::proportional(size)
}

pub fn to_color32(c: deckcraft_color::Rgba) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

#[cfg(test)]
mod tests {
    /// Each character of `text` laid out in the interface fonts at interface sizes, and whether it
    /// has ink (a glyph image).
    fn drawn(text: &str) -> Vec<(char, f32, bool)> {
        let ctx = egui::Context::default();
        ctx.set_fonts(super::font_definitions(&[]));
        let mut o = ctx.run_ui(Default::default(), |_| {});
        o.textures_delta.clear();
        let mut out = vec![];
        ctx.fonts_mut(|f| {
            for size in [11.0, 12.0, 13.0, 16.0] {
                for family in [egui::FontFamily::Proportional, egui::FontFamily::Name("bold".into())] {
                    let g = f.layout_no_wrap(text.to_string(), egui::FontId::new(size, family), egui::Color32::WHITE);
                    for gl in g.rows.iter().flat_map(|r| &r.glyphs) {
                        out.push((gl.chr, size, gl.uv_rect.size.x > 0.0 && gl.uv_rect.size.y > 0.0));
                    }
                }
            }
        });
        let mut o = ctx.run_ui(Default::default(), |_| {});
        o.textures_delta.clear();
        out
    }

    #[test]
    fn the_house_theme_is_the_default_and_comes_from_craft_tokens() {
        use egui::Color32;
        let t = super::Tokens::house();
        assert!(t.dark);
        assert_eq!(t.accent, Color32::from_rgb(0x8f, 0xb8, 0xd8), "steel-blue accent");
        assert_eq!(t.chrome, Color32::from_rgb(0x0b, 0x0e, 0x14));
        assert_eq!(t.canvas, Color32::from_rgb(0x07, 0x09, 0x0d));
        assert_eq!(t.card, Color32::from_rgb(0x0c, 0x0f, 0x15));
        assert_eq!(t.text, Color32::from_rgb(0xef, 0xfc, 0xff));
        assert_eq!(t.current, Color32::from_rgb(0xd9, 0xc0, 0x8a), "gold marks the current slide");
        for c in [t.card, t.border, t.hover, t.pressed, t.selected, t.field_bg] {
            assert_eq!(c.a(), 255, "house surfaces are opaque: {c:?}");
        }
        assert_eq!(super::Brightness::default(), super::Brightness::Dark);
        assert_eq!(super::Tokens::for_brightness(super::Brightness::default()).accent, t.accent);
        // Light stays as the alternate.
        assert!(!super::Tokens::for_brightness(super::Brightness::Light).dark);
        let ctx = egui::Context::default();
        super::apply(&ctx, &t);
        assert!(ctx.global_style().visuals.dark_mode);
        assert_eq!(super::Tokens::get(&ctx).accent, t.accent);
    }

    #[test]
    fn thin_punctuation_is_drawn_in_interface_text() {
        // En and em dashes (thin bars) were hinted down to nothing in the Japanese interface face,
        // so notes and the outline showed gaps where they were.
        for (c, size, ink) in drawn("\u{2013}\u{2014}-_\u{2026}\u{2022}\u{00b7}\u{2018}\u{2019}\u{201c}\u{201d}=~") {
            assert!(ink, "{c:?} ({:04X}) draws nothing at {size} px", c as u32);
        }
    }
}

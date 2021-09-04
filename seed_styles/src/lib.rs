mod style;

// exports
pub use style::CssValueTrait;

// style builder
pub use style::s;

pub use seed_style_macros::{as_tag, process_part, process_submacro_part, view_macro, *};

#[macro_export]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

// style property argument trait
pub use style::{
    // exports for Seed Layout
    composition::{default_breakpoint_theme, Composition, SeedBreakpoint, WithLayoutComposition},
    // Css Values
    css_values::*,
    // resizing
    helpers::conditionally_skip_rendering,
    layout::{Layout, LayoutArea, NoArea, WithGridLayout},
    // measures
    measures::{cm, em, hsl, hsla, hsluv, hsluva, pc, px, rem, rgb, rgba, vh, vw, ExactLength},
    // presets
    presets::{default_colors_theme, seed_colors},
    //ro col layout
    row_col_layout::{
        Column, ColumnBuilder, ColumnItemArgs, Row, RowBuilder, RowColumnArgs, RowItemArgs, *,
    },

    theme::change_theme_with_name,

    // themes
    theme::Theme,
    theme::{app_themes, load_app_themes},
    // themes, conditional rendering
    theme::{at_breakpoint_and_above, except, only, only_and_above, only_and_below},
    //theme alias keys
    theme::{
        BorderRadiusTheme, BorderStyleTheme, BorderTheme, BorderWidthTheme, BreakpointTheme,
        ColorTheme, DisplayTheme, FontSizeTheme, FontTheme, LetterSpacingTheme, LineHeightTheme,
        ShadowTheme, SizeTheme, SpaceTheme, StyleTheme, TransitionTheme, ZIndexTheme,
    },
    AddStyleToNode,
    // global style api
    GlobalStyle,
    // extension trait to allow Style structs to be update_el processed by seed.
    LocalUpdateEl,
    LocalUpdateElForIterator,
    PseudoTrait,
    // Style struct,  technically user shouldn't really need to access this directly
    Style,
    // style property argument trait
    UpdateStyle,
};

// pub trait UpdateView<B,Ms> {
//     fn update_view(self, builder: &mut B);
// }

// impl<T,B> UpdateView<B> for T
// where T:UpdateEl(Ms),
// {
//     fn update_view(self, builder: &mut B) {
//         match builder.root {
//             Node::Element(ref mut el) => self.update_el(el),
//             _ => {}
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

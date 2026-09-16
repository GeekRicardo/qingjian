//! 主题：字体、颜色、间距。所有可视参数都在这里，单位是点；将来从 TOML 读。
//!
//! 视觉层级（产品决定）：候选词最深，译文稍浅，词性最浅，序号弱化。数值对齐 macOS 壳的 AppKit 实现。

mod font_spec;
mod palette;

pub use font_spec::FontSpec;
pub use palette::Palette;

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    /// 候选词字体。
    pub text_font: FontSpec,

    /// 译文与词性字体。
    pub annotation_font: FontSpec,

    /// 序号字体。
    pub index_font: FontSpec,

    /// 配色。
    pub colors: Palette,

    /// 窗口内边距。
    pub padding: f32,

    /// 行内上下留白。
    pub row_padding: f32,

    /// 序号与候选词、候选词与译文之间的间距。
    pub column_gap: f32,

    /// 窗口与高亮条的圆角。
    pub corner_radius: f32,

    /// 最多显示几行。
    pub max_rows: usize,

    /// 文字抗锯齿覆盖率的 gamma：小于 1 笔画显粗。CoreText 对文字有一层类似的加深，深色背景上尤其明显，
    /// 线性混合出来的字会偏细；这个值按真机截图并排调。
    pub text_gamma: f32,
}

impl Theme {
    /// 缺省候选词字号（点）。译文与序号字体、各自的行高都按它的比例缩放，见 [`Theme::with_font_size`]。
    pub const BASE_FONT_SIZE: f32 = 16.0;

    /// 按候选词字号缩放三种字体（配置 `[general] font_size`）。
    /// 间距与圆角不动：调的是字的大小，不是整个窗口的缩放——窗口本来就按内容量出来。
    pub fn with_font_size(mut self, size: f32) -> Self {
        let scale = size / Self::BASE_FONT_SIZE;
        self.text_font = self.text_font.scaled(scale);
        self.annotation_font = self.annotation_font.scaled(scale);
        self.index_font = self.index_font.scaled(scale);
        self
    }

    /// 浅色，对齐 macOS 系统外观。
    pub fn light() -> Self {
        Self::with_palette(Palette::light(), 0.85)
    }

    /// 深色，对齐 macOS 系统外观。
    pub fn dark() -> Self {
        Self::with_palette(Palette::dark(), 0.75)
    }

    fn with_palette(colors: Palette, text_gamma: f32) -> Self {
        Self {
            // 行高取 AppKit 系统字体在这几个字号下 NSAttributedString.size() 的高度
            text_font: FontSpec::new(16.0, 19.0),
            annotation_font: FontSpec::new(12.0, 15.0),
            index_font: FontSpec::new(11.0, 14.0),
            colors,
            padding: 8.0,
            row_padding: 4.0,
            column_gap: 8.0,
            corner_radius: 8.0,
            max_rows: 9,
            text_gamma,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_font_size_is_a_no_op() {
        let theme = Theme::light();
        let scaled = Theme::light().with_font_size(Theme::BASE_FONT_SIZE);
        assert_eq!(scaled.text_font, theme.text_font);
        assert_eq!(scaled.annotation_font, theme.annotation_font);
        assert_eq!(scaled.index_font, theme.index_font);
    }

    #[test]
    fn fonts_and_line_heights_scale_together_and_spacing_stays() {
        let theme = Theme::light().with_font_size(24.0);
        assert_eq!(theme.text_font.size, 24.0);
        assert_eq!(theme.text_font.line_height, 19.0 * 1.5);
        assert_eq!(theme.annotation_font.size, 18.0);
        assert_eq!(theme.index_font.size, 11.0 * 1.5);
        // 间距不跟着字号走
        assert_eq!(theme.padding, Theme::light().padding);
        assert_eq!(theme.column_gap, Theme::light().column_gap);
    }
}

use embedded_graphics_cjk_font_build_tool::{
    BuildError, MonoFontBuilder, CJK_RADICALS_SUPPLEMENT, CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK,
};

const VERSION: &str = include_str!("SARASA_VERSION");

#[rustfmt::skip]
const FONTS: &[(&str, u32)] = &[
    ("sarasa-mono-sc-light", 24),
    ("sarasa-mono-sc-light", 36)
];

fn main() -> Result<(), BuildError> {
    println!(
        "cargo:rerun-if-changed=target/font/sarasa-gothic-ttf-{}/sarasa-mono-sc-light.ttf",
        VERSION
    );

    for &(font_name, font_size) in FONTS {
        let mono_font_builder = MonoFontBuilder::new(
            format!(
                "target/font/sarasa-gothic-ttf-{}/{}.ttf",
                VERSION, font_name
            ),
            font_size,
            ('?'..='?')
                .chain(CJK_RADICALS_SUPPLEMENT.range())
                .chain(CJK_UNIFIED_IDEOGRAPHS_UNICODE_BLOCK.range()),
            0,
        )?;

        mono_font_builder
            .build()?
            .save_all_with_default_paths(font_name, font_size)?;
    }

    Ok(())
}

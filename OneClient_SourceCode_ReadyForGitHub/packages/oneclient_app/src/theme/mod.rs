use bytes::Bytes;

use crate::AppAssets;

pub mod colors;

pub const NAVBAR_HEIGHT_PX: f32 = 80.;
pub const HOME_PADDING_PX: f32 = 48.;
pub const DEFAULT_FONT: &str = "Inter";
pub const ARABIC_FONT: &str = "Cairo";
pub const PIXEL_FONT: &str = "Press Start 2P";
pub const MONO_FONT: &str = "JetBrains Mono";

pub fn load_fonts() -> Vec<(&'static str, Bytes)> {
    const FONTS: [(&str, &str); 6] = [
        (DEFAULT_FONT, "fonts/Inter/Inter-Regular.ttf"),
        (DEFAULT_FONT, "fonts/Inter/Inter-Bold.ttf"),
        (ARABIC_FONT, "fonts/Cairo/Cairo-Regular.ttf"),
        (ARABIC_FONT, "fonts/Cairo/Cairo-Black.ttf"),
        (PIXEL_FONT, "fonts/Pixel/PressStart2P.ttf"),
        (MONO_FONT, "fonts/JetBrainsMono/JetBrainsMonoVariable.ttf"),
    ];

    let mut loaded: Vec<(&'static str, Bytes)> = FONTS
        .iter()
        .filter_map(|&(family, file)| {
            let bytes = AppAssets::get_bytes(file).or_else(|| {
                tracing::error!("Failed to load embedded font '{file}'");
                None
            })?;

            Some((family, bytes))
        })
        .collect();

    #[cfg(target_os = "windows")]
    {
        for &(family, path) in &[
            ("Segoe UI", "C:\\Windows\\Fonts\\segoeui.ttf"),
            ("Segoe UI", "C:\\Windows\\Fonts\\segoeuib.ttf"),
            ("Tahoma", "C:\\Windows\\Fonts\\tahoma.ttf"),
            ("Arial", "C:\\Windows\\Fonts\\arial.ttf"),
        ] {
            if let Ok(data) = std::fs::read(path) {
                loaded.push((family, Bytes::from(data)));
            }
        }
    }

    loaded
}

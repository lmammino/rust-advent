use std::ops::RangeInclusive;

pub(crate) enum Pixel {
    On,
    Off,
}

#[derive(Debug)]
pub(crate) struct Display {
    pixels: String,
    current_pixel: usize,
}

impl Display {
    pub(crate) fn new() -> Self {
        Self {
            pixels: String::with_capacity(40 * 6),
            current_pixel: 0,
        }
    }

    pub(crate) fn set_pixel(&mut self, pixel: Pixel) {
        let char_to_draw = match pixel {
            Pixel::On => '#',
            Pixel::Off => '.',
        };
        self.pixels.push(char_to_draw);
        self.current_pixel += 1;
    }

    pub(crate) fn sprite_pos(&self) -> RangeInclusive<i32> {
        RangeInclusive::new(
            ((self.current_pixel % 40) as i32) - 1,
            ((self.current_pixel % 40) as i32) + 1,
        )
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = self.pixels.chars();
        for _ in 0..6 {
            let line = chars.by_ref().take(40).collect::<String>();
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

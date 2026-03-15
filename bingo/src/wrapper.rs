use owo_colors::colors;
use owo_colors::colors::CustomColor;
use owo_colors::styles::{
	BlinkDisplay, BlinkFastDisplay, BoldDisplay, DimDisplay, HiddenDisplay, ItalicDisplay,
	ReversedDisplay, StrikeThroughDisplay, UnderlineDisplay,
};
use owo_colors::Color;
use owo_colors::{
	BgColorDisplay, BgDynColorDisplay, DynColor, FgColorDisplay, FgDynColorDisplay, OwoColorize,
	Rgb, Style, Styled,
};

pub trait ColorizeWrapper: Sized {
	#[must_use]
	fn fg<C: Color>(&self) -> FgColorDisplay<'_, C, Self>;

	#[must_use]
	fn bg<C: Color>(&self) -> BgColorDisplay<'_, C, Self>;

	#[must_use]
	fn color<Color: DynColor>(&self, color: Color) -> FgDynColorDisplay<'_, Color, Self>;

	#[must_use]
	fn on_color<Color: DynColor>(&self, color: Color) -> BgDynColorDisplay<'_, Color, Self>;

	#[must_use]
	fn fg_rgb<const R: u8, const G: u8, const B: u8>(
		&self,
	) -> FgColorDisplay<'_, CustomColor<R, G, B>, Self>;

	#[must_use]
	fn bg_rgb<const R: u8, const G: u8, const B: u8>(
		&self,
	) -> BgColorDisplay<'_, CustomColor<R, G, B>, Self>;

	fn truecolor(&self, r: u8, g: u8, b: u8) -> FgDynColorDisplay<'_, Rgb, Self>;

	#[must_use]
	fn on_truecolor(&self, r: u8, g: u8, b: u8) -> BgDynColorDisplay<'_, Rgb, Self>;

	#[must_use]
	fn style(&self, style: Style) -> Styled<&Self>;

	#[must_use]
	fn bold(&self) -> BoldDisplay<'_, Self>;

	#[must_use]
	fn dimmed(&self) -> DimDisplay<'_, Self>;

	#[must_use]
	fn italic(&self) -> ItalicDisplay<'_, Self>;

	#[must_use]
	fn underline(&self) -> UnderlineDisplay<'_, Self>;

	#[must_use]
	fn blink(&self) -> BlinkDisplay<'_, Self>;

	#[must_use]
	fn blink_fast(&self) -> BlinkFastDisplay<'_, Self>;

	#[must_use]
	fn reversed(&self) -> ReversedDisplay<'_, Self>;

	#[must_use]
	fn hidden(&self) -> HiddenDisplay<'_, Self>;

	#[must_use]
	fn strikethrough(&self) -> StrikeThroughDisplay<'_, Self>;

	#[must_use]
	fn black(&self) -> FgColorDisplay<'_, colors::Black, Self>;
	#[must_use]
	fn on_black(&self) -> BgColorDisplay<'_, colors::Black, Self>;
	#[must_use]
	fn red(&self) -> FgColorDisplay<'_, colors::Red, Self>;
	#[must_use]
	fn on_red(&self) -> BgColorDisplay<'_, colors::Red, Self>;
	#[must_use]
	fn green(&self) -> FgColorDisplay<'_, colors::Green, Self>;
	#[must_use]
	fn on_green(&self) -> BgColorDisplay<'_, colors::Green, Self>;
	#[must_use]
	fn yellow(&self) -> FgColorDisplay<'_, colors::Yellow, Self>;
	#[must_use]
	fn on_yellow(&self) -> BgColorDisplay<'_, colors::Yellow, Self>;
	#[must_use]
	fn blue(&self) -> FgColorDisplay<'_, colors::Blue, Self>;
	#[must_use]
	fn on_blue(&self) -> BgColorDisplay<'_, colors::Blue, Self>;
	#[must_use]
	fn magenta(&self) -> FgColorDisplay<'_, colors::Magenta, Self>;
	#[must_use]
	fn on_magenta(&self) -> BgColorDisplay<'_, colors::Magenta, Self>;
	#[must_use]
	fn purple(&self) -> FgColorDisplay<'_, colors::Magenta, Self>;
	#[must_use]
	fn on_purple(&self) -> BgColorDisplay<'_, colors::Magenta, Self>;
	#[must_use]
	fn cyan(&self) -> FgColorDisplay<'_, colors::Cyan, Self>;
	#[must_use]
	fn on_cyan(&self) -> BgColorDisplay<'_, colors::Cyan, Self>;
	#[must_use]
	fn white(&self) -> FgColorDisplay<'_, colors::White, Self>;
	#[must_use]
	fn on_white(&self) -> BgColorDisplay<'_, colors::White, Self>;

	// --- Default ---
	#[must_use]
	fn default_color(&self) -> FgColorDisplay<'_, colors::Default, Self>;
	#[must_use]
	fn on_default_color(&self) -> BgColorDisplay<'_, colors::Default, Self>;

	// --- Bright Colors ---
	#[must_use]
	fn bright_black(&self) -> FgColorDisplay<'_, colors::BrightBlack, Self>;
	#[must_use]
	fn on_bright_black(&self) -> BgColorDisplay<'_, colors::BrightBlack, Self>;
	#[must_use]
	fn bright_red(&self) -> FgColorDisplay<'_, colors::BrightRed, Self>;
	#[must_use]
	fn on_bright_red(&self) -> BgColorDisplay<'_, colors::BrightRed, Self>;
	#[must_use]
	fn bright_green(&self) -> FgColorDisplay<'_, colors::BrightGreen, Self>;
	#[must_use]
	fn on_bright_green(&self) -> BgColorDisplay<'_, colors::BrightGreen, Self>;
	#[must_use]
	fn bright_yellow(&self) -> FgColorDisplay<'_, colors::BrightYellow, Self>;
	#[must_use]
	fn on_bright_yellow(&self) -> BgColorDisplay<'_, colors::BrightYellow, Self>;
	#[must_use]
	fn bright_blue(&self) -> FgColorDisplay<'_, colors::BrightBlue, Self>;
	#[must_use]
	fn on_bright_blue(&self) -> BgColorDisplay<'_, colors::BrightBlue, Self>;
	#[must_use]
	fn bright_magenta(&self) -> FgColorDisplay<'_, colors::BrightMagenta, Self>;
	#[must_use]
	fn on_bright_magenta(&self) -> BgColorDisplay<'_, colors::BrightMagenta, Self>;
	#[must_use]
	fn bright_purple(&self) -> FgColorDisplay<'_, colors::BrightMagenta, Self>;
	#[must_use]
	fn on_bright_purple(&self) -> BgColorDisplay<'_, colors::BrightMagenta, Self>;
	#[must_use]
	fn bright_cyan(&self) -> FgColorDisplay<'_, colors::BrightCyan, Self>;
	#[must_use]
	fn on_bright_cyan(&self) -> BgColorDisplay<'_, colors::BrightCyan, Self>;
	#[must_use]
	fn bright_white(&self) -> FgColorDisplay<'_, colors::BrightWhite, Self>;
	#[must_use]
	fn on_bright_white(&self) -> BgColorDisplay<'_, colors::BrightWhite, Self>;
}
impl<T: Sized> ColorizeWrapper for T {
	#[inline(always)]
	fn fg<C: Color>(&self) -> FgColorDisplay<'_, C, Self> {
		OwoColorize::fg(&self)
	}

	#[inline(always)]
	fn bg<C: Color>(&self) -> BgColorDisplay<'_, C, Self> {
		OwoColorize::bg(&self)
	}

	#[inline(always)]
	fn color<Color: DynColor>(&self, color: Color) -> FgDynColorDisplay<'_, Color, Self> {
		OwoColorize::color(&self, color)
	}

	#[inline(always)]
	fn on_color<Color: DynColor>(&self, color: Color) -> BgDynColorDisplay<'_, Color, Self> {
		OwoColorize::on_color(&self, color)
	}

	#[inline(always)]
	fn fg_rgb<const R: u8, const G: u8, const B: u8>(
		&self,
	) -> FgColorDisplay<'_, CustomColor<R, G, B>, Self> {
		OwoColorize::fg_rgb::<R, G, B>(&self)
	}

	#[inline(always)]
	fn bg_rgb<const R: u8, const G: u8, const B: u8>(
		&self,
	) -> BgColorDisplay<'_, CustomColor<R, G, B>, Self> {
		OwoColorize::bg_rgb::<R, G, B>(&self)
	}

	#[inline(always)]
	fn truecolor(&self, r: u8, g: u8, b: u8) -> FgDynColorDisplay<'_, Rgb, Self> {
		OwoColorize::truecolor(&self, r, g, b)
	}

	#[inline(always)]
	fn on_truecolor(&self, r: u8, g: u8, b: u8) -> BgDynColorDisplay<'_, Rgb, Self> {
		OwoColorize::on_truecolor(&self, r, g, b)
	}

	#[inline(always)]
	fn style(&self, style: Style) -> Styled<&Self> {
		OwoColorize::style(&self, style)
	}

	#[inline(always)]
	fn bold(&self) -> BoldDisplay<'_, Self> {
		OwoColorize::bold(&self)
	}

	#[inline(always)]
	fn dimmed(&self) -> DimDisplay<'_, Self> {
		OwoColorize::dimmed(&self)
	}

	#[inline(always)]
	fn italic(&self) -> ItalicDisplay<'_, Self> {
		OwoColorize::italic(&self)
	}

	#[inline(always)]
	fn underline(&self) -> UnderlineDisplay<'_, Self> {
		OwoColorize::underline(&self)
	}

	#[inline(always)]
	fn blink(&self) -> BlinkDisplay<'_, Self> {
		OwoColorize::blink(&self)
	}

	#[inline(always)]
	fn blink_fast(&self) -> BlinkFastDisplay<'_, Self> {
		OwoColorize::blink_fast(&self)
	}

	#[inline(always)]
	fn reversed(&self) -> ReversedDisplay<'_, Self> {
		OwoColorize::reversed(&self)
	}

	#[inline(always)]
	fn hidden(&self) -> HiddenDisplay<'_, Self> {
		OwoColorize::hidden(&self)
	}

	#[inline(always)]
	fn strikethrough(&self) -> StrikeThroughDisplay<'_, Self> {
		OwoColorize::strikethrough(&self)
	}

	#[inline(always)]
	fn black(&self) -> FgColorDisplay<'_, colors::Black, Self> {
		OwoColorize::black(&self)
	}
	#[inline(always)]
	fn on_black(&self) -> BgColorDisplay<'_, colors::Black, Self> {
		OwoColorize::on_black(&self)
	}
	#[inline(always)]
	fn red(&self) -> FgColorDisplay<'_, colors::Red, Self> {
		OwoColorize::red(&self)
	}
	#[inline(always)]
	fn on_red(&self) -> BgColorDisplay<'_, colors::Red, Self> {
		OwoColorize::on_red(&self)
	}
	#[inline(always)]
	fn green(&self) -> FgColorDisplay<'_, colors::Green, Self> {
		OwoColorize::green(&self)
	}
	#[inline(always)]
	fn on_green(&self) -> BgColorDisplay<'_, colors::Green, Self> {
		OwoColorize::on_green(&self)
	}
	#[inline(always)]
	fn yellow(&self) -> FgColorDisplay<'_, colors::Yellow, Self> {
		OwoColorize::yellow(&self)
	}
	#[inline(always)]
	fn on_yellow(&self) -> BgColorDisplay<'_, colors::Yellow, Self> {
		OwoColorize::on_yellow(&self)
	}
	#[inline(always)]
	fn blue(&self) -> FgColorDisplay<'_, colors::Blue, Self> {
		OwoColorize::blue(&self)
	}
	#[inline(always)]
	fn on_blue(&self) -> BgColorDisplay<'_, colors::Blue, Self> {
		OwoColorize::on_blue(&self)
	}
	#[inline(always)]
	fn magenta(&self) -> FgColorDisplay<'_, colors::Magenta, Self> {
		OwoColorize::magenta(&self)
	}
	#[inline(always)]
	fn on_magenta(&self) -> BgColorDisplay<'_, colors::Magenta, Self> {
		OwoColorize::on_magenta(&self)
	}
	#[inline(always)]
	fn purple(&self) -> FgColorDisplay<'_, colors::Magenta, Self> {
		OwoColorize::purple(&self)
	}
	#[inline(always)]
	fn on_purple(&self) -> BgColorDisplay<'_, colors::Magenta, Self> {
		OwoColorize::on_purple(&self)
	}
	#[inline(always)]
	fn cyan(&self) -> FgColorDisplay<'_, colors::Cyan, Self> {
		OwoColorize::cyan(&self)
	}
	#[inline(always)]
	fn on_cyan(&self) -> BgColorDisplay<'_, colors::Cyan, Self> {
		OwoColorize::on_cyan(&self)
	}
	#[inline(always)]
	fn white(&self) -> FgColorDisplay<'_, colors::White, Self> {
		OwoColorize::white(&self)
	}
	#[inline(always)]
	fn on_white(&self) -> BgColorDisplay<'_, colors::White, Self> {
		OwoColorize::on_white(&self)
	}

	// --- Default ---
	#[inline(always)]
	fn default_color(&self) -> FgColorDisplay<'_, colors::Default, Self> {
		OwoColorize::default_color(&self)
	}
	#[inline(always)]
	fn on_default_color(&self) -> BgColorDisplay<'_, colors::Default, Self> {
		OwoColorize::on_default_color(&self)
	}

	// --- Bright Colors ---
	#[inline(always)]
	fn bright_black(&self) -> FgColorDisplay<'_, colors::BrightBlack, Self> {
		OwoColorize::bright_black(&self)
	}
	#[inline(always)]
	fn on_bright_black(&self) -> BgColorDisplay<'_, colors::BrightBlack, Self> {
		OwoColorize::on_bright_black(&self)
	}
	#[inline(always)]
	fn bright_red(&self) -> FgColorDisplay<'_, colors::BrightRed, Self> {
		OwoColorize::bright_red(&self)
	}
	#[inline(always)]
	fn on_bright_red(&self) -> BgColorDisplay<'_, colors::BrightRed, Self> {
		OwoColorize::on_bright_red(&self)
	}
	#[inline(always)]
	fn bright_green(&self) -> FgColorDisplay<'_, colors::BrightGreen, Self> {
		OwoColorize::bright_green(&self)
	}
	#[inline(always)]
	fn on_bright_green(&self) -> BgColorDisplay<'_, colors::BrightGreen, Self> {
		OwoColorize::on_bright_green(&self)
	}
	#[inline(always)]
	fn bright_yellow(&self) -> FgColorDisplay<'_, colors::BrightYellow, Self> {
		OwoColorize::bright_yellow(&self)
	}
	#[inline(always)]
	fn on_bright_yellow(&self) -> BgColorDisplay<'_, colors::BrightYellow, Self> {
		OwoColorize::on_bright_yellow(&self)
	}
	#[inline(always)]
	fn bright_blue(&self) -> FgColorDisplay<'_, colors::BrightBlue, Self> {
		OwoColorize::bright_blue(&self)
	}
	#[inline(always)]
	fn on_bright_blue(&self) -> BgColorDisplay<'_, colors::BrightBlue, Self> {
		OwoColorize::on_bright_blue(&self)
	}
	#[inline(always)]
	fn bright_magenta(&self) -> FgColorDisplay<'_, colors::BrightMagenta, Self> {
		OwoColorize::bright_magenta(&self)
	}
	#[inline(always)]
	fn on_bright_magenta(&self) -> BgColorDisplay<'_, colors::BrightMagenta, Self> {
		OwoColorize::on_bright_magenta(&self)
	}
	#[inline(always)]
	fn bright_purple(&self) -> FgColorDisplay<'_, colors::BrightMagenta, Self> {
		OwoColorize::bright_purple(&self)
	}
	#[inline(always)]
	fn on_bright_purple(&self) -> BgColorDisplay<'_, colors::BrightMagenta, Self> {
		OwoColorize::on_bright_purple(&self)
	}
	#[inline(always)]
	fn bright_cyan(&self) -> FgColorDisplay<'_, colors::BrightCyan, Self> {
		OwoColorize::bright_cyan(&self)
	}
	#[inline(always)]
	fn on_bright_cyan(&self) -> BgColorDisplay<'_, colors::BrightCyan, Self> {
		OwoColorize::on_bright_cyan(&self)
	}
	#[inline(always)]
	fn bright_white(&self) -> FgColorDisplay<'_, colors::BrightWhite, Self> {
		OwoColorize::bright_white(&self)
	}
	#[inline(always)]
	fn on_bright_white(&self) -> BgColorDisplay<'_, colors::BrightWhite, Self> {
		OwoColorize::on_bright_white(&self)
	}
}

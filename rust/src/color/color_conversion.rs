pub fn rgba_to_hsba(rgba: Vec<f64>) -> Vec<f64> {
	let red = rgba[0];
	let green = rgba[1];
	let blue = rgba[2];

	let val = red.max(green).max(blue);
	let chroma = val - red.min(green).min(blue);

	let mut hue = 0.0;
	let sat;
	if chroma == 0.0 {
		hue = 0.0;
		sat = 0.0;
	} else {
		sat = chroma / val;

		if red == val {
			// Magenta to yellow
			hue = (green - blue) / chroma;
		} else if green == val {
			// Yellow to cyan
			hue = 2.0 + (blue - red) / chroma;
		} else if blue == val {
			// Cyan to magenta
			hue = 4.0 + (red - green) / chroma;
		}

		if hue < 0.0 {
			// Confine hue to the interval [0, 1).
			hue += 6.0;
		} else if hue >= 6.0 {
			hue -= 6.0;
		}
	}

	vec!(hue / 6.0, sat, val, rgba[3])
}

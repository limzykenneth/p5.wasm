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

pub fn rgba_to_hsla(rgba: Vec<f64>) -> Vec<f64> {
	let red = rgba[0];
	let green = rgba[1];
	let blue = rgba[2];

	let val = red.max(green).max(blue);
	let min = red.min(green).min(blue);
	let li = val + min;
	let chroma = val - min;

	let mut hue = 0.0;
	let sat;

	if chroma == 0.0 {
		hue = 0.0;
		sat = 0.0;
	} else {
		if li < 1.0 {
			sat = chroma / li;
		} else {
			sat = chroma / (2.0 - li);
		}

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

	vec!(hue / 6.0, sat, li / 2.0, rgba[3])
}

pub fn hsba_to_rgba(hsba: Vec<f64>) -> Vec<f64> {
	let hue = hsba[0] * 6.0; // We will split hue into 6 sectors.
	let sat = hsba[1];
	let val = hsba[2];

	let rgba: Vec<f64>;

	if sat == 0.0 {
		rgba = vec!(val, val, val, hsba[3]);
	} else {
		let sector = hue.floor();
		let tint1 = val * (1.0 - sat);
		let tint2 = val * (1.0 - sat * (hue - sector));
		let tint3 = val * (1.0 - sat * (1.0 + sector - hue));

		if sector == 1.0 {
			rgba = vec!(tint2, val, tint1, hsba[3]);
		} else if sector == 2.0 {
			rgba = vec!(tint1, val, tint3, hsba[3]);
		} else if sector == 3.0 {
			rgba = vec!(tint1, tint2, val, hsba[3]);
		} else if sector == 4.0 {
			rgba = vec!(tint3, tint1, val, hsba[3]);
		} else if sector == 5.0 {
			rgba = vec!(val, tint1, tint2, hsba[3]);
		} else {
			rgba = vec!(val, tint3, tint1, hsba[3]);
		}
	}

	rgba
}

pub fn hsla_to_rgba(hsla: Vec<f64>) -> Vec<f64> {
	let hue = hsla[0] * 6.0; // We will split hue into 6 sectors.
	let sat = hsla[1];
	let li = hsla[2];

	let rgba: Vec<f64>;

	if sat == 0.0 {
		rgba = vec!(li, li, li, hsla[3]);
	} else {
		// Calculate brightness.
	    let val;
		if li < 0.5 {
			val = (1.0 + sat) * li;
		} else {
			val = li + sat - li * sat;
		}

		// Define zest.
		let zest = 2.0 * li - val;

		// Implement projection (project onto green by default).
		let hzv_to_rgb = |mut hue: f64, zest: f64, val: f64| {
			if hue < 0.0 {
				// Hue must wrap to allow projection onto red and blue.
				hue += 6.0;
			} else if hue >= 6.0 {
				hue -= 6.0;
			}
			if hue < 1.0 {
				// Red to yellow (increasing green).
				return zest + (val - zest) * hue;
			} else if hue < 3.0 {
				// Yellow to cyan (greatest green).
				return val;
			} else if hue < 4.0 {
				// Cyan to blue (decreasing green).
				return zest + (val - zest) * (4.0 - hue);
			} else {
				// Blue to red (least green).
				return zest;
			}
		};

		// Perform projections, offsetting hue as necessary.
		rgba = vec!(hzv_to_rgb(hue + 2.0, zest, val), hzv_to_rgb(hue, zest, val), hzv_to_rgb(hue - 2.0, zest, val), hsla[3]);
	}

	rgba
}
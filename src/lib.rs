#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    /// Creates a new RGB color with the specified red, green, and blue values.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns the hexadecimal representation of the RGB color as a string. Used in the `Debug` and `Display` implementations.
    pub fn hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Returns the inverted color of the current one.
    pub fn invert(self) -> Self {
        Self {
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b,
        }
    }
}

impl std::fmt::Debug for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGB color ({})", self.hex())
    }
}

impl std::fmt::Display for RGB {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.hex())
	}
}

impl std::ops::Add for RGB {
    type Output = Self;
	/// Adds two RGB colors together, saturating at 255.
    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
        }
    }
}

impl std::ops::Sub for RGB {
    type Output = Self;
	/// Subtracts one RGB color from another, saturating at 0.
    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r.saturating_sub(rhs.r),
            g: self.g.saturating_sub(rhs.g),
            b: self.b.saturating_sub(rhs.b),
        }
    }
}

impl Default for RGB {
	/// Default constructor for RGB; initializes the color to black.
	fn default() -> Self {
		Self { r: 0, g: 0, b: 0 }
	}
}

impl From<(u8, u8, u8)> for RGB {
	/// RGB construtor from 3 u8's
	fn from(tuple: (u8, u8, u8)) -> Self {
		Self::new(tuple.0, tuple.1, tuple.2)
	}
}

impl From<[u8; 3]> for RGB {
	/// RGB construtor from an array of 3 u8's
	fn from(array: [u8; 3]) -> Self {
		Self::new(array[0], array[1], array[2])
	}
}

#[cfg(test)]
mod tests;

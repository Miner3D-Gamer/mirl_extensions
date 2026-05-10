use super::{U1, U2, U4};

// Conversion between U1 and U2
impl const From<U2> for U1 {
    fn from(val: U2) -> Self {
        // Extract just the lowest bit from U2
        Self {
            b0: val.b0,
        }
    }
}

impl const From<U1> for U2 {
    fn from(val: U1) -> Self {
        // Set the upper bit to false
        Self {
            b0: val.b0,
            b1: false,
        }
    }
}

// Conversion between U1 and U4
impl const From<U4> for U1 {
    fn from(val: U4) -> Self {
        // Extract just the lowest bit from U4
        Self {
            b0: val.b0,
        }
    }
}

impl const From<U1> for U4 {
    fn from(val: U1) -> Self {
        // Set the upper bits to false
        Self {
            b0: val.b0,
            b1: false,
            b2: false,
            b3: false,
        }
    }
}

// Additional helper methods for U1
impl U1 {
    /// Convert to U2 (with upper bit set to 0)
    #[must_use]
    pub fn to_u2(self) -> U2 {
        self.into()
    }

    /// Convert to U4 (with upper 3 bits set to 0)
    #[must_use]
    pub fn to_u4(self) -> U4 {
        self.into()
    }
}

// Additional helper methods for U2
impl U2 {
    /// Convert to U1 by truncating to lowest bit
    #[must_use]
    pub fn to_u1(self) -> U1 {
        self.into()
    }

    /// Create a U2 from two U1 values
    /// The high U1 will occupy bit 1, and the low U1 will occupy bit 0
    #[must_use]
    pub const fn from_u1_pair(high: U1, low: U1) -> Self {
        Self {
            b0: low.b0,
            b1: high.b0,
        }
    }

    /// Split a U2 into high and low U1 values
    /// Returns (`high_bit`, `low_bit`) where `high_bit` is bit 1 and `low_bit` is bit 0
    #[must_use]
    pub const fn split_to_u1_pair(self) -> (U1, U1) {
        let high = U1 {
            b0: self.b1,
        };

        let low = U1 {
            b0: self.b0,
        };

        (high, low)
    }
}

// Additional helper methods for U4
impl U4 {
    /// Convert to U1 by truncating to lowest bit
    #[must_use]
    pub fn to_u1(self) -> U1 {
        self.into()
    }

    /// Create a U4 from four U1 values
    #[must_use]
    pub const fn from_u1_quad(b3: U1, b2: U1, b1: U1, b0: U1) -> Self {
        Self {
            b0: b0.b0,
            b1: b1.b0,
            b2: b2.b0,
            b3: b3.b0,
        }
    }

    /// Split a U4 into four U1 values
    /// Returns (bit3, bit2, bit1, bit0)
    #[must_use]
    pub const fn split_to_u1_quad(self) -> (U1, U1, U1, U1) {
        let bit3 = U1 {
            b0: self.b3,
        };
        let bit2 = U1 {
            b0: self.b2,
        };
        let bit1 = U1 {
            b0: self.b1,
        };
        let bit0 = U1 {
            b0: self.b0,
        };

        (bit3, bit2, bit1, bit0)
    }
}

impl const From<U2> for U4 {
    fn from(val: U2) -> Self {
        // Set the upper 2 bits to false
        Self {
            b0: val.b0,
            b1: val.b1,
            b2: false,
            b3: false,
        }
    }
}

impl const From<U4> for U2 {
    fn from(val: U4) -> Self {
        // Extract just the lower 2 bits from U4
        Self {
            b0: val.b0,
            b1: val.b1,
        }
    }
}

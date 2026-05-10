/// Convert the given keycode into a number if possible
pub const trait GetFunctionKeyFromKeycode {
    #[must_use]
    /// Get the number of the keycode if it is a number
    fn get_function_key<T: crate::ConstNumbers128>(&self) -> Option<T>;
    #[must_use]
    /// Check if the given keycode is a number
    fn is_function_key(&self) -> bool {
        self.get_function_key::<u8>().is_some()
    }
}
impl GetFunctionKeyFromKeycode for mirl_core::platform::keycodes::KeyCode {
    /// Get the number of the keycode if it is a number
    fn get_function_key<T: crate::ConstNumbers128>(&self) -> Option<T> {
        Some(match self {
            Self::F1 => T::CONST_1,
            Self::F2 => T::CONST_2,
            Self::F3 => T::CONST_3,
            Self::F4 => T::CONST_4,
            Self::F5 => T::CONST_5,
            Self::F6 => T::CONST_6,
            Self::F7 => T::CONST_7,
            Self::F8 => T::CONST_8,
            Self::F9 => T::CONST_9,
            Self::F10 => T::CONST_10,
            Self::F11 => T::CONST_11,
            Self::F12 => T::CONST_12,
            Self::F13 => T::CONST_13,
            Self::F14 => T::CONST_14,
            Self::F15 => T::CONST_15,
            Self::F16 => T::CONST_16,
            Self::F17 => T::CONST_17,
            Self::F18 => T::CONST_18,
            Self::F19 => T::CONST_19,
            Self::F20 => T::CONST_20,
            Self::F21 => T::CONST_21,
            Self::F22 => T::CONST_22,
            Self::F23 => T::CONST_23,
            Self::F24 => T::CONST_24,
            Self::F25 => T::CONST_25,
            _ => return None,
        })
    }
}

/// Convert the given keycode into a number if possible
pub const trait GetNumberFromKeycode {
    #[must_use]
    /// Get the number of the keycode if it is a number
    fn get_number<T: [const] crate::ConstNumbers128>(&self) -> Option<T>;
    #[must_use]
    /// Check if the given keycode is a number
    fn is_number(&self) -> bool {
        self.get_number::<u8>().is_some()
    }
}

impl const GetNumberFromKeycode for mirl_core::platform::keycodes::KeyCode {
    /// Get the number of the keycode if it is a number
    fn get_number<T: [const] crate::ConstNumbers128>(&self) -> Option<T> {
        Some(match self {
            Self::Num0 | Self::KeyPad0 => T::CONST_0,
            Self::Num1 | Self::KeyPad1 => T::CONST_1,
            Self::Num2 | Self::KeyPad2 => T::CONST_2,
            Self::Num3 | Self::KeyPad3 => T::CONST_3,
            Self::Num4 | Self::KeyPad4 => T::CONST_4,
            Self::Num5 | Self::KeyPad5 => T::CONST_5,
            Self::Num6 | Self::KeyPad6 => T::CONST_6,
            Self::Num7 | Self::KeyPad7 => T::CONST_7,
            Self::Num8 | Self::KeyPad8 => T::CONST_8,
            Self::Num9 | Self::KeyPad9 => T::CONST_9,

            _ => return None,
        })
    }
}

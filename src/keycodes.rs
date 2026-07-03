use mirl_extensions_math::Numbers128;

/// Convert the given keycode into a number if possible
pub const trait GetFunctionKeyFromKeycode {
    #[must_use]
    /// Get the number of the keycode if it is a number
    fn get_function_key<T: Numbers128>(&self) -> Option<T>;
    #[must_use]
    /// Check if the given keycode is a number
    fn is_function_key(&self) -> bool {
        self.get_function_key::<u8>().is_some()
    }
}
impl GetFunctionKeyFromKeycode for mirl_input::keyboard::KeyCode {
    /// Get the number of the keycode if it is a number
    fn get_function_key<T: Numbers128>(&self) -> Option<T> {
        Some(match self {
            Self::F1 => T::num_1(),
            Self::F2 => T::num_2(),
            Self::F3 => T::num_3(),
            Self::F4 => T::num_4(),
            Self::F5 => T::num_5(),
            Self::F6 => T::num_6(),
            Self::F7 => T::num_7(),
            Self::F8 => T::num_8(),
            Self::F9 => T::num_9(),
            Self::F10 => T::num_10(),
            Self::F11 => T::num_11(),
            Self::F12 => T::num_12(),
            Self::F13 => T::num_13(),
            Self::F14 => T::num_14(),
            Self::F15 => T::num_15(),
            Self::F16 => T::num_16(),
            Self::F17 => T::num_17(),
            Self::F18 => T::num_18(),
            Self::F19 => T::num_19(),
            Self::F20 => T::num_20(),
            Self::F21 => T::num_21(),
            Self::F22 => T::num_22(),
            Self::F23 => T::num_23(),
            Self::F24 => T::num_24(),
            Self::F25 => T::num_25(),
            _ => return None,
        })
    }
}

/// Convert the given keycode into a number if possible
pub const trait GetNumberFromKeycode {
    #[must_use]
    /// Get the number of the keycode if it is a number
    fn get_number<T: [const] Numbers128>(&self) -> Option<T>;
    #[must_use]
    /// Check if the given keycode is a number
    fn is_number(&self) -> bool {
        self.get_number::<u8>().is_some()
    }
}

const impl GetNumberFromKeycode for mirl_input::keyboard::KeyCode {
    /// Get the number of the keycode if it is a number
    fn get_number<T: [const] Numbers128>(&self) -> Option<T> {
        Some(match self {
            Self::Num0 | Self::KeyPad0 => T::num_0(),
            Self::Num1 | Self::KeyPad1 => T::num_1(),
            Self::Num2 | Self::KeyPad2 => T::num_2(),
            Self::Num3 | Self::KeyPad3 => T::num_3(),
            Self::Num4 | Self::KeyPad4 => T::num_4(),
            Self::Num5 | Self::KeyPad5 => T::num_5(),
            Self::Num6 | Self::KeyPad6 => T::num_6(),
            Self::Num7 | Self::KeyPad7 => T::num_7(),
            Self::Num8 | Self::KeyPad8 => T::num_8(),
            Self::Num9 | Self::KeyPad9 => T::num_9(),

            _ => return None,
        })
    }
}

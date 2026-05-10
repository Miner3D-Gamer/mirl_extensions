#![allow(clippy::approx_constant)]

use mirl_core::impl_trait;

use crate::*;

/// The value π for self
pub const trait ConstPi {
    /// The closest approximation of π for self
    const PI: Self;
}

impl const ConstPi for f128 {
    const PI: Self = 3.141_592_653_589_793_238_462_643_383_279_502_884_197_169_399_375_105_820_974_944_592_307_816_406_286_208_998_628_034_825_342_117_067_982_148_086_513_282_306_647_093_844_609_550_582_231_725_359_408_128_481;
}

impl_trait!(ConstPi {const PI: Self = f128::PI as Self;} for  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,f16,f32,f64 );

/// The full-circle constant τ = 2π for `Self`.
pub const trait ConstTau {
    /// The full-circle constant τ = 2π for `Self`.
    const TAU: Self;
}

impl<T: ConstPi + const core::ops::Add<Output = T>> const ConstTau for T {
    const TAU: Self = Self::PI + Self::PI;
}

/// π/2 for `Self`.
pub const trait ConstFracPi2 {
    /// π/2 for `Self`.
    const FRAC_PI_2: Self;
}

impl<T: ConstPi + const core::ops::Div<Output = T> + const FromPatch<u8>> const
    ConstFracPi2 for T
{
    const FRAC_PI_2: Self = Self::PI / T::from_value(2u8);
}

/// π/3 for `Self`.
pub const trait ConstFracPi3 {
    /// π/3 for `Self`.
    const FRAC_PI_3: Self;
}

impl<T: ConstPi + const core::ops::Div<Output = T> + const FromPatch<u8>> const
    ConstFracPi3 for T
{
    const FRAC_PI_3: Self = Self::PI / T::from_value(3u8);
}

/// π/4 for `Self`.
pub const trait ConstFracPi4 {
    /// π/4 for `Self`.
    const FRAC_PI_4: Self;
}

impl<T: ConstPi + const core::ops::Div<Output = T> + const FromPatch<u8>> const
    ConstFracPi4 for T
{
    const FRAC_PI_4: Self = Self::PI / T::from_value(4u8);
}

/// π/6 for `Self`.
pub const trait ConstFracPi6 {
    /// π/6 for `Self`.
    const FRAC_PI_6: Self;
}

impl<T: ConstPi + const core::ops::Div<Output = T> + const FromPatch<u8>> const
    ConstFracPi6 for T
{
    const FRAC_PI_6: Self = Self::PI / T::from_value(6u8);
}

/// π/8 for `Self`.
pub const trait ConstFracPi8 {
    /// π/8 for `Self`.
    const FRAC_PI_8: Self;
}

impl<T: ConstPi + const core::ops::Div<Output = T> + const FromPatch<u8>> const
    ConstFracPi8 for T
{
    const FRAC_PI_8: Self = Self::PI / T::from_value(8u8);
}

/// 1/π for `Self`.
pub const trait ConstFrac1Pi {
    /// 1/π for `Self`.
    const FRAC_1_PI: Self;
}

impl const ConstFrac1Pi for f128 {
    const FRAC_1_PI: Self = 0.318_309_886_183_790_671_537_767_526_745_028_724_068_919_291_480_912_897_495_334_688_117_793_595_268_453_070_180_227_605_532_506_171_912_145_685_453_515_916_316_849_467_786_892_184_724_882_613_572_966_735;
}

impl_trait!(ConstFrac1Pi { const FRAC_1_PI: Self = f128::FRAC_1_PI as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 2/π for `Self`.
pub const trait ConstFrac2Pi {
    /// 2/π for `Self`.
    const FRAC_2_PI: Self;
}

impl<T: ConstFrac1Pi + const core::ops::Add<Output = T> + Copy> const
    ConstFrac2Pi for T
{
    const FRAC_2_PI: Self = Self::FRAC_1_PI + Self::FRAC_1_PI;
}

/// 1/√π for `Self`.
pub const trait ConstFrac1SqrtPi {
    /// 1/√π for `Self`.
    const FRAC_1_SQRT_PI: Self;
}

impl const ConstFrac1SqrtPi for f128 {
    const FRAC_1_SQRT_PI: Self = 0.564_189_583_547_756_286_948_079_451_560_772_585_844_050_629_328_998_856_844_085_721_710_642_468_441_493_414_486_743_660_202_107_363_443_028_347_906_997_685_557_610_799_704_053_132_160_490_796_173_323_183;
}

impl_trait!(ConstFrac1SqrtPi { const FRAC_1_SQRT_PI: Self = f128::FRAC_1_SQRT_PI as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 1/√(2π) for `Self`.
pub const trait ConstFrac1Sqrt2Pi {
    /// 1/√(2π) for `Self`.
    const FRAC_1_SQRT_2PI: Self;
}

impl const ConstFrac1Sqrt2Pi for f128 {
    const FRAC_1_SQRT_2PI: Self = 0.398_942_280_401_432_677_939_946_059_934_381_868_475_858_631_164_934_657_665_925_829_670_657_925_899_301_838_501_252_333_907_306_936_430_302_558_863_635_916_791_231_979_536_054_296_508_827_720_380_217_803;
}

impl_trait!(ConstFrac1Sqrt2Pi { const FRAC_1_SQRT_2PI: Self = f128::FRAC_1_SQRT_2PI as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 2/√π for `Self`.
pub const trait ConstFrac2SqrtPi {
    /// 2/√π for `Self`.
    const FRAC_2_SQRT_PI: Self;
}

impl<T: ConstFrac1SqrtPi + const core::ops::Add<Output = T> + Copy> const
    ConstFrac2SqrtPi for T
{
    const FRAC_2_SQRT_PI: Self = Self::FRAC_1_SQRT_PI + Self::FRAC_1_SQRT_PI;
}

/// √2 for `Self`.
pub const trait ConstSqrt2 {
    /// √2 for `Self`.
    const SQRT_2: Self;
}

impl const ConstSqrt2 for f128 {
    const SQRT_2: Self = 1.414_213_562_373_095_048_801_688_724_209_698_078_569_671_875_376_948_073_176_679_737_990_732_478_462_107_038_850_387_534_327_641_572_735_013_846_230_912_297_024_924_836_055_850_737_212_644_121_497_099_936;
}

impl_trait!(ConstSqrt2 { const SQRT_2: Self = f128::SQRT_2 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 1/√2 for `Self`.
pub const trait ConstFrac1Sqrt2 {
    /// 1/√2 for `Self`.
    const FRAC_1_SQRT_2: Self;
}

impl const ConstFrac1Sqrt2 for f128 {
    // derived: 1/√2 = √2/2
    const FRAC_1_SQRT_2: Self = Self::SQRT_2 / 2.0;
}

impl_trait!(ConstFrac1Sqrt2 { const FRAC_1_SQRT_2: Self = f128::FRAC_1_SQRT_2 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// √3 for `Self`.
pub const trait ConstSqrt3 {
    /// √3 for `Self`.
    const SQRT_3: Self;
}

impl const ConstSqrt3 for f128 {
    const SQRT_3: Self = 1.732_050_807_568_877_293_527_446_341_505_872_366_942_805_253_810_380_628_055_006_811_842_539_275_453_904_092_885_952_734_459_534_823_697_552_271_428_031_459_564_690_450_929_735_892_046_697_906_929_736_761;
}

impl_trait!(ConstSqrt3 { const SQRT_3: Self = f128::SQRT_3 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 1/√3 for `Self`.
pub const trait ConstFrac1Sqrt3 {
    /// 1/√3 for `Self`.
    const FRAC_1_SQRT_3: Self;
}

impl const ConstFrac1Sqrt3 for f128 {
    // derived: 1/√3 = √3/3
    const FRAC_1_SQRT_3: Self = Self::SQRT_3 / 3.0;
}

impl_trait!(ConstFrac1Sqrt3 { const FRAC_1_SQRT_3: Self = f128::FRAC_1_SQRT_3 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// √5 for `Self`.
pub const trait ConstSqrt5 {
    /// √5 for `Self`.
    const SQRT_5: Self;
}

impl const ConstSqrt5 for f128 {
    const SQRT_5: Self = 2.236_067_977_499_789_696_409_173_668_731_276_235_440_618_359_611_525_724_270_897_245_410_520_925_637_804_899_414_414_408_378_782_274_969_508_176_022_283_768_454_473_382_952_960_985_865_757_825_634_441_397;
}

impl_trait!(ConstSqrt5 { const SQRT_5: Self = f128::SQRT_5 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 1/√5 for `Self`.
pub const trait ConstFrac1Sqrt5 {
    /// 1/√5 for `Self`.
    const FRAC_1_SQRT_5: Self;
}

impl const ConstFrac1Sqrt5 for f128 {
    // derived: 1/√5 = √5/5
    const FRAC_1_SQRT_5: Self = Self::SQRT_5 / 5.0;
}

impl_trait!(ConstFrac1Sqrt5 { const FRAC_1_SQRT_5: Self = f128::FRAC_1_SQRT_5 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// The golden ratio φ = (1 + √5) / 2 for `Self`.
pub const trait ConstGoldenRatio {
    /// The golden ratio φ = (1 + √5) / 2 for `Self`.
    const GOLDEN_RATIO: Self;
}

impl const ConstGoldenRatio for f128 {
    // derived from √5
    const GOLDEN_RATIO: Self = Self::midpoint(1.0, Self::SQRT_5);
}

impl_trait!(ConstGoldenRatio { const GOLDEN_RATIO: Self = f128::GOLDEN_RATIO as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// The Euler–Mascheroni constant γ for `Self`.
pub const trait ConstEulerGamma {
    /// The Euler–Mascheroni constant γ for `Self`.
    const EULER_GAMMA: Self;
}

impl const ConstEulerGamma for f128 {
    const EULER_GAMMA: Self = 0.577_215_664_901_532_860_606_512_090_082_402_431_042_159_335_939_923_598_805_767_234_884_867_726_777_664_670_936_947_063_291_746_749_514_631_447_249_807_082_480_960_504_014_486_542_836_224_173_997_644_923;
}

impl_trait!(ConstEulerGamma { const EULER_GAMMA: Self = f128::EULER_GAMMA as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// Euler's number e for `Self`.
pub const trait ConstE {
    /// Euler's number e for `Self`.
    const E: Self;
}

impl const ConstE for f128 {
    const E: Self = 2.718_281_828_459_045_235_360_287_471_352_662_497_757_247_093_699_959_574_966_967_627_724_076_630_353_547_594_571_382_178_525_166_427_427_466_391_932_003_059_921_817_413_596_629_043_572_900_334_295_260_595;
}

impl_trait!(ConstE { const E: Self = f128::E as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// log₂(e) for `Self`.
pub const trait ConstLog2E {
    /// log₂(e) for `Self`.
    const LOG2_E: Self;
}

impl const ConstLog2E for f128 {
    const LOG2_E: Self = 1.442_695_040_888_963_407_359_924_681_001_892_137_426_645_954_152_985_934_135_449_406_931_109_219_181_185_079_885_526_622_893_506_344_496_997_887_060_953_884_853_517_204_976_940_734_965_879_566_948_604_569;
}

impl_trait!(ConstLog2E { const LOG2_E: Self = f128::LOG2_E as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// log₂(10) for `Self`.
pub const trait ConstLog210 {
    /// log₂(10) for `Self`.
    const LOG2_10: Self;
}

impl const ConstLog210 for f128 {
    const LOG2_10: Self =
        3.321_928_094_887_362_347_870_319_429_489_390_175_864_831_393_024_580_612_054;
}

impl_trait!(ConstLog210 { const LOG2_10: Self = f128::LOG2_10 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// log₁₀(e) for `Self`.
pub const trait ConstLog10E {
    /// log₁₀(e) for `Self`.
    const LOG10_E: Self;
}

impl const ConstLog10E for f128 {
    const LOG10_E: Self = 0.434_294_481_903_251_827_651_128_918_916_605_082_294_397_005_803_666_566_114_453_783_165_864_649_208_870_774_729_224_949_338_431_748_318_706_106_744_766_303_733_641_679_252_815_521_765_691_916_292_720_703;
}

impl_trait!(ConstLog10E { const LOG10_E: Self = f128::LOG10_E as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// log₁₀(2) for `Self`.
///
/// Derived as 1 / log₂(10).
pub const trait ConstLog102 {
    /// log₁₀(2) for `Self`.
    ///
    /// Derived as 1 / log₂(10).
    const LOG10_2: Self;
}

impl const ConstLog102 for f128 {
    // derived: log₁₀(2) = 1 / log₂(10)
    const LOG10_2: Self = 1.0 / Self::LOG2_10;
}

impl_trait!(ConstLog102 { const LOG10_2: Self = f128::LOG10_2 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// ln(2) for `Self`.
pub const trait ConstLn2 {
    /// ln(2) for `Self`.
    const LN_2: Self;
}

impl const ConstLn2 for f128 {
    const LN_2: Self = 0.693_147_180_559_945_309_417_232_121_458_176_568_075_500_134_360_255_254_120_680_009_493_393_621_969_694_715_605_863_326_996_418_687_542_001_481_020_570_685_733_685_520_235_758_130_557_032_670_751_635_075;
}

impl_trait!(ConstLn2 { const LN_2: Self = f128::LN_2 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// ln(10) for `Self`.
///
/// Derived as ln(2) × log₂(10).
pub const trait ConstLn10 {
    /// ln(10) for `Self`.
    ///
    /// Derived as ln(2) × log₂(10).
    const LN_10: Self;
}

impl const ConstLn10 for f128 {
    // derived: ln(10) = ln(2) * log₂(10)
    const LN_10: Self = Self::LN_2 * Self::LOG2_10;
}

impl_trait!(ConstLn10 { const LN_10: Self = f128::LN_10 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// A single super trait that bundles every mathematical constant trait
pub const trait MathConsts:
    ConstPi
    + ConstTau
    + ConstFracPi2
    + ConstFracPi3
    + ConstFracPi4
    + ConstFracPi6
    + ConstFracPi8
    + ConstFrac1Pi
    + ConstFrac2Pi
    + ConstFrac1SqrtPi
    + ConstFrac1Sqrt2Pi
    + ConstFrac2SqrtPi
    + ConstSqrt2
    + ConstFrac1Sqrt2
    + ConstSqrt3
    + ConstFrac1Sqrt3
    + ConstSqrt5
    + ConstFrac1Sqrt5
    + ConstGoldenRatio
    + ConstEulerGamma
    + ConstE
    + ConstLog2E
    + ConstLog210
    + ConstLog10E
    + ConstLog102
    + ConstLn2
    + ConstLn10
{
}

// Blanket implementation: anything that satisfies all sub-traits gets MathConsts for free.
impl<
    T: const ConstPi
        + const ConstTau
        + const ConstFracPi2
        + const ConstFracPi3
        + const ConstFracPi4
        + const ConstFracPi6
        + const ConstFracPi8
        + const ConstFrac1Pi
        + const ConstFrac2Pi
        + const ConstFrac1SqrtPi
        + const ConstFrac1Sqrt2Pi
        + const ConstFrac2SqrtPi
        + const ConstSqrt2
        + const ConstFrac1Sqrt2
        + const ConstSqrt3
        + const ConstFrac1Sqrt3
        + const ConstSqrt5
        + const ConstFrac1Sqrt5
        + const ConstGoldenRatio
        + const ConstEulerGamma
        + const ConstE
        + const ConstLog2E
        + const ConstLog210
        + const ConstLog10E
        + const ConstLog102
        + const ConstLn2
        + const ConstLn10,
> const MathConsts for T
{
}
/// Convert between degrees and radians
///
/// [`DEGREES_TO_RADIAN_RATIO`](Self::DEGREES_TO_RADIAN_RATIO) π/180 - Multiply with this number to convert from degrees to radians
/// 
/// [`RADIANS_TO_DEGREES_RATIO`](Self::RADIANS_TO_DEGREES_RATIO) 180/π - Multiply with this number to convert from radians to degrees
pub const trait ConstRotationRatio {
    /// π/180 - Multiply with this number to convert from degrees to radians
    const DEGREES_TO_RADIAN_RATIO: Self;
    /// 180/π - Multiply with this number to convert from radians to degrees
    const RADIANS_TO_DEGREES_RATIO: Self;
}

impl<T: ConstPi + ConstNumbers256 + const std::ops::Div<Output = T>> const
    ConstRotationRatio for T
{
    const DEGREES_TO_RADIAN_RATIO: Self = T::PI / T::CONST_180;
    const RADIANS_TO_DEGREES_RATIO: Self = T::CONST_180 / T::PI;
}

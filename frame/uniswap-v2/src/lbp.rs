use crate::{BalanceOf, Config, Error, LbpWeightOf};
use composable_support::validation::NonZero;
use core::{
	convert::From,
	ops::{AddAssign, BitOrAssign, ShlAssign, Shr, ShrAssign},
};
use fixed::{
	traits::{FixedUnsigned, FromFixed, ToFixed},
	types::U89F39,
};
use sp_runtime::traits::{CheckedDiv, CheckedMul, Zero};

pub fn spot_price<T: Config>(
	in_reserve: NonZero<BalanceOf<T>>,
	out_reserve: BalanceOf<T>,
	in_weight: LbpWeightOf<T>,
	out_weight: LbpWeightOf<T>,
	amount: BalanceOf<T>,
) -> Result<BalanceOf<T>, Error<T>> {
	if amount.is_zero() || out_reserve.is_zero() {
		return Ok(Zero::zero())
	}

	let spot_price = amount
		.checked_mul(&out_reserve)
		.ok_or(Error::<T>::Overflow)?
		.checked_mul(&in_weight)
		.ok_or(Error::<T>::Overflow)?
		.checked_div(&in_reserve.checked_mul(&out_weight).ok_or(Error::<T>::Overflow)?)
		.ok_or(Error::<T>::Overflow)?;

	Ok(spot_price)
}

// fn convert_to_fixed<T: Config>(value: BalanceOf::<T>) -> U89F39 {
//     if value == BalanceOf::<T>::from(1u32) {
//         return U89F39::from_num::<i32>(1);
//     }

//     // Unwrap is safer here
//     let f = value.checked_div(HYDRA_ONE).unwrap();
//     let r = value - (f.checked_mul(HYDRA_ONE).unwrap());
//     FixedBalance::from_num(f) + (FixedBalance::from_num(r) / HYDRA_ONE)
// }

// fn convert_from_fixed(value: FixedBalance) -> Option<U89F39> {
//     let w: Balance = value.int().to_num();
//     let frac = value.frac();
//     let frac: Balance = frac.checked_mul_int(HYDRA_ONE)?.int().to_num();
//     let r = w.checked_mul(HYDRA_ONE)?.checked_add(frac)?;
//     Some(r)
// }

// #[macro_export]
// macro_rules! to_fixed_balance{
//     ($($x:expr),+) => (
//         {($(convert_to_fixed($x)),+)}
//     );
// }

// #[macro_export]
// macro_rules! to_balance_from_fixed {
//     ($x:expr) => {
//         convert_from_fixed($x).ok_or(Overflow)
//     };
// }

pub fn calculate_out_given_in<T: Config>(
	in_reserve: NonZero<BalanceOf<T>>,
	out_reserve: NonZero<BalanceOf<T>>,
	in_weight: NonZero<LbpWeightOf<T>>,
	out_weight: NonZero<LbpWeightOf<T>>,
	amount: NonZero<BalanceOf<T>>,
) -> Result<BalanceOf<T>, Error<T>> {
	let weight_ratio = in_weight.checked_div(&out_weight).ok_or(Error::<T>::Overflow)?;

	let one = U89F39::from_num::<i32>(1);
	let ir = one
		.checked_div(
			one.checked_add(
				amount.checked_div(&in_reserve).ok_or(Error::<T>::Overflow)?.to_fixed(),
			)
			.ok_or(Error::<T>::Overflow)?,
		)
		.ok_or(Error::<T>::Overflow)?;

	let ir = pow(ir, weight_ratio.to_fixed()).map_err(|_| Error::<T>::Overflow)?;

	let ir = U89F39::from_num::<i32>(1).checked_sub(ir).ok_or(Error::<T>::Overflow)?;

	let ir = BalanceOf::<T>::checked_from_fixed(ir).ok_or(Error::<T>::Math)?;
	let r = out_reserve.checked_mul(&ir).ok_or(Error::<T>::Overflow)?;
    Ok(r)
}

/// right-shift with rounding
fn rs<T>(operand: T) -> T
where
	T: FixedUnsigned,
{
	let lsb = T::from_num(1) >> T::FRAC_NBITS;
	(operand >> 1) + (operand & lsb)
}

/// base 2 logarithm assuming self >=1
fn log2_inner<S, D>(operand: S) -> D
where
	S: FixedUnsigned + PartialOrd<D>,
	D: FixedUnsigned,
	D::Bits: Copy + ToFixed + AddAssign + BitOrAssign + ShlAssign,
{
	let two = D::from_num(2);
	let mut x = operand;
	let mut result = D::from_num(0).to_bits();
	let lsb = (D::from_num(1) >> D::FRAC_NBITS).to_bits();

	while x >= two {
		result += lsb;
		x = rs(x);
	}

	if x == D::from_num(1) {
		return D::from_num(result)
	};

	for _i in (0..D::FRAC_NBITS).rev() {
		x *= x;
		result <<= lsb;
		if x >= two {
			result |= lsb;
			x = rs(x);
		}
	}
	D::from_bits(result)
}

/// base 2 logarithm
///
/// Returns tuple(D,bool) where bool indicates whether D is negative. This happens when operand is <
/// 1.
pub fn log2<S, D>(operand: S) -> Result<(D, bool), ()>
where
	S: FixedUnsigned,
	D: FixedUnsigned + From<S>,
	D::Bits: Copy + ToFixed + AddAssign + BitOrAssign + ShlAssign,
{
	if operand <= S::from_num(0) {
		return Err(())
	};

	let operand = D::from(operand);
	if operand < D::from_num(1) {
		let inverse = D::from_num(1).checked_div(operand).unwrap(); // Unwrap is safe because operand is always > 0
		return Ok((log2_inner::<D, D>(inverse), true))
	};
	Ok((log2_inner::<D, D>(operand), false))
}

/// natural logarithm
/// Returns tuple(D,bool) where bool indicates whether D is negative. This happens when operand is <
/// 1.
pub fn ln<S, D>(operand: S) -> Result<(D, bool), ()>
where
	S: FixedUnsigned,
	D: FixedUnsigned + From<S>,
	D::Bits: Copy + ToFixed + AddAssign + BitOrAssign + ShlAssign,
	S::Bits: Copy + ToFixed + AddAssign + BitOrAssign + ShrAssign + Shr,
{
	let log2_e = S::from_str("1.442695").map_err(|_| ())?;
	let log_result = log2::<S, D>(operand)?;
	Ok((log_result.0 / D::from(log2_e), log_result.1))
}

/// exponential function e^(operand)
/// neg - bool indicates that operand is negative value.
pub fn exp<S, D>(operand: S, neg: bool) -> Result<D, ()>
where
	S: FixedUnsigned + PartialOrd<D>,
	D: FixedUnsigned + PartialOrd<S> + From<S>,
{
	if operand.is_zero() {
		return Ok(D::from_num(1))
	};
	if operand == S::from_num(1) {
		//TODO: make this as const somewhere
		let e = S::from_str("2.718281828459045235360287471352662497757").map_err(|_| ())?;
		return Ok(D::from(e))
	};

	let operand = D::from(operand);
	let mut result = operand + D::from_num(1);
	let mut term = operand;

	result = (2..D::FRAC_NBITS).try_fold(result, |acc, i| -> Result<D, ()> {
		term = term.checked_mul(operand).ok_or(())?;
		term = term.checked_div(D::from_num(i)).ok_or(())?;
		acc.checked_add(term).ok_or(())
	})?;

	if neg {
		result = D::from_num(1).checked_div(result).ok_or(())?;
	}

	Ok(result)
}

pub fn pow<S, D>(operand: S, exponent: S) -> Result<D, ()>
where
	S: FixedUnsigned + PartialOrd<D>,
	D: FixedUnsigned + From<S>,
	D::Bits: Copy + ToFixed + AddAssign + BitOrAssign + ShlAssign,
	S::Bits: Copy + ToFixed + AddAssign + BitOrAssign + ShlAssign + Shr + ShrAssign,
{
	if operand.is_zero() {
		return Ok(D::from_num(0))
	};
	if exponent == S::from_num(0) {
		return Ok(D::from_num(1))
	};
	if exponent == S::from_num(1) {
		return Ok(D::from(operand))
	};

	let (r, neg) = ln::<S, D>(operand)?;

	let r: D = r.checked_mul(exponent.into()).ok_or(())?;
	let r: D = exp(r, neg)?;

	let (result, oflw) = r.overflowing_to_num::<D>();
	if oflw {
		return Err(())
	};
	Ok(result)
}

/// power with integer exponent
pub fn powi<S, D>(operand: S, exponent: u32) -> Result<D, ()>
where
	S: FixedUnsigned,
	D: FixedUnsigned + From<S>,
{
	if operand == S::from_num(0) {
		return Ok(D::from_num(0))
	};
	if exponent == 0 {
		return Ok(D::from_num(1))
	};
	if exponent == 1 {
		return Ok(D::from(operand))
	};
	let operand = D::from(operand);

	let r = (1..exponent).try_fold(operand, |acc, _| acc.checked_mul(operand));

	r.ok_or(())
}

#[cfg(test)]
mod tests {
	use core::str::FromStr;
	use approx::assert_relative_eq;
use fixed::{
		traits::LossyInto,
		types::{U64F64, U89F39},
	};

	use super::{exp, log2, pow, powi};

	#[test]
	fn exp_works() {
		type S = U64F64;
		type D = U64F64;

		let e = S::from_str("2.718281828459045235360287471352662497757").unwrap();

		let zero = S::from_num(0);
		let one = S::from_num(1);
		let two = S::from_num(2);

		assert_eq!(exp::<S, D>(zero, false), Ok(D::from_num(one)));
		assert_eq!(exp::<S, D>(one, false), Ok(D::from_num(e)));
		assert_eq!(exp::<S, D>(two, false), Ok(D::from_str("7.3890560989306502265").unwrap()));
		assert_eq!(exp::<S, D>(two, true), Ok(D::from_str("0.13533528323661269186").unwrap()));
	}

	#[test]
	fn log2_works() {
		type S = U64F64;
		type D = U64F64;

		let zero = S::from_num(0);
		let one = S::from_num(1);
		let two = S::from_num(2);
		let four = S::from_num(4);

		assert_eq!(log2::<S, D>(zero), Err(()));

		assert_eq!(log2(two), Ok((D::from_num(one), false)));
		assert_eq!(log2(one / four), Ok((D::from_num(two), true)));
		assert_eq!(log2(S::from_num(0.5)), Ok((D::from_num(one), true)));
		assert_eq!(log2(S::from_num(1.0 / 0.5)), Ok((D::from_num(one), false)));
	}

	#[test]
	fn powi_works() {
		type S = U64F64;
		type D = U64F64;

		let zero = S::from_num(0);
		let one = S::from_num(1);
		let two = S::from_num(2);
		let four = S::from_num(4);

		assert_eq!(powi(two, 0), Ok(D::from_num(one)));
		assert_eq!(powi(zero, 2), Ok(D::from_num(zero)));
		assert_eq!(powi(two, 1), Ok(D::from_num(2)));
		assert_eq!(powi(two, 3), Ok(D::from_num(8)));
		assert_eq!(powi(one / four, 2), Ok(D::from_num(0.0625)));
		assert_eq!(powi(S::from_num(2), 2), Ok(D::from_num(4)));
	}

	#[test]
	fn pow_works() {
		type S = U89F39;
		type D = U89F39;
		let zero = S::from_num(0);
		let one = S::from_num(1);
		let two = S::from_num(2);
		let three = S::from_num(3);
		let four = S::from_num(4);

		assert_eq!(pow::<S, D>(two, zero), Ok(one.into()));
		assert_eq!(pow::<S, D>(zero, two), Ok(zero.into()));

		let result: f64 = pow::<S, D>(two, three).unwrap().lossy_into();
		assert_relative_eq!(result, 8.0, epsilon = 1.0e-6);

		let result: f64 = pow::<S, D>(one / four, two).unwrap().lossy_into();
		assert_relative_eq!(result, 0.0625, epsilon = 1.0e-6);

		assert_eq!(pow::<S, D>(two, one), Ok(two.into()));

		let result: f64 = pow::<S, D>(one / four, one / two).unwrap().lossy_into();
		assert_relative_eq!(result, 0.5, epsilon = 1.0e-6);

		assert_eq!(pow(S::from_num(22.1234), S::from_num(2.1)), Ok(D::from_num(667.097035126091)));

		assert_eq!(
			pow(S::from_num(0.986069911074), S::from_num(1.541748732743)),
			Ok(D::from_num(0.978604513883))
		);
	}
}

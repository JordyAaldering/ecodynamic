use std::{io, str::FromStr};

use rand::distr::Distribution;
use rand_distr::Normal;

/// Curve families used to synthesize measurements.
///
/// Example TikZ to visualize the curves:
/// ```tex
/// \begin{tikzpicture}
/// \begin{axis}[
///   declare function={
///     f(\x) = \x;
///     g(\x) = 1 - \x;
///     score(\x,\alpha) = f(\x)^\alpha * g(\x)^(1 - \alpha);
///   },
/// ]
///   \addplot[domain=0:1,samples=100,color=energycolor]  {f(x)};
///   \addplot[domain=0:1,samples=100,color=runtimecolor] {g(x)};
///   \addplot[domain=0:1,samples=100,color=escorecolor]  {score(x,0.5)};
/// \end{axis}
/// \end{tikzpicture}
/// ```
#[derive(Clone, Copy, Debug)]
pub enum Curve {
	/// ```tex
	/// f(\x) = lb + (ub - lb) * \x};
	/// ```
	Linear {
		lb: f32,
		ub: f32,
	},
	/// ```tex
	/// f(\x) = lb + steepness * (\x - t_middle)^2;
	/// ```
	Quadratic {
        lb: f32,
		t_middle: f32,
        steepness: f32,
	},
	/// ```tex
	/// f(\x) = lb + (ub - lb) * 0.5 * (1 + tanh((\x - t_middle) * steepness));
	/// ```
	Sigmoid {
		lb: f32,
		ub: f32,
		t_middle: f32,
		steepness: f32,
	},
}

impl FromStr for Curve {
	type Err = io::Error;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let (variant, values) = input.split_once(':').unwrap();
		let mut values = values
			.split(',')
			.map(|s| s.parse::<f32>()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, s)));

		Ok(match variant {
			"Linear" => Curve::Linear {
				lb: values.next().unwrap()?,
				ub: values.next().unwrap()?,
			},
			"Quadratic" => Curve::Quadratic {
				lb: values.next().unwrap()?,
				t_middle: values.next().unwrap()?,
				steepness: values.next().unwrap()?,
			},
			"Sigmoid" => {
				Curve::Sigmoid {
					lb: values.next().unwrap()?,
					ub: values.next().unwrap()?,
					t_middle: values.next().unwrap()?,
					steepness: values.next().unwrap()?,
				}
			}
			_ => return Err(io::Error::new(io::ErrorKind::InvalidInput, input.to_string())),
		})
	}
}

impl Curve {
	pub fn eval(self, t: f32, cv: f32) -> f32 {
        debug_assert!(t >= 0.0 && t <= 1.0);
        debug_assert!(cv >= 0.0);
		let mean = match self {
			Curve::Linear { lb, ub } => {
                let v_delta = ub - lb;
				lb + v_delta * t
			}
			Curve::Quadratic { lb, t_middle, steepness } => {
                let t_delta = t - t_middle;
                lb + steepness * (t_delta).powi(2)
			}
			Curve::Sigmoid { lb, ub, t_middle, steepness } => {
                let v_delta = ub - lb;
                let t_delta = t - t_middle;
                lb + v_delta * (0.5 * (1.0 + f32::tanh(t_delta * steepness)))
			}
		};
        debug_assert!(mean >= 0.0);
        sample_normal_value(mean, cv)
	}
}

fn sample_normal_value(mean: f32, cv: f32) -> f32 {
	if cv <= 0.0 {
		return mean;
	}
	let std = mean * cv;
	let mut rng = rand::rng();
	let normal = Normal::new(mean, std).unwrap();
	let v = normal.sample(&mut rng);
    debug_assert!(v >= 0.0);
    v
}

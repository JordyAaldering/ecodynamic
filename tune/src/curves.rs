use std::{io, str::FromStr};

use rand::{RngExt, distr::Distribution};
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
			"Linear" => Self::Linear {
				lb: values.next().unwrap()?,
				ub: values.next().unwrap()?,
			},
			"Quadratic" => Self::Quadratic {
				lb: values.next().unwrap()?,
				t_middle: values.next().unwrap()?,
				steepness: values.next().unwrap()?,
			},
			"Sigmoid" => Self::Sigmoid {
				lb: values.next().unwrap()?,
				ub: values.next().unwrap()?,
				t_middle: values.next().unwrap()?,
				steepness: values.next().unwrap()?,
			},
			_ => return Err(io::Error::new(io::ErrorKind::InvalidInput, variant)),
		})
	}
}

impl ToString for Curve {
	fn to_string(&self) -> String {
		match self {
			Self::Linear { lb, ub } =>
				format!("Linear:{lb:.2},{ub:.2}"),
			Self::Quadratic { lb, t_middle, steepness } =>
				format!("Quadratic:{lb:.2},{t_middle:.2},{steepness:.2}"),
			Self::Sigmoid { lb, ub, t_middle, steepness } =>
				format!("Sigmoid:{lb:.2},{ub:.2},{t_middle:.2},{steepness:.2}"),
		}
	}
}

impl Curve {
	pub fn random() -> Self {
		let mut rng = rand::rng();
		// Deliberately make the linear variant less likely, as linear curves have less variation between them
		match rng.random_range(0..5) {
			0 => Self::Linear {
				lb: rng.random_range(0.0..=1.0),
				ub: rng.random_range(0.0..=1.0),
			},
			1 | 2 => Self::Quadratic {
				lb: rng.random_range(0.0..=1.0),
				t_middle: rng.random_range(0.0..=1.0),
				steepness: rng.random_range(-0.3..=3.0),
			},
			3 | 4 => Self::Sigmoid {
				lb: rng.random_range(0.0..=1.0),
				ub: rng.random_range(0.0..=1.0),
				t_middle: rng.random_range(0.0..=1.0),
				steepness: rng.random_range(-15.0..=15.0),
			},
			_ => unreachable!(),
		}
	}

	pub fn eval(&self, t: f32, cv: f32) -> f32 {
        assert!(t >= 0.0);
		assert!(t <= 1.0);
        assert!(cv >= 0.0);
		let v = match self {
			Self::Linear { lb, ub } => {
				lb + (ub - lb) * t
			}
			Self::Quadratic { lb, t_middle, steepness } => {
                let t_dt = t - t_middle;
                lb + steepness * (t_dt).powi(2)
			}
			Self::Sigmoid { lb, ub, t_middle, steepness } => {
                let v_dt = ub - lb;
                let t_dt = t - t_middle;
                lb + v_dt * (0.5 * (1.0 + f32::tanh(t_dt * steepness)))
			}
		};
        sample_normal_value(v.max(0.01), cv).max(f32::EPSILON)
	}

    pub fn to_tikz(&self) -> String {
        match self {
            Self::Linear { lb, ub } =>
                format!("{lb} + ({ub} - {lb}) * \\x"),
            Self::Quadratic { lb, t_middle, steepness } =>
                format!("{lb} + {steepness} * (\\x - {t_middle})^2"),
            Self::Sigmoid { lb, ub, t_middle, steepness } =>
                format!("{lb} + ({ub} - {lb}) * 0.5 * (1 + tanh((\\x - {t_middle}) * {steepness}))"),
        }
    }
}

fn sample_normal_value(mean: f32, cv: f32) -> f32 {
	if cv <= 0.0 {
		return mean;
	}
	let std = mean * cv;
	let mut rng = rand::rng();
	let normal = Normal::new(mean, std).unwrap();
	normal.sample(&mut rng)
}

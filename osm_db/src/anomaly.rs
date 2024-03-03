use crate::orbit::{self, MeanMotion, Perihelion, SemiAxis};

#[derive(Debug, Clone, Copy)]
/// This represents ways of describing an object in its orbit
pub struct Anomaly;

impl Anomaly {
    /// (Mean Anomaly) Calculates the period since the last periapsis.
    pub fn mean(self, day: f64, peri: Perihelion, orbital_period: f64) -> f64 {
        MeanMotion.by(day, peri, orbital_period).abs()
    }

    /// (Eccentric Anomaly) Calculates the body's position along its orbital path.
    ///
    /// * (HKE) Hyperbolic Kepler Equation
    /// > - ![Hyper Bolic Anomaly](https://latex.codecogs.com/svg.image?&space;e\sinh(H)H)
    /// > - ![Hyper Bolic Anomaly 0](https://latex.codecogs.com/svg.image?H_{k&plus;1}=H_k&plus;{\tfrac{M-e\sinh(H_k)&plus;H_k}{e\cosh(H_k)-1}})
    ///
    /// * (EKE) Elliptical Kepler Equation
    /// > - ![Mean Anomaly](https://latex.codecogs.com/svg.image?M=E-e\sin&space;E)
    /// > - ![Mean Anomaly Part 2](https://latex.codecogs.com/svg.image?f(E)=E-e\sin(E)-M(t))
    /// 
    /// * (PKE) Parabolic Kepler Equation
    /// > - ![Parabolic Kepler Equation](https://latex.codecogs.com/svg.image?q=p/2&space;)
    /// > - ![Parabolic Kepler Equation D](https://latex.codecogs.com/svg.image?D=D/\sqrt{2q})
    /// > - ![Parabolic Kepler Equation M](https://latex.codecogs.com/svg.image?M=qD&plus;(D^3/6))
    ///
    pub fn eccentric(
        self,
        shape: crate::orbit::Type,
        day: f64,
        orbital_eccentricity: f64,
        peri: Perihelion,
        orbital_period: f64,
        major_axis: f64,
    ) -> f64 {
        match shape {
            orbit::Type::Circular => {
                // Mean Anomaly
                let xref = self.mean(day, peri, orbital_period);

                // v = M = E
                xref
            }
            orbit::Type::Parabolic => {
                // Initial Pn which allows for precesion
                let mut pdx = 10.0;

                // Mean Anomaly
                let xref = self.mean(day, peri, orbital_period);

                // Initial Parabolic Anomaly
                let mut px0 = xref;

                // Newtons Iterative Step
                while pdx > 1.0e-7 {
                    let x0 = px0.powf(3.0);
                    let x1 = 6.0;

                    pdx = x0 / x1;

                    // Semi-Latus Rectum ( semji-major-axis * (1.0 - eccentricity^2))
                    let p =
                        SemiAxis(major_axis).major() * (1.0_f64 - orbital_eccentricity.powf(2.0));

                    // (Perifocal Distance) q = p/2
                    let q = p / 2.0;

                    // M = qD + (D^3 / 6)
                    px0 = (q * px0) + pdx;
                }

                let mean_motion = MeanMotion.by(day, peri, orbital_period);
                // makes sure that the mean motion isn't negative
                if mean_motion < 0.0 {
                    px0 = -px0;
                }

                px0
            }
            orbit::Type::Hyperbolic => {
                // Initial Hn which allows for precesion
                let mut hdx = 10.0;

                // Mean Anomaly
                let xref = self.mean(day, peri, orbital_period);

                // Initial Hyperbolic Anomaly
                let mut hx0 = xref;

                // Newtons Iterative Step
                while hdx > 1.0e-7 {
                    // M-esinh(Hk)+Hk
                    let x0 = (xref - orbital_eccentricity) * hx0.sinh() + hx0;

                    // ecosh(Hk)-1
                    let x1 = orbital_eccentricity * hx0.cosh() - 1.0;

                    // (M-esinh(Hk)+Hk)/(ecosh(Hk)-1)
                    hdx = x0 / x1;

                    // Hk+1 = Hk + (M-esinh(Hk)+Hk)/(ecosh(Hk)-1)
                    hx0 = hx0 + hdx;
                }

                let mean_motion = MeanMotion.by(day, peri, orbital_period);

                // makes sure that the mean motion isn't negative
                if mean_motion < 0.0 {
                    hx0 = -hx0;
                }

                hx0
            }
            orbit::Type::Elliptical => {
                // Initial En which allows for precesion
                let mut zdx: f64 = 10.0;

                // Mean Anomaly
                let xref = self.mean(day, peri, orbital_period);

                // Initial Eccentric Anomaly
                let mut zx0 = xref + orbital_eccentricity * xref.sin();

                // Newtons Iterative step
                while zdx > 1.0e-7 {
                    let x0 = -(zx0 - orbital_eccentricity * zx0.sin() - xref);
                    let x1 = 1.0 - orbital_eccentricity * zx0.cos();

                    // En = - ((En - e * En.sin() - M(t)) / 1 - e * En.cos() )
                    // the En at its first increment En = E0
                    zdx = x0 / x1;

                    // En = En + En+1
                    zx0 = zx0 + zdx;
                }

                let mean_motion = MeanMotion.by(day, peri, orbital_period);

                // makes sure that the mean motion isn't negative
                if mean_motion < 0.0 {
                    zx0 = -zx0;
                }

                // println!("zx0: {:?}", zx0);

                zx0
            }
            _ => 0.0,
        }
    }

    /// (True Anomaly) Calculates the angle between the periapsis and the body's current position.
    ///
    /// * Elliptical Eccentric Anomaly
    /// > - ![Elliptical Eccentric Anomaly](https://latex.codecogs.com/svg.image?\nu=2\,\operatorname{arctan}\left(\,{\sqrt{{1&plus;e\,}\over{1-e\,}}}\tan{E\over&space;2}\,\right))
    ///
    /// * Hyperbolic (Eccentric) Anomaly
    /// > - ![Hyperbolic Anomaly](https://latex.codecogs.com/svg.image?(\frac{e&plus;1}{e-1})^{1/2}\tanh(\frac{H}{2}))
    ///
    /// * Parabolic (Eccentric) Anomaly
    /// > - ![Parabolic Anomaly](https://latex.codecogs.com/svg.image?D=D/\sqrt{2q})
    ///
    /// * Circular (Eccentric) Anomaly
    /// > - ![Circular Eccentric Anomaly](https://latex.codecogs.com/svg.image?nt=M(t))
    /// > - ![Circular Eccentric Anomaly](https://latex.codecogs.com/svg.image?M=M_0&plus;nt&space;)
    ///
    pub fn truly(
        self,
        shape: crate::orbit::Type,
        day: f64,
        orbital_eccentricity: f64,
        peri: Perihelion,
        orbital_period: f64,
        major_axis: f64,
    ) -> f64 {
        match shape {
            orbit::Type::Circular => {
                let mut theta: f64 = self.eccentric(
                    shape,
                    day,
                    orbital_eccentricity,
                    peri,
                    orbital_period,
                    major_axis,
                );

                let mean_motion = MeanMotion.by(day, peri, orbital_period);

                theta = theta + mean_motion;

                theta
            }
            orbit::Type::Parabolic => {
                let theta: f64 = self.eccentric(
                    shape,
                    day,
                    orbital_eccentricity,
                    peri,
                    orbital_period,
                    major_axis,
                );
                let p = 0.0;
                let q = p / 2.0_f64;

                theta / (2.0_f64 * q).sqrt()
            }
            orbit::Type::Hyperbolic => {
                let theta: f64 = self.eccentric(
                    shape,
                    day,
                    orbital_eccentricity,
                    peri,
                    orbital_period,
                    major_axis,
                );

                // tan v/2 = (e+1/e-1)^1/2 * tanh(F/2)
                // `where F = H`
                ((orbital_eccentricity + 1.0) / (orbital_eccentricity - 1.0)).powf(0.5)
                    * (theta / 2.0).tanh()
            }
            orbit::Type::Elliptical => {
                let theta: f64 = self.eccentric(
                    shape,
                    day,
                    orbital_eccentricity,
                    peri,
                    orbital_period,
                    major_axis,
                );

                // println!("zx0: {:?}", theta);

                let mean_motion =
                    ((1.0 + orbital_eccentricity) / (1.0 - orbital_eccentricity)).sqrt();

                2.0 * (mean_motion * (theta / 2.0).tan()).atan()
            }
            _ => 0.0,
        }
    }
}

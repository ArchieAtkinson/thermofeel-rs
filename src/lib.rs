use std::f64::consts::PI;

mod helpers;

pub use helpers::*;

/// Calculates relative humidity from temperatures and dew point temperature.
///
/// Where `t2_k` is the temperature at 2 meters in Kelvin.
///
/// Where `td_k` is the dew point temperature in Kelvin.
///
/// The return value is relative humidity as a percentage.
///
/// Reference: <https://www.theweatherprediction.com/habyhints/186/>
pub fn calculate_relative_humidity_percent(t2_k: f64, td_k: f64) -> f64 {
    let t2_c = kelvin_to_celsius(t2_k);
    let td_c = kelvin_to_celsius(td_k);

    let es = 6.11 * f64::from(10.0).powf(7.5 * t2_c / (237.3 + t2_c));

    let e = (6.11) * f64::from(10.0).powf(7.5 * td_c / (237.3 + td_c));
    println!("{t2_c} {td_c}");
    (e / es) * 100.0
}

/// Calculates saturation vapour pressure over water.
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// The return value is saturation vapor pressure over water in the pure phase in hPa (mBar).
///
/// Reference: Hardy (1998) [https://www.decatur.de/javascript/dew/resources/its90formulas.pdf](https://www.decatur.de/javascript/dew/resources/its90formulas.pdf)
pub fn calculate_saturation_vapour_pressure(t2_k: f64) -> f64 {
    let g = [
        -2.8365744e3,
        -6.028076559e3,
        1.954263612e1,
        -2.737830188e-2,
        1.6261698e-5,
        7.0229056e-10,
        -1.8680009e-13,
        2.7150305,
    ];

    let mut ess = g[7] * t2_k.ln();
    for i in 0..g.len() - 1 {
        ess = ess + g[i] * t2_k.powi(i as i32 - 2);
    }

    ess = ess.exp() * 0.01; // hPa

    ess
}

/// Represents the phase of water for saturation vapor pressure calculations.
pub enum Phase {
    /// Liquid water phase.
    Liquid,
    /// Ice phase.
    Ice,
}

/// Calculates saturation vapour pressure over liquid water or ice.
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `phase` specifies whether to calculate over liquid water or ice.
///
/// The return value is the pressure of water vapor over a surface of liquid water or ice in hPa (mBar).
///
/// Reference: ECMWF IFS Documentation CY45R1 - Part IV : Physical processes (2018) pp. 116 [https://doi.org/10.21957/4whwo8jw0](https://doi.org/10.21957/4whwo8jw0)
///
/// See also: [https://metview.readthedocs.io/en/latest/api/functions/saturation_vapour_pressure.html](https://metview.readthedocs.io/en/latest/api/functions/saturation_vapour_pressure.html)
pub fn calculate_saturation_vapour_pressure_multiphase(t2_k: f64, phase: Phase) -> f64 {
    let t0 = 273.16; // triple point of water 273.16 K (0.01 °C) at 611.73 Pa

    match phase {
        Phase::Liquid => {
            let y = (t2_k - t0) / (t2_k - 32.19);
            6.1121 * (17.502 * y).exp()
        }
        Phase::Ice => {
            let y = (t2_k - t0) / (t2_k + 0.7);
            6.1121 * (22.587 * y).exp()
        }
    }
}

/// Calculates non-saturated vapour pressure.
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `rh` is the relative humidity percentage.
///
/// The return value is the non-saturated vapor pressure in hPa (mBar).
///
/// Reference: Bureau of Meteorology (2010) [http://www.bom.gov.au/info/thermal_stress/#approximation](http://www.bom.gov.au/info/thermal_stress/#approximation)
pub fn calculate_nonsaturation_vapour_pressure(t2_k: f64, rh: f64) -> f64 {
    let t2_c = kelvin_to_celsius(t2_k);
    rh / 100.0 * 6.105 * (17.27 * t2_c / (237.7 + t2_c)).exp()
}

/// Scales wind speed from 10 meters to a specified height.
///
/// Where `va` is the 10m wind speed in m/s.
///
/// Where `h` is the target height in meters at which wind speed needs to be scaled.
///
/// The return value is the wind speed at height `h`.
///
/// Reference: Bröde et al. (2012) [https://doi.org/10.1007/s00484-011-0454-1](https://doi.org/10.1007/s00484-011-0454-1)
pub fn scale_windspeed(va: f64, h: f64) -> f64 {
    let target_height = 10.0;
    let c = 1.0 / f64::from(target_height / 0.01).log10();
    let vh = va * (h / 0.01).log10() * c;

    return vh;
}

/// Approximates direct solar radiation from total sky direct solar radiation and cosine of solar zenith angle.
///
/// Note that the function introduces large errors as `cossza` approaches zero.
/// Only use if `dsrp` is not available in your dataset.
///
/// Where `fdir` is the total sky direct solar radiation at surface in W m-2.
///
/// Where `cossza` is the cosine of the solar zenith angle (dimensionless).
///
/// The return value is direct radiation from the Sun in W m-2, or `None` if `cossza` is too small.
pub fn approximate_dsrp(fdir: f64, cossza: f64) -> Option<f64> {
    if cossza <= 0.1 {
        None
    } else {
        Some(fdir / cossza)
    }
}

/// Calculates dew point temperature at 2m from relative humidity.
///
/// Where `rh` is the relative humidity in percent.
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// The return value is the dew point temperature in Kelvin.
///
/// Reference: Alduchov and Eskridge (1996) [https://doi.org/10.1175/1520-0450(1996)035<0601:IMFAOS>2.0.CO;2](https://doi.org/10.1175/1520-0450(1996)035<0601:IMFAOS>2.0.CO;2)
pub fn calculate_dew_point_from_relative_humidity(rh: f64, t2_k: f64) -> f64 {
    let t2_c = kelvin_to_celsius(t2_k);
    let td_c = 243.04 * ((rh / 100.0).ln() + ((17.625 * t2_c) / (243.04 + t2_c)))
        / (17.625 - (rh / 100.0).ln() - ((17.625 * t2_c) / (243.04 + t2_c)));
    celsius_to_kelvin(td_c)
}

/// Calculates Mean Radiant Temperature (MRT).
///
/// Where `ssrd` is the surface solar radiation downwards in W m-2.
///
/// Where `ssr` is the surface net solar radiation in W m-2.
///
/// Where `dsrp` is the direct solar radiation in W m-2.
///
/// Where `strd` is the surface thermal radiation downwards in W m-2.
///
/// Where `fdir` is the total sky direct solar radiation at surface in W m-2.
///
/// Where `strr` is the surface net thermal radiation in W m-2.
///
/// Where `cossza` is the cosine of the solar zenith angle (dimensionless).
///
/// The return value is the mean radiant temperature in Kelvin.
///
/// Reference: Di Napoli et al. (2020) [https://link.springer.com/article/10.1007/s00484-020-01900-5](https://link.springer.com/article/10.1007/s00484-020-01900-5)
pub fn calculate_mean_radiant_temperature(
    ssrd: f64,
    ssr: f64,
    dsrp: f64,
    strd: f64,
    fdir: f64,
    strr: f64,
    cossza: f64,
) -> f64 {
    let dsw = ssrd - fdir;
    let rsw = ssrd - ssr;
    let lur = strd - strr;

    let gamma = cossza.asin() * 180.0 / PI;
    let fp = 0.308 * ((PI / 180.0) * gamma * (0.998 - gamma * gamma / 50000.0)).cos();

    let mrt = ((1.0 / 0.0000000567)
        * (0.5 * strd + 0.5 * lur + (0.7 / 0.97) * (0.5 * dsw + 0.5 * rsw + fp * dsrp)))
        .powf(0.25);

    return mrt;
}

/// Helper function to calculate the UTCI polynomial approximation.
///
/// Where `t2m` is the 2m temperature in Kelvin.
///
/// Where `mrt` is the mean radiant temperature in Kelvin.
///
/// Where `va` is the wind speed at 10 meters in m/s.
///
/// Where `wvp` is the water vapour pressure in kPa.
///
/// The return value is UTCI in Kelvin.
///
/// Reference: Brode et al. (2012) [https://doi.org/10.1007/s00484-011-0454-1](https://doi.org/10.1007/s00484-011-0454-1)
fn calculate_utci_polynomial(t2m: f64, mrt: f64, va: f64, wvp: f64) -> f64 {
    let e_mrt = mrt - t2m;

    let t2m2 = t2m * t2m;
    let t2m3 = t2m2 * t2m;
    let t2m4 = t2m3 * t2m;
    let t2m5 = t2m4 * t2m;
    let t2m6 = t2m5 * t2m;

    let va2 = va * va;
    let va3 = va2 * va;
    let va4 = va3 * va;
    let va5 = va4 * va;
    let va6 = va5 * va;

    let e_mrt2 = e_mrt * e_mrt;
    let e_mrt3 = e_mrt2 * e_mrt;
    let e_mrt4 = e_mrt3 * e_mrt;
    let e_mrt5 = e_mrt4 * e_mrt;
    let e_mrt6 = e_mrt5 * e_mrt;

    let wvp2 = wvp * wvp;
    let wvp3 = wvp2 * wvp;
    let wvp4 = wvp3 * wvp;
    let wvp5 = wvp4 * wvp;
    let wvp6 = wvp5 * wvp;

    let varh2 = va * wvp2;
    let va2_rh = va2 * wvp;
    let va2_e_mrt = va2 * e_mrt;
    let e_mrt_rh = e_mrt * wvp;
    let e_mrt_rh2 = e_mrt * wvp2;
    let e_mrt2_rh = e_mrt2 * wvp;
    let e_mrt2_rh2 = e_mrt2 * wvp2;
    let e_mrt_rh3 = e_mrt * wvp3;
    let va_e_mrt = va * e_mrt;
    let va_e_mrt2 = va * e_mrt2;
    let va_rh = va * wvp;
    let t2m_va = t2m * va;
    let e_mrt3_rh = e_mrt3 * wvp;
    let e_mrt4_rh = e_mrt4 * wvp;

    let utci = t2m
        + 6.07562052e-01
        + -2.27712343e-02 * t2m
        + 8.06470249e-04 * t2m2
        + -1.54271372e-04 * t2m3
        + -3.24651735e-06 * t2m4
        + 7.32602852e-08 * t2m5
        + 1.35959073e-09 * t2m6
        + -2.25836520e00 * va
        + 8.80326035e-02 * t2m * va
        + 2.16844454e-03 * t2m2 * va
        + -1.53347087e-05 * t2m3 * va
        + -5.72983704e-07 * t2m4 * va
        + -2.55090145e-09 * t2m5 * va
        + -7.51269505e-01 * va2
        + -4.08350271e-03 * t2m * va2
        + -5.21670675e-05 * t2m2 * va2
        + 1.94544667e-06 * t2m3 * va2
        + 1.14099531e-08 * t2m4 * va2
        + 1.58137256e-01 * va3
        + -6.57263143e-05 * t2m * va3
        + 2.22697524e-07 * t2m2 * va3
        + -4.16117031e-08 * t2m3 * va3
        + -1.27762753e-02 * va4
        + 9.66891875e-06 * t2m * va4
        + 2.52785852e-09 * t2m2 * va4
        + 4.56306672e-04 * va5
        + -1.74202546e-07 * t2m * va5
        + -5.91491269e-06 * va6
        + 3.98374029e-01 * e_mrt
        + 1.83945314e-04 * t2m * e_mrt
        + -1.73754510e-04 * t2m2 * e_mrt
        + -7.60781159e-07 * t2m3 * e_mrt
        + 3.77830287e-08 * t2m4 * e_mrt
        + 5.43079673e-10 * t2m5 * e_mrt
        + -2.00518269e-02 * va_e_mrt
        + 8.92859837e-04 * t2m * va_e_mrt
        + 3.45433048e-06 * t2m2 * va_e_mrt
        + -3.77925774e-07 * t2m3 * va_e_mrt
        + -1.69699377e-09 * t2m4 * va_e_mrt
        + 1.69992415e-04 * va2_e_mrt
        + -4.99204314e-05 * t2m * va2_e_mrt
        + 2.47417178e-07 * t2m2 * va2_e_mrt
        + 1.07596466e-08 * t2m3 * va2_e_mrt
        + 8.49242932e-05 * va3 * e_mrt
        + 1.35191328e-06 * t2m * va3 * e_mrt
        + -6.21531254e-09 * t2m2 * va3 * e_mrt
        + -4.99410301e-06 * va4 * e_mrt
        + -1.89489258e-08 * t2m * va4 * e_mrt
        + 8.15300114e-08 * va5 * e_mrt
        + 7.55043090e-04 * e_mrt2
        + -5.65095215e-05 * t2m * e_mrt2
        + -4.52166564e-07 * t2m2 * e_mrt2
        + 2.46688878e-08 * t2m3 * e_mrt2
        + 2.42674348e-10 * t2m4 * e_mrt2
        + 1.54547250e-04 * va_e_mrt2
        + 5.24110970e-06 * t2m * va_e_mrt2
        + -8.75874982e-08 * t2m2 * va_e_mrt2
        + -1.50743064e-09 * t2m3 * va_e_mrt2
        + -1.56236307e-05 * va2 * e_mrt2
        + -1.33895614e-07 * t2m * va2 * e_mrt2
        + 2.49709824e-09 * t2m2 * va2 * e_mrt2
        + 6.51711721e-07 * va3 * e_mrt2
        + 1.94960053e-09 * t2m * va3 * e_mrt2
        + -1.00361113e-08 * va4 * e_mrt2
        + -1.21206673e-05 * e_mrt3
        + -2.18203660e-07 * t2m * e_mrt3
        + 7.51269482e-09 * t2m2 * e_mrt3
        + 9.79063848e-11 * t2m3 * e_mrt3
        + 1.25006734e-06 * va * e_mrt3
        + -1.81584736e-09 * t2m_va * e_mrt3
        + -3.52197671e-10 * t2m2 * va * e_mrt3
        + -3.36514630e-08 * va2 * e_mrt3
        + 1.35908359e-10 * t2m * va2 * e_mrt3
        + 4.17032620e-10 * va3 * e_mrt3
        + -1.30369025e-09 * e_mrt4
        + 4.13908461e-10 * t2m * e_mrt4
        + 9.22652254e-12 * t2m2 * e_mrt4
        + -5.08220384e-09 * va * e_mrt4
        + -2.24730961e-11 * t2m_va * e_mrt4
        + 1.17139133e-10 * va2 * e_mrt4
        + 6.62154879e-10 * e_mrt5
        + 4.03863260e-13 * t2m * e_mrt5
        + 1.95087203e-12 * va * e_mrt5
        + -4.73602469e-12 * e_mrt6
        + 5.12733497e00 * wvp
        + -3.12788561e-01 * t2m * wvp
        + -1.96701861e-02 * t2m2 * wvp
        + 9.99690870e-04 * t2m3 * wvp
        + 9.51738512e-06 * t2m4 * wvp
        + -4.66426341e-07 * t2m5 * wvp
        + 5.48050612e-01 * va_rh
        + -3.30552823e-03 * t2m * va_rh
        + -1.64119440e-03 * t2m2 * va_rh
        + -5.16670694e-06 * t2m3 * va_rh
        + 9.52692432e-07 * t2m4 * va_rh
        + -4.29223622e-02 * va2_rh
        + 5.00845667e-03 * t2m * va2_rh
        + 1.00601257e-06 * t2m2 * va2_rh
        + -1.81748644e-06 * t2m3 * va2_rh
        + -1.25813502e-03 * va3 * wvp
        + -1.79330391e-04 * t2m * va3 * wvp
        + 2.34994441e-06 * t2m2 * va3 * wvp
        + 1.29735808e-04 * va4 * wvp
        + 1.29064870e-06 * t2m * va4 * wvp
        + -2.28558686e-06 * va5 * wvp
        + -3.69476348e-02 * e_mrt_rh
        + 1.62325322e-03 * t2m * e_mrt_rh
        + -3.14279680e-05 * t2m2 * e_mrt_rh
        + 2.59835559e-06 * t2m3 * e_mrt_rh
        + -4.77136523e-08 * t2m4 * e_mrt_rh
        + 8.64203390e-03 * va * e_mrt_rh
        + -6.87405181e-04 * t2m_va * e_mrt_rh
        + -9.13863872e-06 * t2m2 * va * e_mrt_rh
        + 5.15916806e-07 * t2m3 * va * e_mrt_rh
        + -3.59217476e-05 * va2 * e_mrt_rh
        + 3.28696511e-05 * t2m * va2 * e_mrt_rh
        + -7.10542454e-07 * t2m2 * va2 * e_mrt_rh
        + -1.24382300e-05 * va3 * e_mrt_rh
        + -7.38584400e-09 * t2m * va3 * e_mrt_rh
        + 2.20609296e-07 * va4 * e_mrt_rh
        + -7.32469180e-04 * e_mrt2_rh
        + -1.87381964e-05 * t2m * e_mrt2_rh
        + 4.80925239e-06 * t2m2 * e_mrt2_rh
        + -8.75492040e-08 * t2m3 * e_mrt2_rh
        + 2.77862930e-05 * va * e_mrt2_rh
        + -5.06004592e-06 * t2m_va * e_mrt2_rh
        + 1.14325367e-07 * t2m2 * va * e_mrt2_rh
        + 2.53016723e-06 * va2 * e_mrt2_rh
        + -1.72857035e-08 * t2m * va2 * e_mrt2_rh
        + -3.95079398e-08 * va3 * e_mrt2_rh
        + -3.59413173e-07 * e_mrt3_rh
        + 7.04388046e-07 * t2m * e_mrt3_rh
        + -1.89309167e-08 * t2m2 * e_mrt3_rh
        + -4.79768731e-07 * va * e_mrt3_rh
        + 7.96079978e-09 * t2m_va * e_mrt3_rh
        + 1.62897058e-09 * va2 * e_mrt3_rh
        + 3.94367674e-08 * e_mrt4_rh
        + -1.18566247e-09 * t2m * e_mrt4_rh
        + 3.34678041e-10 * va * e_mrt4_rh
        + -1.15606447e-10 * e_mrt5 * wvp
        + -2.80626406e00 * wvp2
        + 5.48712484e-01 * t2m * wvp2
        + -3.99428410e-03 * t2m2 * wvp2
        + -9.54009191e-04 * t2m3 * wvp2
        + 1.93090978e-05 * t2m4 * wvp2
        + -3.08806365e-01 * varh2
        + 1.16952364e-02 * t2m * varh2
        + 4.95271903e-04 * t2m2 * varh2
        + -1.90710882e-05 * t2m3 * varh2
        + 2.10787756e-03 * va2 * wvp2
        + -6.98445738e-04 * t2m * va2 * wvp2
        + 2.30109073e-05 * t2m2 * va2 * wvp2
        + 4.17856590e-04 * va3 * wvp2
        + -1.27043871e-05 * t2m * va3 * wvp2
        + -3.04620472e-06 * va4 * wvp2
        + 5.14507424e-02 * e_mrt_rh2
        + -4.32510997e-03 * t2m * e_mrt_rh2
        + 8.99281156e-05 * t2m2 * e_mrt_rh2
        + -7.14663943e-07 * t2m3 * e_mrt_rh2
        + -2.66016305e-04 * va * e_mrt_rh2
        + 2.63789586e-04 * t2m_va * e_mrt_rh2
        + -7.01199003e-06 * t2m2 * va * e_mrt_rh2
        + -1.06823306e-04 * va2 * e_mrt_rh2
        + 3.61341136e-06 * t2m * va2 * e_mrt_rh2
        + 2.29748967e-07 * va3 * e_mrt_rh2
        + 3.04788893e-04 * e_mrt2_rh2
        + -6.42070836e-05 * t2m * e_mrt2_rh2
        + 1.16257971e-06 * t2m2 * e_mrt2_rh2
        + 7.68023384e-06 * va * e_mrt2_rh2
        + -5.47446896e-07 * t2m_va * e_mrt2_rh2
        + -3.59937910e-08 * va2 * e_mrt2_rh2
        + -4.36497725e-06 * e_mrt3 * wvp2
        + 1.68737969e-07 * t2m * e_mrt3 * wvp2
        + 2.67489271e-08 * va * e_mrt3 * wvp2
        + 3.23926897e-09 * e_mrt4 * wvp2
        + -3.53874123e-02 * wvp3
        + -2.21201190e-01 * t2m * wvp3
        + 1.55126038e-02 * t2m2 * wvp3
        + -2.63917279e-04 * t2m3 * wvp3
        + 4.53433455e-02 * va * wvp3
        + -4.32943862e-03 * t2m_va * wvp3
        + 1.45389826e-04 * t2m2 * va * wvp3
        + 2.17508610e-04 * va2 * wvp3
        + -6.66724702e-05 * t2m * va2 * wvp3
        + 3.33217140e-05 * va3 * wvp3
        + -2.26921615e-03 * e_mrt_rh3
        + 3.80261982e-04 * t2m * e_mrt_rh3
        + -5.45314314e-09 * t2m2 * e_mrt_rh3
        + -7.96355448e-04 * va * e_mrt_rh3
        + 2.53458034e-05 * t2m_va * e_mrt_rh3
        + -6.31223658e-06 * va2 * e_mrt_rh3
        + 3.02122035e-04 * e_mrt2 * wvp3
        + -4.77403547e-06 * t2m * e_mrt2 * wvp3
        + 1.73825715e-06 * va * e_mrt2 * wvp3
        + -4.09087898e-07 * e_mrt3 * wvp3
        + 6.14155345e-01 * wvp4
        + -6.16755931e-02 * t2m * wvp4
        + 1.33374846e-03 * t2m2 * wvp4
        + 3.55375387e-03 * va * wvp4
        + -5.13027851e-04 * t2m_va * wvp4
        + 1.02449757e-04 * va2 * wvp4
        + -1.48526421e-03 * e_mrt * wvp4
        + -4.11469183e-05 * t2m * e_mrt * wvp4
        + -6.80434415e-06 * va * e_mrt * wvp4
        + -9.77675906e-06 * e_mrt2 * wvp4
        + 8.82773108e-02 * wvp5
        + -3.01859306e-03 * t2m * wvp5
        + 1.04452989e-03 * va * wvp5
        + 2.47090539e-04 * e_mrt * wvp5
        + 1.48348065e-03 * wvp6;

    utci
}

/// Calculates the Universal Thermal Climate Index (UTCI).
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `va` is the wind speed at 10 meters in m/s.
///
/// Where `mrt` is the mean radiant temperature in Kelvin.
///
/// Where `td_k` is an optional 2m dew point temperature in Kelvin.
///
/// Where `eh_pa` is an optional water vapour pressure in hPa.
///
/// The return value is UTCI in Kelvin.
///
/// Reference: Brode et al. (2012) [https://doi.org/10.1007/s00484-011-0454-1](https://doi.org/10.1007/s00484-011-0454-1)
pub fn calculate_utci(t2_k: f64, va: f64, mrt: f64, td_k: Option<f64>, eh_pa: Option<f64>) -> f64 {
    let wvp: f64;

    if let Some(eh_pa) = eh_pa {
        wvp = eh_pa / 10.0; // water vapour pressure in kPa
    } else {
        if let Some(td_k) = td_k {
            let rh_pc = calculate_relative_humidity_percent(t2_k, td_k);
            let eh_pa = calculate_saturation_vapour_pressure(t2_k) * rh_pc / 100.0;
            wvp = eh_pa / 10.0; // water vapour pressure in kPa
        } else {
            panic!("Missing input ehPa or td_k");
        }
    }

    let t2_c = kelvin_to_celsius(t2_k);
    let mrt_c = kelvin_to_celsius(mrt);

    let utci = calculate_utci_polynomial(t2_c, mrt_c, va, wvp);
    let utci_k = celsius_to_kelvin(utci);

    return utci_k;
}

/// Calculates Wet Bulb Globe Temperature (WBGT) using a simplified algorithm.
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `rh` is the relative humidity percentage.
///
/// The return value is the Wet Bulb Globe Temperature in Kelvin.
///
/// Reference: ACSM (1984) [https://doi.org/10.1080/00913847.1984.11701899](https://doi.org/10.1080/00913847.1984.11701899)
///
/// See also: [http://www.bom.gov.au/info/thermal_stress/#approximation](http://www.bom.gov.au/info/thermal_stress/#approximation)
///
/// See also: [https://www.jstage.jst.go.jp/article/indhealth/50/4/50_MS1352/_pdf](https://www.jstage.jst.go.jp/article/indhealth/50/4/50_MS1352/_pdf)
pub fn calculate_wbgt_simple(t2_k: f64, rh: f64) -> f64 {
    let t2_c = kelvin_to_celsius(t2_k);
    let e = calculate_nonsaturation_vapour_pressure(t2_k, rh);
    let wbgt = 0.567 * t2_c + 0.393 * e + 3.94;
    let wbgt_k = celsius_to_kelvin(wbgt);

    return wbgt_k;
}

/// Calculates Wet Bulb Temperature.
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `rh` is the relative humidity percentage.
///
/// The return value is the wet bulb temperature in Kelvin.
///
/// Reference: Stull (2011) [https://doi.org/10.1175/JAMC-D-11-0143.1](https://doi.org/10.1175/JAMC-D-11-0143.1)
pub fn calculate_wbt(t2_k: f64, rh: f64) -> f64 {
    let t2_c = kelvin_to_celsius(t2_k);
    let tw = t2_c * (0.151977 * (rh + 8.313659).sqrt()).atan() + (t2_c + rh).atan()
        - (rh - 1.676331).atan()
        + 0.00391838 * (rh).powf(3.0 / 2.0) * (0.023101 * rh).atan()
        - 4.686035;
    celsius_to_kelvin(tw)
}

/// Calculates Globe Temperature.
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `mrt` is the mean radiant temperature in Kelvin.
///
/// Where `va` is the wind speed at 10 meters in m/s.
///
/// The return value is the globe temperature in Kelvin.
///
/// Reference: Guo et al. 2018 [https://doi.org/10.1016/j.enbuild.2018.08.029](https://doi.org/10.1016/j.enbuild.2018.08.029)
pub fn calculate_bgt(t2_k: f64, mrt: f64, va: f64) -> f64 {
    let v = scale_windspeed(va, 1.1); // formula requires wind speed at 1.1m (i.e., at the level of the globe)

    let d = (1.1e8 * v.powf(0.6)) / (0.95 * f64::from(0.15).powf(0.4));
    let e = -(mrt.powi(4)) - d * t2_k;

    let q: f64 = 12.0 * e;
    let s = 27.0 * (d.powi(2));
    let delta = ((s + (s.powi(2) - 4.0 * (q.powi(3))).sqrt()) / 2.0).powf(1.0 / 3.0);
    let q = 0.5 * ((1.0 / 3.0) * (delta + q / delta)).sqrt();

    let bgt = -q + 0.5 * (-4.0 * (q.powi(2)) + d / q).sqrt();

    return bgt;
}

/// Calculates Wet Bulb Globe Temperature (WBGT).
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `mrt` is the mean radiant temperature in Kelvin.
///
/// Where `va` is the wind speed at 10 meters in m/s.
///
/// Where `td_k` is the dew point temperature in Kelvin.
///
/// The return value is the wet bulb globe temperature in Kelvin.
///
/// Reference: Stull (2011) [https://doi.org/10.1175/JAMC-D-11-0143.1](https://doi.org/10.1175/JAMC-D-11-0143.1)
///
/// See also: [http://www.bom.gov.au/info/thermal_stress/](http://www.bom.gov.au/info/thermal_stress/)
pub fn calculate_wbgt(t2_k: f64, mrt: f64, va: f64, td_k: f64) -> f64 {
    let bgt_k = calculate_bgt(t2_k, mrt, va);
    let bgt_c = kelvin_to_celsius(bgt_k);

    let rh = calculate_relative_humidity_percent(t2_k, td_k);
    let t2_c = kelvin_to_celsius(t2_k);
    let tw_k = calculate_wbt(t2_k, rh);
    let tw_c = kelvin_to_celsius(tw_k);

    let wbgt = 0.7 * tw_c + 0.2 * bgt_c + 0.1 * t2_c;
    let wbgt_k = celsius_to_kelvin(wbgt);

    return wbgt_k;
}

/// Calculates Mean Radiant Temperature from Globe Temperature.
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `bgt_k` is the globe temperature in Kelvin.
///
/// Where `va` is the wind speed at 10 meters in m/s.
///
/// The return value is the mean radiant temperature in Kelvin.
///
/// Reference: Brimicombe et al. (2023) [https://doi.org/10.1029/2022GH000701](https://doi.org/10.1029/2022GH000701)
pub fn calculate_mrt_from_bgt(t2_k: f64, bgt_k: f64, va: f64) -> f64 {
    let v = scale_windspeed(va, 1.1); // formula requires wind speed at 1.1m (i.e., at the level of the globe)
    let f = (1.1e8 * v.powf(0.6)) / (0.95 * f64::from(0.15).powf(0.4));
    let bgt4 = bgt_k.powi(4);
    let mrtc = bgt4 + f * (bgt_k - t2_k);
    let mrtc2 = ((mrtc).sqrt()).sqrt();

    return mrtc2;
}

/// Calculates Humidex.
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `td_k` is the dew point temperature in Kelvin.
///
/// The return value is Humidex in Kelvin.
///
/// Reference: Blazejczyk et al. (2012) [https://doi.org/10.1007/s00484-011-0453-2](https://doi.org/10.1007/s00484-011-0453-2)
pub fn calculate_humidex(t2_k: f64, td_k: f64) -> f64 {
    let vp = 6.11 * f64::from(5417.7530 * ((1.0 / 273.16) - (1.0 / td_k))).exp(); // vapour pressure [hPa]
    let h = 0.5555 * (vp - 10.0);
    let humidex = t2_k + h;

    return humidex;
}

/// Calculates Normal Effective Temperature (NET).
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `va` is the wind speed at 10 meters in m/s.
///
/// Where `rh` is the relative humidity percentage.
///
/// The return value is the normal effective temperature in Kelvin.
///
/// Reference: Li and Chan (2006) [https://doi.org/10.1017/S1350482700001602](https://doi.org/10.1017/S1350482700001602)
pub fn calculate_normal_effective_temperature(t2_k: f64, va: f64, rh: f64) -> f64 {
    let t2_k = kelvin_to_celsius(t2_k);
    let v = scale_windspeed(va, 1.2); // formula requires wind speed at 1.2m
    let ditermeq = 1.0 / (1.76 + 1.4 * v.powf(0.75));
    let net =
        37.0 - ((37.0 - t2_k) / (0.68 - 0.0014 * rh + ditermeq)) - 0.29 * t2_k * (1.0 - 0.01 * rh);
    let net_k = celsius_to_kelvin(net);

    return net_k;
}

/// Calculates Apparent Temperature
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `va` is the wind speed at 10 meters in m/s.
///
/// Where `rh` is the relative humidity percentage.
///
/// The return value is the apparent temperature in Kelvin.
///
/// Reference: Steadman (1984) [https://doi.org/10.1175/1520-0450(1984)023%3C1674:AUSOAT%3E2.0.CO;2](https://doi.org/10.1175/1520-0450(1984)023%3C1674:AUSOAT%3E2.0.CO;2)
///
/// See also: [http://www.bom.gov.au/info/thermal_stress/#atapproximation](http://www.bom.gov.au/info/thermal_stress/#atapproximation)
pub fn calculate_apparent_temperature(t2_k: f64, va: f64, rh: f64) -> f64 {
    let t2_c = kelvin_to_celsius(t2_k);
    let e = calculate_nonsaturation_vapour_pressure(t2_k, rh);
    println!("{t2_k} {e} {rh}");
    let at = t2_c + 0.33 * e - 0.7 * va - 4.0;
    let at_k = celsius_to_kelvin(at);

    return at_k;
}

/// Calculates Wind Chill.
///
/// Where `t2_k` is the 2m Temperature in Kelvin.
///
/// Where `va` is the wind speed at 10 meters in m/s.
///
/// The return value is the wind chill in Kelvin.
///
/// Computation is only valid for temperatures between -50°C and 5°C and wind speeds between 5km/h and 80km/h.
/// For input values outside those ranges, computed results should not be considered valid.
///
/// Reference: Blazejczyk et al. (2012) [https://doi.org/10.1007/s00484-011-0453-2](https://doi.org/10.1007/s00484-011-0453-2)
///
/// See also: [https://web.archive.org/web/20130627223738/http://climate.weatheroffice.gc.ca/prods_servs/normals_documentation_e.html](https://web.archive.org/web/20130627223738/http://climate.weatheroffice.gc.ca/prods_servs/normals_documentation_e.html)
pub fn calculate_wind_chill(t2_k: f64, va: f64) -> f64 {
    let t2_c = kelvin_to_celsius(t2_k);
    let v = va * 3.6; // convert to kilometers per hour
    let windchill = 13.12 + 0.6215 * t2_c - 11.37 * v.powf(0.16) + 0.3965 * t2_c * v.powf(0.16);
    let windchill_k = celsius_to_kelvin(windchill);

    return windchill_k;
}

/// Calculates Heat Index using a simplified method.
///
/// Where `t2m` is the 2m temperature in Kelvin.
///
/// Where `rh` is the relative humidity percentage.
///
/// The return value is the heat index in Kelvin, or `None` if the temperature is too low.
///
/// Reference: Blazejczyk et al. (2012) [https://doi.org/10.1007/s00484-011-0453-2](https://doi.org/10.1007/s00484-011-0453-2)
pub fn calculate_heat_index_simplified(t2_k: f64, rh: f64) -> Option<f64> {
    let t2_c = kelvin_to_celsius(t2_k);

    let hiarray = [
        8.784695,
        1.61139411,
        2.338549,
        0.14611605,
        1.2308094e-2,
        1.6424828e-2,
        2.211732e-3,
        7.2546e-4,
        3.582e-6,
    ];

    if t2_c <= 20.0 {
        return None;
    }

    let hi = -hiarray[0] + hiarray[1] * t2_c + hiarray[2] * rh
        - hiarray[3] * t2_c * rh
        - hiarray[4] * t2_c.powi(2)
        - hiarray[5] * rh.powi(2)
        + hiarray[6] * t2_c.powi(2) * rh
        + hiarray[7] * t2_c * rh.powi(2)
        - hiarray[8] * t2_c.powi(2) * rh.powi(2);

    let hi_k = celsius_to_kelvin(hi);

    return Some(hi_k);
}

/// Calculates Heat Index with adjustments.
///
/// Where `t2_k` is the 2m temperature in Kelvin.
///
/// Where `td_k` is the 2m dewpoint temperature in Kelvin.
///
/// The return value is the heat index in Kelvin, or `None` if conditions are not met for calculation.
///
/// Reference: [https://www.wpc.ncep.noaa.gov/html/heatindex_equation.shtml](https://www.wpc.ncep.noaa.gov/html/heatindex_equation.shtml)
pub fn calculate_heat_index_adjusted(t2_k: f64, td_k: f64) -> Option<f64> {
    let rh = calculate_relative_humidity_percent(t2_k, td_k);
    let t2_f = kelvin_to_fahrenheit(t2_k);

    let hiarray = [
        42.379, 2.04901523, 10.1433312, 0.22475541, 0.00683783, 0.05481717, 0.00122874, 0.00085282,
        0.00000199,
    ];

    let hi_initial = 0.5 * (t2_f + 61.0 + ((t2_f - 68.0) * 1.2) + (rh * 0.094));

    let mut hi = -hiarray[0] + hiarray[1] * t2_f + hiarray[2] * rh
        - hiarray[3] * t2_f * rh
        - hiarray[4] * t2_f.powi(2)
        - hiarray[5] * rh.powi(2)
        + hiarray[6] * t2_f.powi(2) * rh
        + hiarray[7] * t2_f * rh.powi(2)
        - hiarray[8] * t2_f.powi(2) * rh.powi(2);

    if t2_f > 80.0 && t2_f < 112.0 && rh <= 13.0 {
        let adj = (13.0 - rh) / 4.0 * (17.0 - (t2_f - 95.0).abs() / 17.0).sqrt();
        hi = hi - adj;
    } else if t2_f > 80.0 && t2_f < 87.0 && rh > 85.0 {
        let adj = (rh - 85.0) / 10.0 * ((87.0 - t2_f) / 5.0);
        hi = hi + adj;
    } else if t2_f < 80.0 {
        let adj = 0.5 * (t2_f + 61.0 + ((t2_f - 68.0) * 1.2) + (rh * 0.094));
        hi = adj;
    } else if (hi_initial + t2_f / 2.0) < 80.0 {
        hi = 0.5 * (t2_f + 61.0 + ((t2_f - 68.0) * 1.2) + (rh * 0.094));
    } else {
        return None;
    }

    let hi_k = fahrenheit_to_kelvin(hi);

    return Some(hi_k);
}

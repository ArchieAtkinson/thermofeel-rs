#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use thermofeel_rs::*;

    #[test]
    fn test_relative_humidity_percent() {
        let t2_k = celsius_to_kelvin(30.0);
        let td_k = celsius_to_kelvin(28.0);
        let rhpc = calculate_relative_humidity_percent(t2_k, td_k);
        assert_relative_eq!(rhpc, 89.08526710467393, epsilon = 1e-6);
    }

    #[test]
    fn test_saturation_vapour_pressure() {
        let t2_k = celsius_to_kelvin(25.0);
        let svp = calculate_saturation_vapour_pressure(t2_k);
        assert_relative_eq!(svp, 31.699201897293, epsilon = 1e-6);
    }

    #[test]
    fn test_saturation_vapour_pressure_multiphase() {
        let t2_k = celsius_to_kelvin(-25.0);
        let es = calculate_saturation_vapour_pressure_multiphase(t2_k, Phase::Ice);
        assert_relative_eq!(es, 0.63142553, epsilon = 1e-6);
    }

    #[test]
    fn test_nonsaturation_vapour_pressure() {
        let t2_k = 300.0;
        let rh = 87.0;
        let svp = calculate_nonsaturation_vapour_pressure(t2_k, rh);
        assert_relative_eq!(svp, 30.649976725404283, epsilon = 1e-6);
    }

    #[test]
    fn test_scale_windspeed() {
        let va = 7.0;
        let h = 2.0;
        let vh = scale_windspeed(va, h);
        assert_relative_eq!(vh, 5.369069989882623, epsilon = 1e-6);
    }

    #[test]
    fn test_dew_point_from_relative_humidity() {
        let rh = 56.0;
        let t2_k = 304.15;
        let td_k = calculate_dew_point_from_relative_humidity(rh, t2_k);
        assert_relative_eq!(td_k, 294.3484414118635, epsilon = 1e-6);
    }

    #[test]
    fn test_mean_radiant_temperature() {
        let ssrd = 60000.0 / 3600.0;
        let ssr = 471818.0 / 3600.0;
        let fdir = 374150.0 / 3600.0;
        let strd = 1061213.0 / 3600.0;
        let strr = -182697.0 / 3600.0;
        let cossza = 0.4;
        let dsrp = approximate_dsrp(fdir, cossza).unwrap();
        let mrt =
            calculate_mean_radiant_temperature(ssrd, ssr, dsrp, strd, fdir, strr, cossza / 3600.0);
        assert_relative_eq!(mrt, 270.85099123, epsilon = 1e-6);
    }

    #[test]
    fn test_utci() {
        // case 1
        let t2_k_1 = 309.0;
        let va_1 = 3.0;
        let mrt_1 = 310.0;
        let e_hpa_1 = 12.0;
        let utci_1 = calculate_utci(t2_k_1, va_1, mrt_1, None, Some(e_hpa_1));
        assert_relative_eq!(utci_1, 307.76473586, epsilon = 1e-5);

        // case 2
        let t2_k_2 = celsius_to_kelvin(27.0);
        let va_2 = 4.0;
        let mrt_2 = celsius_to_kelvin(9.2);
        let e_hpa_2 = 16.5;
        let utci_2 = calculate_utci(t2_k_2, va_2, mrt_2, None, Some(e_hpa_2));
        assert_relative_eq!(kelvin_to_celsius(utci_2), 18.93148565062157, epsilon = 1e-5);
    }

    #[test]
    fn test_wbgt_simple() {
        let t2_k = celsius_to_kelvin(30.0);
        let rh = 80.0;
        let wbgts = calculate_wbgt_simple(t2_k, rh);
        assert_relative_eq!(wbgts, 307.39508355517813, epsilon = 1e-6);
    }

    #[test]
    fn test_wbt() {
        let t2_k = celsius_to_kelvin(20.0);
        let rh = 50.0;
        let wbt = calculate_wbt(t2_k, rh);
        assert_relative_eq!(wbt, 286.84934189999996, epsilon = 1e-6);
    }

    #[test]
    fn test_bgt() {
        let t2_k = [278.15, 300.0, 300.0];
        let va = [20.0, 20.0, -10.0]; // negative va values are treated as 0
        let mrt = [278.15, 310.0, 310.0];

        let mut bgt = Vec::new();

        for i in 0..3 {
            bgt.push(calculate_bgt(t2_k[i], va[i], mrt[i]));
        }

        // Assuming calculate_bgt can handle slices/vectors and returns a vector

        assert_relative_eq!(bgt[0], 277.1238737724192, epsilon = 1e-6);
        assert_relative_eq!(bgt[1], 298.70218703427656, epsilon = 1e-6);
        assert_relative_eq!(bgt[2], 298.70216299754475, epsilon = 1e-6);
    }

    #[test]
    fn test_wbgt() {
        let t2_k = 300.0;
        let td_k = 290.0;
        let va = 20.0;
        let mrt = 310.0;
        let wbgt = calculate_wbgt(t2_k, mrt, va, td_k);
        assert_relative_eq!(wbgt, 295.5769818634555, epsilon = 1e-6);
    }

    #[test]
    fn test_mrt_from_bgt() {
        let t2_k = celsius_to_kelvin(25.0);
        let bgt_k = celsius_to_kelvin(23.0);
        let va = 10.0;
        let mrt_c = calculate_mrt_from_bgt(t2_k, bgt_k, va);
        assert_relative_eq!(mrt_c, 279.80189775556704, epsilon = 1e-6);
    }

    #[test]
    fn test_humidex() {
        let t2_k = 304.0;
        let td_k = 300.0;
        let hu = calculate_humidex(t2_k, td_k);
        assert_relative_eq!(hu, 318.4601286141123, epsilon = 1e-6);
    }

    #[test]
    fn test_normal_effective_temperature() {
        let t2_k = 307.0;
        let va = 4.0;
        let rh = 80.0;
        let net = calculate_normal_effective_temperature(t2_k, va, rh);
        assert_relative_eq!(net, 304.13650125, epsilon = 1e-6);
    }

    #[test]
    fn test_apparent_temperature() {
        let t2_k = celsius_to_kelvin(25.0);
        let va = 3.0;
        let rh = 75.0;
        let at = calculate_apparent_temperature(t2_k, va, rh);
        assert_relative_eq!(at, 299.86678322384626, epsilon = 1e-6);
    }

    #[test]
    fn test_wind_chill() {
        let t2_k = 270.0;
        let va = 10.0;
        let wc_k = calculate_wind_chill(t2_k, va);
        assert_relative_eq!(wc_k, 261.92338925380074, epsilon = 1e-6);

        // reference result from wikipedia article https://en.wikipedia.org/wiki/Wind_chill
        let t2_k_wiki_1 = celsius_to_kelvin(-20.0); // -20C to K
        let va_wiki_1 = 5.0 / 3.6; // 5 km/h to m/s
        let wc_k_wiki_1 = calculate_wind_chill(t2_k_wiki_1, va_wiki_1);
        let wc_c_wiki_1 = kelvin_to_celsius(wc_k_wiki_1);
        assert_relative_eq!(wc_c_wiki_1, -24.27850328, epsilon = 1e-6);

        let va_wiki_2 = 30.0 / 3.6; // 30 km/h to m/s
        let wc_k_wiki_2 = calculate_wind_chill(t2_k_wiki_1, va_wiki_2);
        let wc_c_wiki_2 = kelvin_to_celsius(wc_k_wiki_2);
        assert_relative_eq!(wc_c_wiki_2, -32.56804448, epsilon = 1e-6);
    }

    #[test]
    fn test_heat_index_simplified() {
        let t2_k = celsius_to_kelvin(21.0);
        let rh = 80.0;
        let hi = calculate_heat_index_simplified(t2_k, rh).unwrap();
        assert_relative_eq!(hi, 294.68866082, epsilon = 1e-6);
    }

    #[test]
    fn test_heat_index_adjusted() {
        let t2_k = 295.0;
        let td_k = 290.0;
        let hia = calculate_heat_index_adjusted(t2_k, td_k).unwrap();
        assert_relative_eq!(hia, 295.15355699, epsilon = 1e-6);
    }
}

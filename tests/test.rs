use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use csv::ReaderBuilder;
use serde::Deserialize;

use thermofeel_rs::*;

#[derive(Debug, Deserialize)]
struct TestCaseRow {
    t2m: f64,
    ssr: f64,
    td: f64,
    va: f64,
    mrt: f64,
    ssrd: f64,
    strd: f64,
    fdir: f64,
    strr: f64,
    cossza: f64,
    phase: usize,
    va_height: f64,
}

fn load_single_column_csv(filename: &str) -> io::Result<Vec<f64>> {
    let path = Path::new("tests").join("data").join(filename);

    eprintln!("{:?}", path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(reader);
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if let Some(value_str) = record.get(0) {
            records.push(
                value_str
                    .parse::<f64>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            );
        }
    }
    Ok(records)
}

fn load_test_cases_csv(filename: &str) -> io::Result<Vec<TestCaseRow>> {
    let path = Path::new("tests").join("data").join(filename);

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(reader);
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        records.push(result?);
    }
    Ok(records)
}

fn assert_almost_equal(
    expected: f64,
    actual: f64,
    decimal_places: u32,
    test_name: &str,
    index: usize,
) {
    let epsilon = 10.0_f64.powi(-(decimal_places as i32));
    if (expected - actual).abs() > epsilon {
        panic!(
            "Assertion failed for {}: Mismatch at index {}. Expected {}, got {}. Difference: {}",
            test_name,
            index,
            expected,
            actual,
            (expected - actual).abs()
        );
    }
}

#[test]
fn test_saturation_vapour_pressure() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_es = load_single_column_csv("es.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let es = calculate_saturation_vapour_pressure(case.t2m);
        assert_almost_equal(expected_es[i], es, 6, "test_saturation_vapour_pressure", i);
    }
    Ok(())
}

#[test]
fn test_saturation_vapour_pressure_multiphase() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_es_multiphase = load_single_column_csv("es_multiphase.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let phase = if case.phase == 0 {
            Phase::Liquid
        } else {
            Phase::Ice
        };
        let es_multiphase = calculate_saturation_vapour_pressure_multiphase(case.t2m, phase);
        assert_almost_equal(
            expected_es_multiphase[i],
            es_multiphase,
            6,
            "test_saturation_vapour_pressure_multiphase",
            i,
        );
    }
    Ok(())
}

#[test]
fn test_nonsaturation_vapour_pressure() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_ens = load_single_column_csv("ens.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let rh = calculate_relative_humidity_percent(case.t2m, case.td);
        let ens = calculate_nonsaturation_vapour_pressure(case.t2m, rh);
        assert_almost_equal(
            expected_ens[i],
            ens,
            6,
            "test_nonsaturation_vapour_pressure",
            i,
        );
    }
    Ok(())
}

#[test]
fn test_scale_windspeed() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_va_scaled = load_single_column_csv("va_scaled.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let va_scaled = scale_windspeed(case.va, case.va_height);
        assert_almost_equal(
            expected_va_scaled[i],
            va_scaled,
            6,
            "test_scale_windspeed",
            i,
        );
    }
    Ok(())
}

#[test]
fn test_mean_radiant_temperature() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_mrtr = load_single_column_csv("mrtr.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        if let Some(dsrp) = approximate_dsrp(case.fdir / 3600.0, case.cossza / 3600.0) {
            let mrtr = calculate_mean_radiant_temperature(
                case.ssrd / 3600.0,
                case.ssr / 3600.0,
                dsrp,
                case.strd / 3600.0,
                case.fdir / 3600.0,
                case.strr / 3600.0,
                case.cossza / 3600.0,
            );
            assert_almost_equal(
                expected_mrtr[i],
                mrtr,
                6,
                "test_mean_radiant_temperature",
                i,
            );
        }
    }
    Ok(())
}

#[test]
fn test_utci() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_utci = load_single_column_csv("utci.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let rh_pc = calculate_relative_humidity_percent(case.t2m, case.td);
        let ehpa = calculate_saturation_vapour_pressure(case.t2m) * rh_pc / 100.0;
        let utci = calculate_utci(case.t2m, case.va, case.mrt, None, Some(ehpa));
        assert_almost_equal(expected_utci[i], utci, 6, "test_utci", i);
    }
    Ok(())
}

#[test]
fn test_wbgt_simple() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_wbgts = load_single_column_csv("wbgts.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let rh_pc = calculate_relative_humidity_percent(case.t2m, case.td);
        let wbgts = calculate_wbgt_simple(case.t2m, rh_pc);
        assert_almost_equal(expected_wbgts[i], wbgts, 6, "test_wbgt_simple", i);
    }
    Ok(())
}

#[test]
fn test_wbt() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_wbt = load_single_column_csv("wbt.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let rh_pc = calculate_relative_humidity_percent(case.t2m, case.td);
        let wbt = calculate_wbt(case.t2m, rh_pc);
        assert_almost_equal(expected_wbt[i], wbt, 6, "test_wbt", i);
    }
    Ok(())
}

#[test]
fn test_bgt() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_bgt = load_single_column_csv("bgt.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let bgt = calculate_bgt(case.t2m, case.mrt, case.va);
        assert_almost_equal(expected_bgt[i], bgt, 6, "test_bgt", i);
    }
    Ok(())
}

#[test]
fn test_wbgt() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_wbgt = load_single_column_csv("wbgt.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let wbgt = calculate_wbgt(case.t2m, case.mrt, case.va, case.td);
        assert_almost_equal(expected_wbgt[i], wbgt, 6, "test_wbgt", i);
    }
    Ok(())
}

#[test]
fn test_mrt_from_bgt() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_mrt_from_bgt = load_single_column_csv("mrt_from_bgt.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        // Recalculate bgt as it's an input to mrt_from_bgt
        let bgt = calculate_bgt(case.t2m, case.mrt, case.va);
        let mrt_from_bgt = calculate_mrt_from_bgt(case.t2m, bgt, case.va);
        assert_almost_equal(
            expected_mrt_from_bgt[i],
            mrt_from_bgt,
            6,
            "test_mrt_from_bgt",
            i,
        );
    }
    Ok(())
}

#[test]
fn test_humidex() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_humidex = load_single_column_csv("humidex.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let humidex = calculate_humidex(case.t2m, case.td);
        assert_almost_equal(expected_humidex[i], humidex, 6, "test_humidex", i);
    }
    Ok(())
}

#[test]
fn test_normal_effective_temperature() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_net = load_single_column_csv("net.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let rh_pc = calculate_relative_humidity_percent(case.t2m, case.td);
        let net = calculate_normal_effective_temperature(case.t2m, case.va, rh_pc);
        assert_almost_equal(
            expected_net[i],
            net,
            6,
            "test_normal_effective_temperature",
            i,
        );
    }
    Ok(())
}

#[test]
fn test_apparent_temperature() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_at = load_single_column_csv("at.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let rh_pc = calculate_relative_humidity_percent(case.t2m, case.td);
        let at = calculate_apparent_temperature(case.t2m, case.va, rh_pc);
        assert_almost_equal(expected_at[i], at, 6, "test_apparent_temperature", i);
    }
    Ok(())
}

#[test]
fn test_wind_chill() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_windchill = load_single_column_csv("windchill.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let windchill = calculate_wind_chill(case.t2m, case.va);
        assert_almost_equal(expected_windchill[i], windchill, 6, "test_wind_chill", i);
    }
    Ok(())
}

#[test]
fn test_heat_index() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_heatindex = load_single_column_csv("heatindex.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let rh_pc = calculate_relative_humidity_percent(case.t2m, case.td);
        if let Some(heatindex) = calculate_heat_index_simplified(case.t2m, rh_pc) {
            assert_almost_equal(expected_heatindex[i], heatindex, 6, "test_heat_index", i);
        }
    }
    Ok(())
}

#[test]
fn test_heat_index_adjusted() -> io::Result<()> {
    let test_cases = load_test_cases_csv("thermofeel_testcases.csv")?;
    let expected_hia = load_single_column_csv("hia.csv")?;

    for (i, case) in test_cases.iter().enumerate() {
        let hia = calculate_heat_index_adjusted(case.t2m, case.td);
        if let Some(hia) = hia {
            assert_almost_equal(expected_hia[i], hia, 6, "test_heat_index_adjusted", i);
        }
    }
    Ok(())
}

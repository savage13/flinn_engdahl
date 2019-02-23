
//! Implementation of the Flinn-Engdahl seismic and geographical regionalization scheme.
//!
//! Converts a location (latitude, longitude) into a named region of the world
//!
//!
//! ```rust
//! use flinn_engdahl as fe;
//! let name = fe::region(-42.448299, 171.214005).unwrap();
//! assert_eq!(name, "SOUTH ISLAND, NEW ZEALAND");
//! ```
//!
//! # References
//!
//!    - [Young, J.B., Presgrave, B.W., Aichele, H., Wiens, D.A. and Flinn, E.A., 1996, The Flinn-Engdahl Regionalisation Scheme: the 1995 revision, Physics of the Earth and Planetary Interiors, v. 96, p. 223-297.](https://www.sciencedirect.com/science/article/pii/003192019603141X)
//!    - [Flinn, E.A., Engdahl, E.R. and Hill, A.R., 1974, Seismic and geographical regionalization, Bulletin of the Seismological Society of America, vol. 64, p. 771-993.](https://pubs.geoscienceworld.org/ssa/bssa/article/64/3-2/771/117264/seismic-and-geographical-regionalization)
//!    - [Flinn, E.A., and Engdahl, E.R., 1965, A proposed basis for geographical and seismic regionalization, Reviews of Geophysics, vol. 3, p. 123-149.](https://doi.org/10.1029/RG003i001p00123)
//!
//! # Further Information
//! [https://en.wikipedia.org/wiki/Flinn%E2%80%93Engdahl_regions](https://en.wikipedia.org/wiki/Flinn%E2%80%93Engdahl_regions)
//!
//! [https://earthquake.usgs.gov/learn/topics/flinn_engdahl.php](https://earthquake.usgs.gov/learn/topics/flinn_engdahl.php)
//!
//! [ftp://hazards.cr.usgs.gov/feregion/fe_1995/](ftp://hazards.cr.usgs.gov/feregion/fe_1995/)
//! 

/// Errors for Flinn_Engdahl
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum RegionError {
    /// Longitude is out of allowable range
    BadLongitude,
    /// Latitude is out of allowable range
    BadLatitude,
}

/// Convert lat,lon position in region number
///
/// # Arguments
///  - lat - Latitude [-90, 90]
///  - lon - Longitude [-360, 360]
///  - quadid - Quadrant Index ([274, 183, 92, 1])
///  - llindx - (Tier onset index, length of segments in tier)
///  - lattiers - (Longitude (truncated), Region Number)
///
fn namnum(lat: f64, lon: f64,
          quadid: &[usize],
          llindx: &[(usize,usize)],
          lattiers: &[(usize,usize)]) -> Result<usize,RegionError> {
    if lat.abs() > 90.0 {
        return Err(RegionError::BadLatitude);
    }
    if lon.abs() > 360.0 {
        return Err(RegionError::BadLongitude);
    }

    let mut lon = lon;
    if lon < -180.0 {
        lon = lon + 360.0;
    } else if lon > 180.0 {
        lon = lon - 360.0;
    }

    // Find the Hemisphere of the input position
    let quadon = match (lat >= 0.0, lon >= 0.0) {
        (true,  true)  => quadid[3], // South West
        (true,  false) => quadid[2], // South East
        (false, true)  => quadid[1], // North West
        (false, false) => quadid[0], // North East
    };

    // Truncate the latitude and longitude
    let lt = lat.abs().trunc() as usize;
    let ln = lon.abs().trunc() as usize;

    // Get first index of latitudes in the correct quadrant
    let recnbr = quadon + lt - 1;
    // tieron - Tier Onset
    // nbrbdy - Number of segments in tier
    let (tieron, nbrbdy) = llindx[ recnbr ];
    // Search through the seguments for longitude
    let mut pair = lattiers[tieron-1];
    for i in tieron - 1 .. tieron - 1 + nbrbdy {
        pair = lattiers[i];
        if lattiers[i].0 > ln {
            pair = lattiers[i-1];
            break;
        }
    }
    Ok(pair.1)
}

/// Get the Flinn_Engdahl region name from a location at (`lat`,`lon`)
///
/// ```rust
///  use flinn_engdahl as fe;
///  let region = fe::region(41.440971, -71.502289).unwrap();
///  assert_eq!(region, "SOUTHERN NEW ENGLAND");
/// ```
///
/// # Arguments
///   - lat - Latitude  [-90, 90]
///   - lon - Longitude [-360, 360]
///
///   Note: Values returned at longitude of -180 and 180 are different
///
/// # Returns
///   - Flinn_Engdahl Region Name
///
///
pub fn region(lat: f64, lon: f64) -> Result<&'static str, RegionError> {
    let n = crate::namnum(lat, lon, &crate::quadids(), &LLINDX, &LAT_TIERS)?;
    Ok(NAMES[n-1])
}
/// Get the Flinn_Engdahl region numner from a location at (`lat`,`lon`)
///
/// ```rust
///  use flinn_engdahl as fe;
///  let region = fe::region(37.871593, -122.272743).unwrap();
///  assert_eq!(region, "CENTRAL CALIFORNIA");
/// ```
///
/// # Arguments
///   - lat - Latitude
///   - lon - Longitude
///
/// # Returns
///   - Flinn_Engdahl Region Number
///
///
pub fn region_number(lat: f64, lon: f64) -> Result<usize, RegionError> {
    crate::namnum(lat, lon, &crate::quadids(), &LLINDX, &LAT_TIERS)
}


const fn quadids() -> [usize; 4] {
    [274, 183, 92, 1]
}

/// Read in data from quadsidx.asc
///
/// Output is the "tier" and number of segments
///
/// The tier represent the offset into the latitude tiers (lattiers) array
/// the number of segmens is the number of segments in that latitude tier
///
/// quadsidx.asc include the number of segments in each tier, organized by quadrant.
/// For the 1995 version, there should be 364 total tiers, 91 per quadrant
/// Each tier represents a latitude, starting at 0 and working towards the pole by 1 degree
/// [0..90] inclusive range
///
fn llindx() -> Vec<(usize,usize)> {
    let base = std::path::Path::new("data");
    let data = std::fs::read_to_string(base.join("quadsidx.asc")).unwrap();
    let mut carry = 1;
    let mut out = vec![];
    for item in data.split_whitespace() {
        let v = item.parse().unwrap();
        out.push( (carry, v) );
        carry += v;
    }
    out
}

fn lat_lon_index_write<P: AsRef<std::path::Path>>(llindx: &[(usize,usize)], file: P) {
    let mut out = String::new();
    let n = llindx.len();
    out += &format!("const LLINDX: [(usize,usize); {}] = [\n", n);
    for (a,b) in llindx {
        out += &format!(" ({}, {}),\n", a,b);
    }
    out += &format!("];\n");
    std::fs::write(file, out).unwrap();
}

/// Read in data from the quadrant section as Latitude Tiers
///
/// Data within the file is organized as (longitude, region number) pairs
/// Within each Tier, the longitude is always increasing
///
fn latitude_tiers_read() -> Vec<(usize,usize)> {
    let base = std::path::Path::new("data");
    let sects = ["nesect.asc", "nwsect.asc", "sesect.asc", "swsect.asc" ];
    let mut out = vec![];
    for sect in sects.iter() {
        let file = base.join(sect);
        let data = std::fs::read_to_string(file).unwrap();
        let mut items = data.split_whitespace();
        while let (Some(a),Some(b)) = (items.next(), items.next()) {
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            out.push( (a,b) )
        }
    }
    out
}

fn latitude_tiers_write<P: AsRef<std::path::Path>>(lattiers: &[(usize,usize)], file: P) {
    let mut out = String::new();
    let n = lattiers.len();
    out += &format!("const LAT_TIERS: [(usize,usize); {}] = [\n", n);
    for (a,b) in lattiers {
        out += &format!( " ({}, {}),\n", a,b);
    }
    out += &format!("];");
    std::fs::write(file, out).unwrap();
}

include!("names.rs");
include!("latitude_tiers.rs");
include!("lat_lon_index.rs");

fn names_read() -> Vec<String> {
    let base = std::path::Path::new("data");
    let data = std::fs::read_to_string(base.join("names.asc")).unwrap();
    data.lines().map(|x| x.to_string()).collect()
}

fn names_write<P: AsRef<std::path::Path>>(names: &[String], file: P) {
    let mut out = String::new();
    let n = names.len();
    out += &format!("const NAMES: [&str; {}] = [\n", n);
    for n in names {
        out += &format!(" \"{}\",\n", n);
    }
    out += &format!("];");
    std::fs::write(file, out).unwrap();
}

#[allow(dead_code)]
fn reformat_names<P: AsRef<std::path::Path>>(file: P) {
    names_write( &names_read(), file );
}
#[allow(dead_code)]
fn reformat_latitude_tiers<P: AsRef<std::path::Path>>(file: P) {
    latitude_tiers_write( &latitude_tiers_read(), file );
}
#[allow(dead_code)]
fn reformat_lat_lon_index<P: AsRef<std::path::Path>>(file: P) {
    lat_lon_index_write( &llindx(), file );
}


#[cfg(test)]
mod tests {
    #[test]
    fn check_simple() {
        let names = crate::names_read();
        assert_eq!(names.len(), 757);
        let n = crate::namnum(-90.0, -90.0,
                              &crate::quadids(),
                              &crate::LLINDX,
                              &crate::LAT_TIERS).unwrap();
        assert_eq!(names[n-1], "ANTARCTICA");
        assert_eq!(crate::NAMES[n-1], "ANTARCTICA");
        assert_eq!(n, 729);
    }
    #[test]
    fn check_region() {

        let (lat,lon,rid,name) = (-79.50, 0.00, 729, "ANTARCTICA");
        let n = crate::region_number(lat, lon).unwrap();
        let place = crate::region(lat, lon).unwrap();
        assert_eq!(name, place, "{} {}", place, n);
        assert_eq!(rid, n, "{} {}", place, n);

        let (lat,lon,rid,name) = (-78.50, 155.00, 727, "VICTORIA LAND, ANTARCTICA");
        let n = crate::region_number(lat, lon).unwrap();
        let place = crate::region(lat, lon).unwrap();
        assert_eq!(name, place, "{} {}", place, n);
        assert_eq!(rid, n, "{} {}", place, n);

        let name = crate::region(-77.845753, 166.675927).unwrap();
        assert_eq!(name, "VICTORIA LAND, ANTARCTICA");

        
    }

    #[test]
    fn check_llindx() {
        //let index = &crate::llindx();
        let index = &crate::LLINDX;
        assert_eq!(index.len(), 364);
        assert_eq!(index[0], (1,24));
    }
    #[test]
    fn check_lat_tiers() {
        //let index = &crate::latitude_tiers_read();
        let index = &crate::LAT_TIERS;
        assert_eq!(index.len(), 5958);
        assert_eq!(index[0], (0,561));
    }
    #[test]
    fn reformat_files() {
        // crate::reformat_names("src/names.rs");
        // crate::reformat_latitude_tiers("src/latitude_tiers.rs");
        // crate::reformat_lat_lon_index("src/lat_lon_index.rs");
    }
    #[test]
    fn full_comparison() {

        let data = std::fs::read_to_string("data/fe-short.txt").unwrap();
        for line in data.lines() {
            if line.trim().len() == 0 {
                continue;
            }
            let mut items = line.split_whitespace();
            let lat : f64 = items.next().unwrap().parse().unwrap();
            let lon : f64 = items.next().unwrap().parse().unwrap();
            let rid : usize = items.next().unwrap().parse().unwrap();
            let name : &str = &items.collect::<Vec<_>>().join(" ");
            let n = crate::region_number(lat, lon).unwrap();
            let place = crate::region(lat, lon).unwrap();
            assert_eq!(name, place, "{} {} {}", line, place, n);
            assert_eq!(rid, n, "{} {} {}", line, place, n);
        }
    }
    #[test]
    fn lat_test() {
        assert_eq!(crate::region_number(0., 0.), Ok(561));
        assert_eq!(crate::region_number(-91., 0.), Err(crate::RegionError::BadLatitude));
        assert_eq!(crate::region_number(91., 0.), Err(crate::RegionError::BadLatitude));
        assert_eq!(crate::region_number(90., 0.), Ok(633));
        assert_eq!(crate::region_number(-90., 0.), Ok(729));
    }
    #[test]
    fn lon_test() {
        assert_eq!(crate::region_number(0., 361.), Err(crate::RegionError::BadLongitude));
        assert_eq!(crate::region_number(0., -361.), Err(crate::RegionError::BadLongitude));
        assert_eq!(crate::region_number(0., 360.), Ok(561));
        assert_eq!(crate::region_number(0., -360.), Ok(561));
        assert_eq!(crate::region_number(0., 0.), crate::region_number(0.0,  360.0));
        assert_eq!(crate::region_number(0., 0.), crate::region_number(0.0, -360.0));
        assert_eq!(crate::region_number(0., 179.9), crate::region_number(0.0, -180.1));
        assert_eq!(crate::region_number(0., 180.1), crate::region_number(0.0, -179.9));
        assert_ne!(crate::region_number(0., 180.0), crate::region_number(0.0, -180.0));
    }
}

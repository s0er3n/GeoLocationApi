#![feature(proc_macro_hygiene, decl_macro)]

use geo::prelude::BoundingRect;
use geo::{prelude::Contains, Geometry, Point, Rect};
use geojson::GeoJson;
use lazy_static::lazy_static;
// use serde::{Deserialize, Serialize};

use rocket::{get, routes};
use std::fs;

lazy_static! {

// Turn the GeoJSON string into a geo_types GeometryCollection
    static ref ISO_COUNTRY_GEOMETRY: Vec<(String, Geometry<f64>, Rect<f64>)> = load_borders_from_git();
}

#[get("/getIsoCodes?<lng>&<lat>")]
fn index(lng: f64, lat: f64) -> std::string::String {
    get_country(lng, lat)
}

fn main() {
    // get_country(52., 52., &COLLECTION, &FEATURES);
    // get_country(52., 13., &COLLECTION, &FEATURES);

    rocket::ignite().mount("/", routes![index]).launch();

    // println!("{:?}", country_name_geometry)
}

fn load_borders_from_git() -> Vec<(String, Geometry<f64>, Rect<f64>)> {
    let mut country_name_geometry = Vec::new();
    for country_file in fs::read_dir("./rhinoBorderGit/BorderData").unwrap() {
        // print!("{:?}", country_file);
        match country_file {
            Ok(name) => {
                let geojson = fs::read_to_string(name.path())
                    .unwrap_or_default()
                    .parse::<GeoJson>();

                match geojson {
                    Ok(geojson_ok) => {
                        if let GeoJson::FeatureCollection(feat) = geojson_ok {
                            for f in feat.features.iter() {
                                let geometry =
                                    geo::Geometry::try_from(f.to_owned().geometry.unwrap())
                                        .unwrap();
                                let bbox = geometry.bounding_rect().unwrap();

                                country_name_geometry.push((
                                    serde_json::to_string(&f.properties.clone().unwrap()).unwrap(),
                                    geometry,
                                    bbox,
                                ));
                            }
                        }
                    }
                    _ => println!("unreadable"),
                }
            }
            _ => println!("fail"),
        }
    }

    country_name_geometry
}

fn get_country(lat: f64, lng: f64) -> String {
    let point: Point<f64> = (lng, lat).into();

    let mut res: Vec<String> = Vec::new();

    for (properties, geometry, bbox) in ISO_COUNTRY_GEOMETRY.iter() {
        if !bbox.contains(&point) {
            continue;
        }
        if geometry.contains(&point) {
            res.push(properties.clone());
        }
    }
    return res.join(",");
}

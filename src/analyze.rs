use std::collections::BTreeMap;
use geojson::{Feature, GeoJson, Geometry, Value};
use priority_queue::PriorityQueue;
use crate::quickhull::quick_hull;

pub fn analyze(geojson: GeoJson, propertyToFilter: &str, filter: &str) {
    let mut areas = BTreeMap::new();
    let mut qh_areas = BTreeMap::new();

    if let GeoJson::FeatureCollection(collection) = geojson {
        
        for feature in collection.features {
            if let Some(properties) = feature.properties {
                if !properties.contains_key(propertyToFilter) {
                    continue;
                }
                if properties[propertyToFilter] != filter {
                    continue;
                }
                // println!("Properties: {:?}", properties);
                println!("Analyzing district: {:?}", properties["OFFICE_ID"]);
            
                if let Some(geometry) = feature.geometry {
                    // println!("Area: {:?}", area(&geometry));
                    match geometry.value {
                        Value::Polygon(ref polygon) => {
                            let points: Vec<[f64; 2]> = polygon[0].iter()
                                .filter_map(|coords| {
                                    if coords.len() >= 2 {
                                        Some([coords[0], coords[1]])
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            areas.insert(properties["OFFICE_ID"].to_string(), area(&points));
                            qh_areas.insert(properties["OFFICE_ID"].to_string(), area(&quick_hull(&points)));
                        }
                        _ => todo!()
                    }
                }
            }
        }
    }

    // Print the areas
    let mut failedDistricts = PriorityQueue::new();
    let mut passedDistricts = PriorityQueue::new();

    for (district, area) in areas {
        println!("District: {}, Area: {}", district, area);
        println!("District: {}, QH_Area: {}", district, qh_areas[&district]);
        if (qh_areas[&district] / area) > 1.4 {
            failedDistricts.push(district.clone(), std::cmp::Reverse(district.clone()));
        } else {
            passedDistricts.push(district.clone(), std::cmp::Reverse(district.clone()));
        }
    }

    println!("Failed districts: {:?}", failedDistricts.iter()
            .map(|(district, _)| district.as_str())
            .collect::<Vec<_>>()
            .join(", "));
    println!("Passed districts: {:?}", passedDistricts.iter()
            .map(|(district, _)| district.as_str())
            .collect::<Vec<_>>()
            .join(", "));
}

fn area(polygon: &Vec<[f64; 2]>) -> f64 {
    // println!("coords: {:?}", polygon);
    let points = &polygon;
    let n = points.len();
    let mut area = 0.0;
    for i in 0..n {
        let prev = if i == 0 { n - 1 } else { i - 1 };
        let next = if i == n - 1 { 0 } else { i + 1 };
        
        area += points[i][0] * (points[next][1] - points[prev][1]);
    }
    
    area /= 2.0;
    area.abs()
}

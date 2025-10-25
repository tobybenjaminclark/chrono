use crate::types::{Map, Place};
use serde::Deserialize;
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct GeocodeResponse {
    results: Vec<GeocodeResult>,
}

#[derive(Debug, Deserialize)]
struct GeocodeResult {
    geometry: Geometry,
    formatted_address: String,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    location: LatLng,
}

#[derive(Debug, Deserialize)]
struct LatLng {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Deserialize)]
struct PlacesResponse {
    results: Vec<PlaceResult>,
}

#[derive(Debug, Deserialize)]
struct PlaceResult {
    name: String,
    geometry: Geometry,
}

#[derive(Debug, Deserialize)]
struct DirectionsResponse {
    routes: Vec<DirectionsRoute>,
}

#[derive(Debug, Deserialize)]
struct DirectionsRoute {
    overview_polyline: Polyline,
}

#[derive(Debug, Deserialize)]
struct Polyline {
    points: String,
}

// Decode Google’s encoded polyline format → Vec<(lat, lng)>
fn decode_polyline(encoded: &str) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let mut index = 0;
    let mut lat = 0i64;
    let mut lng = 0i64;

    while index < encoded.len() {
        let mut b;
        let mut shift = 0;
        let mut result = 0;
        loop {
            b = encoded.as_bytes()[index] as i64 - 63;
            index += 1;
            result |= (b & 0x1F) << shift;
            shift += 5;
            if b < 0x20 {
                break;
            }
        }
        let dlat = if (result & 1) != 0 { !(result >> 1) } else { result >> 1 };
        lat += dlat;

        shift = 0;
        result = 0;
        loop {
            b = encoded.as_bytes()[index] as i64 - 63;
            index += 1;
            result |= (b & 0x1F) << shift;
            shift += 5;
            if b < 0x20 {
                break;
            }
        }
        let dlng = if (result & 1) != 0 { !(result >> 1) } else { result >> 1 };
        lng += dlng;

        points.push((lat as f64 / 1e5, lng as f64 / 1e5));
    }

    points
}

fn haversine_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    // Approximate distance between two lat/lng points in meters
    let r = 6371000.0; // Earth radius in meters
    let lat1 = a.0.to_radians();
    let lat2 = b.0.to_radians();
    let dlat = (b.0 - a.0).to_radians();
    let dlng = (b.1 - a.1).to_radians();

    let hav = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlng / 2.0).sin().powi(2);
    2.0 * r * hav.sqrt().asin()
}

pub async fn fetch_map(
    place: &str,
    n: usize,
    min_distance_m: f64,
) -> Result<Map, Box<dyn std::error::Error>> {
    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let client = Client::new();

    // Step 1: Geocode starting place
    let geo_url = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
        urlencoding::encode(place),
        api_key
    );
    let geo_res: GeocodeResponse = client.get(&geo_url).send().await?.json().await?;
    let first_result = geo_res
        .results
        .get(0)
        .ok_or("No results found for that place")?;
    let center = (
        first_result.geometry.location.lat,
        first_result.geometry.location.lng,
    );

    // Step 2: Find nearby attractions
    let places_url = format!(
        "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius=1609&type=tourist_attraction&key={}",
        center.0, center.1, api_key
    );
    let places_res: PlacesResponse = client.get(&places_url).send().await?.json().await?;

    let mut locations = Vec::new();
    for p in places_res.results.into_iter() {
        let candidate = Place {
            name: p.name,
            location: (p.geometry.location.lat, p.geometry.location.lng),
        };

        if locations
            .iter()
            .all(|existing: &Place| haversine_distance(existing.location, candidate.location) >= min_distance_m)
        {
            locations.push(candidate);
        }

        if locations.len() >= n {
            break;
        }
    }

    if locations.is_empty() {
        return Err("No locations found".into());
    }

    // Compute centroid
    let (sum_lat, sum_lng) = locations.iter().fold((0.0, 0.0), |acc, loc| {
        (acc.0 + loc.location.0, acc.1 + loc.location.1)
    });
    let centroid = (sum_lat / locations.len() as f64, sum_lng / locations.len() as f64);

    // Compute max distance from centroid
    let max_dist = locations
        .iter()
        .map(|loc| {
            let dx = loc.location.0 - centroid.0;
            let dy = loc.location.1 - centroid.1;
            (dx * dx + dy * dy).sqrt()
        })
        .fold(0.0, f64::max)
        .max(1e-9); // avoid division by zero

    // Scale factor so farthest point is at, say, 90% of the circle radius
    let scale = 0.9 / max_dist;

    // Normalize locations to inside the circle
    let normalized_locations: Vec<Place> = locations
        .iter()
        .map(|loc| {
            let dx = loc.location.0 - centroid.0;
            let dy = loc.location.1 - centroid.1;
            Place {
                name: loc.name.clone(),
                location: (dx * scale, dy * scale),
            }
        })
        .collect();

    // Normalize routes the same way
    let mut routes = Vec::new();

    // inside the for loop over locations
    for i in 0..locations.len().saturating_sub(1) {
        let origin = locations[i].location;
        let dest = locations[i + 1].location;
        let directions_url = format!(
            "https://maps.googleapis.com/maps/api/directions/json?origin={},{}&destination={},{}&mode=driving&key={}",
            origin.0, origin.1, dest.0, dest.1, api_key
        );

        let dir_res: DirectionsResponse = client.get(&directions_url).send().await?.json().await?;
        if let Some(route) = dir_res.routes.get(0) {
            let decoded = decode_polyline(&route.overview_polyline.points);

            // Normalize points
            let normalized: Vec<(f64, f64)> = decoded
                .iter()
                .map(|&(lat, lng)| ((lat - centroid.0) * scale, (lng - centroid.1) * scale))
                .collect();

            routes.push(normalized);
        }
    }


    // Rotate normalized locations 90° counterclockwise
    let rotated_locations: Vec<Place> = normalized_locations
        .into_iter()
        .map(|loc| Place {
            name: loc.name,
            location: (loc.location.1, -loc.location.0), // rotate 90° left
        })
        .collect();

    // Rotate normalized routes 90° counterclockwise
    let rotated_routes: Vec<Vec<(f64, f64)>> = routes
        .into_iter()
        .map(|route| {
            route
                .into_iter()
                .map(|(x, y)| (y, -x)) // rotate 90° left
                .collect()
        })
        .collect();

    // Mirror vertically after rotation
    let mirrored_locations: Vec<Place> = rotated_locations
        .into_iter()
        .map(|loc| Place {
            name: loc.name,
            location: (loc.location.0, -loc.location.1), // mirror vertically
        })
        .collect();

    let mirrored_routes: Vec<Vec<(f64, f64)>> = rotated_routes
        .into_iter()
        .map(|route| {
            route
                .into_iter()
                .map(|(x, y)| (x, -y)) // mirror vertically
                .collect()
        })
        .collect();

    Ok(Map {
        locations: mirrored_locations,
        routes: mirrored_routes,
    })

}
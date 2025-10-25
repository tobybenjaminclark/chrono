use plotters::prelude::*;
use std::error::Error;
use std::f64::consts::PI;
use crate::types::{Map, Event, Faction, Ownership};

pub fn viz_map(map: &Map, ownership: &Ownership) -> Result<(), Box<dyn Error>> {
    // Create drawing area
    let root = BitMapBackend::new("map.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Map Visualization", ("sans-serif", 25))
        .build_cartesian_2d(-1.1f64..1.1f64, -1.1f64..1.1f64)?;

    chart.configure_mesh().disable_mesh().draw()?;

    // Draw unit circle boundary
    let circle_points: Vec<(f64, f64)> = (0..360)
        .map(|d| {
            let rad = (d as f64) * PI / 180.0;
            (rad.cos(), rad.sin())
        })
        .collect();
    chart.draw_series(LineSeries::new(circle_points, &BLACK))?;

    // --- Plot routes ---
    for route in &map.routes {
        chart.draw_series(LineSeries::new(route.clone(), &BLUE.mix(0.6)))?;
    }

    // Helper to get color by faction
    let faction_color = |f: &Faction| match f {
        Faction::Gnomes => &GREEN,
        Faction::Trolls => &RED,
        Faction::Centaurs => &MAGENTA,
    };

    // --- Plot locations colored by faction ---
    for place in &map.locations {
        if let Some(faction) = ownership.get(place) {
            chart.draw_series(PointSeries::of_element(
                vec![place.location],
                8,
                faction_color(faction),
                &|c, s, st| {
                    EmptyElement::at(c)
                        + Circle::new((0, 0), s, st.filled())
                        + Text::new(place.name.clone(), (10, 0), ("sans-serif", 15).into_font())
                },
            ))?;
        }
    }

    root.present()?;
    println!("✅ Map saved to {}", "map.png");
    Ok(())
}

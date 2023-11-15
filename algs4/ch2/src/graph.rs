// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use plotters::prelude::*;
use std::cmp;
use std::fmt;
use std::ops;
use std::path::Path;

pub fn export_graph<P>(list: &[i32], filename: P) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    if list.is_empty() {
        return Ok(());
    }

    let root = BitMapBackend::new(filename.as_ref(), (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;
    let min_value = list
        .iter()
        .min()
        .expect("Failed to get min value from list");
    let max_value = list
        .iter()
        .max()
        .expect("Failed to get max value from list");
    let len = list.len();

    let mut chart = ChartBuilder::on(&root).margin(8).build_cartesian_2d(
        (0..(len as i32 + 2)).into_segmented(),
        *min_value..(*max_value + 2),
    )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.2))
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    let data: Vec<(i32, i32)> = list
        .iter()
        .enumerate()
        .map(|(index, val)| ((index + 1) as i32, *val))
        .collect();
    println!("data: {:?}", data);

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}

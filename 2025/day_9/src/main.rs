use std::fs;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 2: {}", part_2(input));
}

fn plot(input: &Vec<(usize, usize)>, name: &str) {
    let input = input
        .iter()
        .map(|i| (i.0 as f64, i.1 as f64))
        .collect::<Vec<(f64, f64)>>();
    // Parse tiles as (x, y)
    let tiles = input;

    if tiles.is_empty() {
        eprintln!("plot: no tiles to plot");
        return;
    }

    // Determine bounds
    let mut min_x = tiles[0].0;
    let mut max_x = tiles[0].0;
    let mut min_y = tiles[0].1;
    let mut max_y = tiles[0].1;
    for &(x, y) in tiles.iter() {
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let raw_w = (max_x - min_x).abs();
    let raw_h = (max_y - min_y).abs();

    // Canvas config (scaled up for deep zooming)
    let mut width: f64 = 4000.0;
    let mut height: f64 = 4000.0;
    let padding: f64 = 200.0;

    // If the aspect ratio is very skewed, adjust one side to keep points visible
    let aspect = if raw_h == 0.0 { 1.0 } else { raw_w / raw_h };
    if aspect > 2.0 {
        width = 6000.0;
        height = 3000.0;
    } else if aspect < 0.5 {
        width = 3000.0;
        height = 6000.0;
    }

    let drawable_w = (width - 2.0 * padding).max(1.0);
    let drawable_h = (height - 2.0 * padding).max(1.0);

    let span_w = raw_w.max(1.0);
    let span_h = raw_h.max(1.0);
    let scale = drawable_w / span_w.min(drawable_w / drawable_h * span_h).max(1.0);
    // Better simple uniform scale to fit both dimensions
    let scale = drawable_w.min(drawable_h) / span_w.max(span_h).max(1.0);

    // Transform to SVG coordinates (y grows down). Also center if one span is smaller than the other
    let used_w = span_w * scale;
    let used_h = span_h * scale;
    let offset_x = padding + (drawable_w - used_w).max(0.0) / 2.0;
    let offset_y = padding + (drawable_h - used_h).max(0.0) / 2.0;

    let transform = |x: f64, y: f64| -> (f64, f64) {
        let nx = offset_x + (x - min_x) * scale;
        // invert Y so larger y appears lower on the image
        let ny = offset_y + (max_y - y) * scale;
        (nx, ny)
    };

    // Build polyline string, circles, and labels
    let mut poly_points = String::new();
    let mut circles = String::new();
    let mut labels = String::new();
    for (i, &(x, y)) in tiles.iter().enumerate() {
        let top = i % 2 == 0;
        let (sx, sy) = transform(x, y);
        if i > 0 {
            poly_points.push(' ');
        }
        poly_points.push_str(&format!("{:.2},{:.2}", sx, sy));
        circles.push_str(&format!(
            "    <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"6\" fill=\"#e63946\" stroke=\"white\" stroke-width=\"2\" />\n",
            sx, sy
        ));
        // Label: index (1-based) and original coordinates
        // Slight offset to avoid overlapping the point
        let label = format!("{}: ({:.0},{:.0})", i + 1, x, y);
        let x1 = sy - 6.0 * (if top { 1.0 } else { -1.0 });
        labels.push_str(&format!(
            "    <text x=\"{:.2}\" y=\"{:.2}\" font-size=\"5\" font-family=\"monospace\" fill=\"#f1faee\" stroke=\"#0b132b\" stroke-width=\"4\" paint-order=\"stroke\">{}</text>\n",
            sx + 10.0, x1, label
        ));
    }

    let svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">\n  <rect x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" fill=\"#0b132b\"/>\n  <g>\n    <polyline points=\"{pts}\" fill=\"none\" stroke=\"#a8dadc\" stroke-width=\"6\" stroke-linejoin=\"round\" stroke-linecap=\"round\"/>\n{circles}{labels}  </g>\n</svg>\n",
        w = width,
        h = height,
        pts = poly_points,
        circles = circles,
        labels = labels
    );

    // Write to file next to the binary execution directory
    let out_path = &format!("{}.svg", name);
    match fs::write(out_path, svg) {
        Ok(_) => {
            println!("Wrote SVG plot with {} points to {}", tiles.len(), out_path);
        }
        Err(e) => {
            eprintln!("Failed to write {}: {}", out_path, e);
        }
    }
}

fn part_1(input: &str) -> i64 {
    let tiles = input
        .lines()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .collect::<Vec<(usize, usize)>>();

    (0..tiles.len() - 1)
        .enumerate()
        .map(|(index, a)| {
            (index + 1..tiles.len())
                .map(|b| {
                    let a = tiles[a];
                    let b = tiles[b];
                    let width = (a.0 as i64 - b.0 as i64).abs() + 1;
                    let height = (a.1 as i64 - b.1 as i64).abs() + 1;
                    width * height
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn part_2(input: &str) -> i64 {
    let tiles = input
        .lines()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .collect::<Vec<(usize, usize)>>();
    plot(&tiles, "plot");

    let mut max = 0;

    (0..tiles.len() - 1)
        .enumerate()
        .map(|(index, a)| {
            (index + 1..tiles.len())
                .map(|b| {
                    let a = tiles[a];
                    let b = tiles[b];
                    let width = (a.0 as i64 - b.0 as i64).abs() + 1;
                    let height = (a.1 as i64 - b.1 as i64).abs() + 1;
                    let area = width * height;
                    if area > max {
                        println!("{:?}  {:?}", a, b);
                        max = area;
                    }
                    area
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt")), 50);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../resource/test.txt")), 24);
    }

    #[test]
    fn more_test() {
        assert_eq!(part_2(include_str!("../resource/more.txt")), 83);
    }
}

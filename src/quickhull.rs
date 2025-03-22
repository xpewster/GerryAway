type Point = [f64; 2];

/// Calculates the convex hull of a set of points using the QuickHull algorithm
pub fn quick_hull(points: &[Point]) -> Vec<Point> {
    // Find the leftmost and rightmost points
    let (min_idx, max_idx) = find_min_max_x_indices(points);
    
    // Initialize result vectors - will collect indices during computation
    let mut hull_indices = Vec::new();
    
    // Two arrays to track points on each side (using indices for efficiency)
    let mut s1 = Vec::new();
    let mut s2 = Vec::new();

    // Divide points by which side they lie on
    for i in 0..points.len() {
        if i == min_idx || i == max_idx {
            continue;
        }

        let side = which_side(&points[min_idx], &points[max_idx], &points[i]);
        if side > 0.0 {
            s1.push(i);
        } else if side < 0.0 {
            s2.push(i);
        }
    }

    // Build hull by finding points on each side
    hull_indices.push(min_idx);
    find_hull(&points, &mut hull_indices, &s1, min_idx, max_idx);
    hull_indices.push(max_idx);
    find_hull(&points, &mut hull_indices, &s2, max_idx, min_idx);

    // Convert indices to actual points only at the end
    hull_indices.iter().map(|&idx| points[idx]).collect()
}

/// Find the indices of points with minimum and maximum x-coordinates
fn find_min_max_x_indices(points: &[Point]) -> (usize, usize) {
    let mut min_idx = 0;
    let mut max_idx = 0;

    for i in 1..points.len() {
        if points[i][0] < points[min_idx][0] {
            min_idx = i;
        }
        if points[i][0] > points[max_idx][0] {
            max_idx = i;
        }
    }

    (min_idx, max_idx)
}

/// Determine which side of line AB the point P lies on
/// Returns positive if P is to the right of AB, negative if to the left
fn which_side(a: &Point, b: &Point, p: &Point) -> f64 {
    (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
}

/// Calculate the square of the distance from point p to line segment ab
fn distance_squared_to_line(a: &Point, b: &Point, p: &Point) -> f64 {
    let ab_x = b[0] - a[0];
    let ab_y = b[1] - a[1];
    let ap_x = p[0] - a[0];
    let ap_y = p[1] - a[1];
    
    let cross_product = ab_x * ap_y - ab_y * ap_x;
    cross_product * cross_product / (ab_x * ab_x + ab_y * ab_y)
}

/// Recursive function to find the hull (using indices for efficiency)
fn find_hull(
    points: &[Point], 
    hull_indices: &mut Vec<usize>, 
    point_indices: &[usize], 
    a_idx: usize, 
    b_idx: usize
) {
    if point_indices.is_empty() {
        return;
    }

    // Find point with maximum distance
    let mut max_dist = 0.0;
    let mut farthest_idx = 0;
    let mut farthest_original_idx = 0;

    for (i, &idx) in point_indices.iter().enumerate() {
        let dist = distance_squared_to_line(&points[a_idx], &points[b_idx], &points[idx]);
        if dist > max_dist {
            max_dist = dist;
            farthest_idx = i;
            farthest_original_idx = idx;
        }
    }

    // Create two sublists
    let mut s1 = Vec::new();
    let mut s2 = Vec::new();

    for &idx in point_indices {
        if idx == farthest_original_idx {
            continue;
        }

        let side1 = which_side(&points[a_idx], &points[farthest_original_idx], &points[idx]);
        let side2 = which_side(&points[farthest_original_idx], &points[b_idx], &points[idx]);

        if side1 > 0.0 {
            s1.push(idx);
        } else if side2 > 0.0 {
            s2.push(idx);
        }
    }

    // Recursively find hull points
    find_hull(points, hull_indices, &s1, a_idx, farthest_original_idx);
    
    // Insert the current point at the right position (between a and b)
    let position = hull_indices.iter().position(|&x| x == b_idx).unwrap_or(hull_indices.len());
    hull_indices.insert(position, farthest_original_idx);
    
    find_hull(points, hull_indices, &s2, farthest_original_idx, b_idx);
}

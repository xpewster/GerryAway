use crate::quickhull::quick_hull;

type Point = [f64; 2];

/// Calculates the minimum area bounding rectangle for a polygon and returns its aspect ratio
/// The aspect ratio is defined as the longer dimension divided by the shorter dimension
pub fn min_bounding_rectangle_aspect_ratio(points: &[Point]) -> f64 {
    // First compute the convex hull
    let hull = quick_hull(points);

    // Find the minimum bounding rectangle using rotating calipers
    let mut min_area = f64::MAX;
    let mut min_rect = vec![];
    let mut min_width = 0.0;
    let mut min_height = 0.0;
    
    // For each edge of the convex hull
    for i in 0..hull.len() {
        let j = (i + 1) % hull.len();
        
        // Get the edge vector
        let edge_x = hull[j][0] - hull[i][0];
        let edge_y = hull[j][1] - hull[i][1];
        
        // Normalize to get the direction
        let length = (edge_x * edge_x + edge_y * edge_y).sqrt();
        let dir_x = edge_x / length;
        let dir_y = edge_y / length;
        
        // Perpendicular direction
        let perp_x = -dir_y;
        let perp_y = dir_x;
        
        // Find extreme points along these directions
        let mut min_along_dir = f64::MAX;
        let mut max_along_dir = f64::MIN;
        let mut min_along_perp = f64::MAX;
        let mut max_along_perp = f64::MIN;
        
        for point in &hull {
            // Project the point onto the direction and perpendicular direction
            let proj_dir = point[0] * dir_x + point[1] * dir_y;
            let proj_perp = point[0] * perp_x + point[1] * perp_y;
            
            min_along_dir = min_along_dir.min(proj_dir);
            max_along_dir = max_along_dir.max(proj_dir);
            min_along_perp = min_along_perp.min(proj_perp);
            max_along_perp = max_along_perp.max(proj_perp);
        }
        
        // Calculate the width and height of this bounding rectangle
        let width = max_along_dir - min_along_dir;
        let height = max_along_perp - min_along_perp;
        let area = width * height;
        
        if area < min_area {
            min_area = area;
            min_width = width;
            min_height = height;
            
            // Calculate the four corners of the rectangle
            let mut rect = vec![];
            
            // Bottom-left corner
            rect.push(vec![
                min_along_dir * dir_x + min_along_perp * perp_x,
                min_along_dir * dir_y + min_along_perp * perp_y
            ]);
            
            // Bottom-right corner
            rect.push(vec![
                max_along_dir * dir_x + min_along_perp * perp_x,
                max_along_dir * dir_y + min_along_perp * perp_y
            ]);
            
            // Top-right corner
            rect.push(vec![
                max_along_dir * dir_x + max_along_perp * perp_x,
                max_along_dir * dir_y + max_along_perp * perp_y
            ]);
            
            // Top-left corner
            rect.push(vec![
                min_along_dir * dir_x + max_along_perp * perp_x,
                min_along_dir * dir_y + max_along_perp * perp_y
            ]);
            
            min_rect = rect;
        }
    }
    
    // Calculate aspect ratio (long side / short side)
    let aspect_ratio = if min_width > min_height {
        min_width / min_height
    } else {
        min_height / min_width
    };
    
    aspect_ratio
}

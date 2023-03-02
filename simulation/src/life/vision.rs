use crate::world::Level;

pub fn look<const SIZE: usize>(
    level: &Level<SIZE>,
    x: usize,
    y: usize,
    radius: usize,
) -> Vec<f32> {
    let vsize = radius * 2 + 1;
    let mut vision = vec![0.0; vsize * vsize];
    for xoff in -(radius as i64)..=radius as _ {
        'yl: for yoff in -(radius as i64)..=radius as _ {
            let vi = (xoff + radius as i64) as usize * vsize + (yoff + radius as i64) as usize;
            let dist2 = xoff * xoff + yoff * yoff;
            if dist2 as usize >= radius * radius {
                vision[vi] = 0.0;
                continue;
            }
            for i in 0..=dist2 {
                let i = i as f64 / (dist2 as f64);
                let ix = xoff as f64 * i;
                let iy = yoff as f64 * i;
                let vx = (x as i64 + ix as i64 + SIZE as i64) as usize % SIZE;
                let vy = (y as i64 + iy as i64 + SIZE as i64) as usize % SIZE;
                if level[(vx as usize, vy as usize)].tile.is_opaque() {
                    continue 'yl;
                }
            }
            vision[vi] = ((radius * radius) as i64 - dist2) as f32 / (radius * radius) as f32;
        }
    }
    vision
}
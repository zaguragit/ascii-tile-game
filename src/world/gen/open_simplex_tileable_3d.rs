
/*
 * OpenSimplex Noise
 * by Kurt Spencer
 * 
 * Tileable 3D version, preliminary release.
 * Could probably use further optimization.
 *
 * w6, h6, and d6 are each 1/6 of the repeating period.
 * for x, y, z respectively. If w6 = 2, h6 = 2, d6 = 2,
 * then the noise repeats in blocks of (0,0,0)->(12,12,12)
 */

const STRETCH_CONSTANT_3D: f64 = -1.0 / 6.0;    // (1/Math.sqrt(3+1)-1)/3;
const SQUISH_CONSTANT_3D: f64 = 1.0 / 3.0;      // (Math.sqrt(3+1)-1)/3;

const NORM_CONSTANT_3D: f64 = 103.0;

	
//Gradients for 3D. They approximate the directions to the
//vertices of a rhombicuboctahedron from the center, skewed so
//that the triangular and square facets can be inscribed inside
//circles of the same radius.
const GRADIENTS_3D: [i8; 72] = [
    -11,  4,  4,     -4,  11,  4,    -4,  4,  11,
     11,  4,  4,      4,  11,  4,     4,  4,  11,
    -11, -4,  4,     -4, -11,  4,    -4, -4,  11,
     11, -4,  4,      4, -11,  4,     4, -4,  11,
    -11,  4, -4,     -4,  11, -4,    -4,  4, -11,
     11,  4, -4,      4,  11, -4,     4,  4, -11,
    -11, -4, -4,     -4, -11, -4,    -4, -4, -11,
     11, -4, -4,      4, -11, -4,     4, -4, -11,
];
	
fn fast_floor(x: f64) -> i32 {
    let xi = x as i32;
    if x < xi as f64 { xi - 1 } else { xi }
}

pub struct OpenSimplexTileable3D {
	perm: [i16; 256],
	perm_grad_index_3d: [i16; 256],

	w6: i32,
	h6: i32,
	d6: i32,
	s_offset: i32,
}

impl OpenSimplexTileable3D {
	pub fn new_with_perm(perm: [i16; 256], w6: i32, h6: i32, d6: i32) -> Self {
		let mut perm_grad_index_3d = [0i16; 256];
		let s_offset = w6.max(h6.max(d6)) * 6;
		
		for i in 0..256 {
			//Since 3D has 24 gradients, simple bitmask won't work, so precompute modulo array.
			perm_grad_index_3d[i] = ((perm[i] % (GRADIENTS_3D.len() as i16 / 3)) * 3) as i16;
		}
        Self { perm, perm_grad_index_3d, w6, h6, d6, s_offset }
	}

	pub fn new_with_seed_square(mut seed: i64, s6: i32) -> Self {
		Self::new_with_seed(seed, s6, s6, s6)
	}
	
	//Initializes the class using a permutation array generated from a 64-bit seed.
	//Generates a proper permutation (i.e. doesn't merely perform N successive pair swaps on a base array)
	//Uses a simple 64-bit LCG.
	pub fn new_with_seed(mut seed: i64, w6: i32, h6: i32, d6: i32) -> Self {
		let mut perm = [0i16; 256];
		let mut perm_grad_index_3d = [0i16; 256];
		let mut source = [0i16; 256];
		for i in 0..256 {
			source[i] = i as _;
        }
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
		for i in (0..256).rev() {
			seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
			let mut r = ((seed + 31) % (i as i64 + 1)) as i32;
			if r < 0 {
				r += i as i32 + 1;
            }
			perm[i] = source[r as usize];
			perm_grad_index_3d[i] = ((perm[i] % (GRADIENTS_3D.len() as i16 / 3)) * 3) as i16;
			source[r as usize] = source[i];
		}
		let s_offset = w6.max(h6.max(d6)) * 6;
        Self { perm, perm_grad_index_3d, w6, h6, d6, s_offset }
	}
}

impl OpenSimplexTileable3D {
	
	//3D OpenSimplex Noise.
	pub fn eval(&self, x: f64, y: f64, z: f64) -> f64 {

		//Place input coordinates on simplectic honeycomb.
		let stretch_offset = (x + y + z) * STRETCH_CONSTANT_3D;
		let xs = x + stretch_offset;
		let ys = y + stretch_offset;
		let zs = z + stretch_offset;
		
		//Floor to get simplectic honeycomb coordinates of rhombohedron (stretched cube) super-cell origin.
		let mut xsb = fast_floor(xs);
		let mut ysb = fast_floor(ys);
		let mut zsb = fast_floor(zs);
		
		//Skew out to get actual coordinates of rhombohedron origin. We'll need these later.
		let squish_offset = (xsb + ysb + zsb) as f64 * SQUISH_CONSTANT_3D;
		let xb = xsb as f64 + squish_offset;
		let yb = ysb as f64 + squish_offset;
		let zb = zsb as f64 + squish_offset;
		
		//Compute simplectic honeycomb coordinates relative to rhombohedral origin.
		let xins = xs - xsb as f64;
		let yins = ys - ysb as f64;
		let zins = zs - zsb as f64;
		
		//Sum those together to get a value that determines which region we're in.
		let in_sum = xins + yins + zins;

		//Positions relative to origin point.
		let mut dx0 = x - xb;
		let mut dy0 = y - yb;
		let mut dz0 = z - zb;
		
		//From here on out, these can't be negative.
		xsb += self.s_offset;
		ysb += self.s_offset;
		zsb += self.s_offset;
		
		//We'll be defining these inside the next block and using them afterwards.
		let (dx_ext0, mut dy_ext0, dz_ext0): (f64, f64, f64);
		let (mut dx_ext1, mut dy_ext1, mut dz_ext1): (f64, f64, f64);
		let (xsv_ext0, mut ysv_ext0, zsv_ext0): (i32, i32, i32);
		let (mut xsv_ext1, mut ysv_ext1, mut zsv_ext1): (i32, i32, i32);
		
		let mut value = 0.0;
		if in_sum <= 1.0 { //We're inside the tetrahedron (3-Simplex) at (0,0,0)
			
			//Determine which two of (0,0,1), (0,1,0), (1,0,0) are closest.
			let mut a_point = 0x01i8;
			let mut a_score = xins;
			let mut b_point = 0x02i8;
			let mut b_score = yins;
			if a_score >= b_score && zins > b_score {
				b_score = zins;
				b_point = 0x04;
			} else if a_score < b_score && zins > a_score {
				a_score = zins;
				a_point = 0x04;
			}
			
			//Now we determine the two lattice points not part of the tetrahedron that may contribute.
			//This depends on the closest two tetrahedral vertices, including (0,0,0)
			let wins = 1.0 - in_sum;
			if wins > a_score || wins > b_score { //(0,0,0) is one of the closest two tetrahedral vertices.
				let c = if b_score > a_score { b_point } else { a_point }; //Our other closest vertex is the closest out of a and b.
				
				if (c & 0x01) == 0 {
					xsv_ext0 = xsb - 1;
					xsv_ext1 = xsb;
					dx_ext0 = dx0 + 1.0;
					dx_ext1 = dx0;
				} else {
					xsv_ext1 = xsb + 1;
                    xsv_ext0 = xsv_ext1;
					dx_ext1 = dx0 - 1.0;
                    dx_ext0 = dx_ext1;
				}

				if (c & 0x02) == 0 {
					ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
					dy_ext1 = dy0;
                    dy_ext0 = dy_ext1;
					if (c & 0x01) == 0 {
						ysv_ext1 -= 1;
						dy_ext1 += 1.0;
					} else {
						ysv_ext0 -= 1;
						dy_ext0 += 1.0;
					}
				} else {
					ysv_ext1 = ysb + 1;
					ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0 - 1.0;
                    dy_ext0 = dy_ext1;
				}

				if (c & 0x04) == 0 {
					zsv_ext0 = zsb;
					zsv_ext1 = zsb - 1;
					dz_ext0 = dz0;
					dz_ext1 = dz0 + 1.0;
				} else {
					zsv_ext1 = zsb + 1;
                    zsv_ext0 = zsv_ext1;
					dz_ext1 = dz0 - 1.0;
                    dz_ext0 = dz_ext1;
				}
			} else { //(0,0,0) is not one of the closest two tetrahedral vertices.
				let c = (a_point | b_point) as i8; //Our two extra vertices are determined by the closest two.
				
				if (c & 0x01) == 0 {
					xsv_ext0 = xsb;
					xsv_ext1 = xsb - 1;
					dx_ext0 = dx0 - 2.0 * SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 + 1.0 - SQUISH_CONSTANT_3D;
				} else {
					xsv_ext1 = xsb + 1;
                    xsv_ext0 = xsv_ext1;
					dx_ext0 = dx0 - 1.0 - 2.0 * SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 1.0 - SQUISH_CONSTANT_3D;
				}

				if (c & 0x02) == 0 {
					ysv_ext0 = ysb;
					ysv_ext1 = ysb - 1;
					dy_ext0 = dy0 - 2.0 * SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 + 1.0 - SQUISH_CONSTANT_3D;
				} else {
					ysv_ext1 = ysb + 1;
                    ysv_ext0 = ysv_ext1;
					dy_ext0 = dy0 - 1.0 - 2.0 * SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 1.0 - SQUISH_CONSTANT_3D;
				}

				if (c & 0x04) == 0 {
					zsv_ext0 = zsb;
					zsv_ext1 = zsb - 1;
					dz_ext0 = dz0 - 2.0 * SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 + 1.0 - SQUISH_CONSTANT_3D;
				} else {
					zsv_ext1 = zsb + 1;
                    zsv_ext0 = zsv_ext1;
					dz_ext0 = dz0 - 1.0 - 2.0 * SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 1.0 - SQUISH_CONSTANT_3D;
				}
			}

			//Contribution (0,0,0)
			let mut attn0 = 2.0 - dx0 * dx0 - dy0 * dy0 - dz0 * dz0;
			if attn0 > 0.0 {
				attn0 *= attn0;
				value += attn0 * attn0 * self.extrapolate(xsb + 0, ysb + 0, zsb + 0, dx0, dy0, dz0);
			}

			//Contribution (1,0,0)
			let dx1 = dx0 - 1.0 - SQUISH_CONSTANT_3D;
			let dy1 = dy0 - 0.0 - SQUISH_CONSTANT_3D;
			let dz1 = dz0 - 0.0 - SQUISH_CONSTANT_3D;
			let mut attn1 = 2.0 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
			if attn1 > 0.0 {
				attn1 *= attn1;
				value += attn1 * attn1 * self.extrapolate(xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1);
			}

			//Contribution (0,1,0)
			let dx2 = dx0 - 0.0 - SQUISH_CONSTANT_3D;
			let dy2 = dy0 - 1.0 - SQUISH_CONSTANT_3D;
			let dz2 = dz1;
			let mut attn2 = 2.0 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
			if attn2 > 0.0 {
				attn2 *= attn2;
				value += attn2 * attn2 * self.extrapolate(xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2);
			}

			//Contribution (0,0,1)
			let dx3 = dx2;
			let dy3 = dy1;
			let dz3 = dz0 - 1.0 - SQUISH_CONSTANT_3D;
			let mut attn3 = 2.0 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
			if attn3 > 0.0 {
				attn3 *= attn3;
				value += attn3 * attn3 * self.extrapolate(xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3);
			}
		} else if in_sum >= 2.0 { //We're inside the tetrahedron (3-Simplex) at (1,1,1)
		
			//Determine which two tetrahedral vertices are the closest, out of (1,1,0), (1,0,1), (0,1,1) but not (1,1,1).
			let mut a_point = 0x06i8;
			let mut a_score = xins;
			let mut b_point = 0x05i8;
			let mut b_score = yins;
			if a_score <= b_score && zins < b_score {
				b_score = zins;
				b_point = 0x03;
			} else if a_score > b_score && zins < a_score {
				a_score = zins;
				a_point = 0x03;
			}
			
			//Now we determine the two lattice points not part of the tetrahedron that may contribute.
			//This depends on the closest two tetrahedral vertices, including (1,1,1)
			let wins = 3.0 - in_sum;
			if wins < a_score || wins < b_score { //(1,1,1) is one of the closest two tetrahedral vertices.
				let c = if b_score < a_score { b_point } else { a_point }; //Our other closest vertex is the closest out of a and b.
				
				if (c & 0x01) != 0 {
					xsv_ext0 = xsb + 2;
					xsv_ext1 = xsb + 1;
					dx_ext0 = dx0 - 2.0 - 3.0 * SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 1.0 - 3.0 * SQUISH_CONSTANT_3D;
				} else {
					xsv_ext1 = xsb;
                    xsv_ext0 = xsv_ext1;
					dx_ext1 = dx0 - 3.0 * SQUISH_CONSTANT_3D;
                    dx_ext0 = dx_ext1;
				}

				if (c & 0x02) != 0 {
					ysv_ext1 = ysb + 1;
                    ysv_ext0 = ysv_ext1;
					dy_ext1 = dy0 - 1.0 - 3.0 * SQUISH_CONSTANT_3D;
                    dy_ext0 = dy_ext1;
					if (c & 0x01) != 0 {
						ysv_ext1 += 1;
						dy_ext1 -= 1.0;
					} else {
						ysv_ext0 += 1;
						dy_ext0 -= 1.0;
					}
				} else {
					ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
					dy_ext1 = dy0 - 3.0 * SQUISH_CONSTANT_3D;
                    dy_ext0 = dy_ext1;
				}

				if (c & 0x04) != 0 {
					zsv_ext0 = zsb + 1;
					zsv_ext1 = zsb + 2;
					dz_ext0 = dz0 - 1.0 - 3.0 * SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 2.0 - 3.0 * SQUISH_CONSTANT_3D;
				} else {
					zsv_ext1 = zsb;
                    zsv_ext0 = zsv_ext1;
					dz_ext1 = dz0 - 3.0 * SQUISH_CONSTANT_3D;
                    dz_ext0 = dz_ext1;
				}
			} else { //(1,1,1) is not one of the closest two tetrahedral vertices.
				let c = (a_point & b_point) as i8; //Our two extra vertices are determined by the closest two.
				
				if (c & 0x01) != 0 {
					xsv_ext0 = xsb + 1;
					xsv_ext1 = xsb + 2;
					dx_ext0 = dx0 - 1.0 - SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 2.0 - 2.0 * SQUISH_CONSTANT_3D;
				} else {
					xsv_ext1 = xsb;
                    xsv_ext0 = xsv_ext1;
					dx_ext0 = dx0 - SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 2.0 * SQUISH_CONSTANT_3D;
				}

				if (c & 0x02) != 0 {
					ysv_ext0 = ysb + 1;
					ysv_ext1 = ysb + 2;
					dy_ext0 = dy0 - 1.0 - SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 2.0 - 2.0 * SQUISH_CONSTANT_3D;
				} else {
					ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
					dy_ext0 = dy0 - SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 2.0 * SQUISH_CONSTANT_3D;
				}

				if (c & 0x04) != 0 {
					zsv_ext0 = zsb + 1;
					zsv_ext1 = zsb + 2;
					dz_ext0 = dz0 - 1.0 - SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 2.0 - 2.0 * SQUISH_CONSTANT_3D;
				} else {
					zsv_ext1 = zsb;
                    zsv_ext0 = zsv_ext1;
					dz_ext0 = dz0 - SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 2.0 * SQUISH_CONSTANT_3D;
				}
			}
			
			//Contribution (1,1,0)
			let dx3 = dx0 - 1.0 - 2.0 * SQUISH_CONSTANT_3D;
			let dy3 = dy0 - 1.0 - 2.0 * SQUISH_CONSTANT_3D;
			let dz3 = dz0 - 0.0 - 2.0 * SQUISH_CONSTANT_3D;
			let mut attn3 = 2.0 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
			if attn3 > 0.0 {
				attn3 *= attn3;
				value += attn3 * attn3 * self.extrapolate(xsb + 1, ysb + 1, zsb + 0, dx3, dy3, dz3);
			}

			//Contribution (1,0,1)
			let dx2 = dx3;
			let dy2 = dy0 - 0.0 - 2.0 * SQUISH_CONSTANT_3D;
			let dz2 = dz0 - 1.0 - 2.0 * SQUISH_CONSTANT_3D;
			let mut attn2 = 2.0 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
			if attn2 > 0.0 {
				attn2 *= attn2;
				value += attn2 * attn2 * self.extrapolate(xsb + 1, ysb + 0, zsb + 1, dx2, dy2, dz2);
			}

			//Contribution (0,1,1)
			let dx1 = dx0 - 0.0 - 2.0 * SQUISH_CONSTANT_3D;
			let dy1 = dy3;
			let dz1 = dz2;
			let mut attn1 = 2.0 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
			if attn1 > 0.0 {
				attn1 *= attn1;
				value += attn1 * attn1 * self.extrapolate(xsb + 0, ysb + 1, zsb + 1, dx1, dy1, dz1);
			}

			//Contribution (1,1,1)
			dx0 = dx0 - 1.0 - 3.0 * SQUISH_CONSTANT_3D;
			dy0 = dy0 - 1.0 - 3.0 * SQUISH_CONSTANT_3D;
			dz0 = dz0 - 1.0 - 3.0 * SQUISH_CONSTANT_3D;
			let mut attn0 = 2.0 - dx0 * dx0 - dy0 * dy0 - dz0 * dz0;
			if attn0 > 0.0 {
				attn0 *= attn0;
				value += attn0 * attn0 * self.extrapolate(xsb + 1, ysb + 1, zsb + 1, dx0, dy0, dz0);
			}
		} else { //We're inside the octahedron (Rectified 3-Simplex) in between.
			let mut a_score: f64;
			let mut a_point: i8;
			let mut a_is_further_side: bool;
			let mut b_score: f64;
			let mut b_point: i8;
			let mut b_is_further_side: bool;

			//Decide between point (0,0,1) and (1,1,0) as closest
			let p1 = xins + yins;
			if p1 > 1.0 {
				a_score = p1 - 1.0;
				a_point = 0x03;
				a_is_further_side = true;
			} else {
				a_score = 1.0 - p1;
				a_point = 0x04;
				a_is_further_side = false;
			}

			//Decide between point (0,1,0) and (1,0,1) as closest
			let p2 = xins + zins;
			if p2 > 1.0 {
				b_score = p2 - 1.0;
				b_point = 0x05;
				b_is_further_side = true;
			} else {
				b_score = 1.0 - p2;
				b_point = 0x02;
				b_is_further_side = false;
			}
			
			//The closest out of the two (1,0,0) and (0,1,1) will replace the furthest out of the two decided above, if closer.
			let p3 = yins + zins;
			if p3 > 1.0 {
				let score = p3 - 1.0;
				if a_score <= b_score && a_score < score {
					a_score = score;
					a_point = 0x06;
					a_is_further_side = true;
				} else if a_score > b_score && b_score < score {
					b_score = score;
					b_point = 0x06;
					b_is_further_side = true;
				}
			} else {
				let score = 1.0 - p3;
				if a_score <= b_score && a_score < score {
					a_score = score;
					a_point = 0x01;
					a_is_further_side = false;
				} else if a_score > b_score && b_score < score {
					b_score = score;
					b_point = 0x01;
					b_is_further_side = false;
				}
			}
			
			//Where each of the two closest points are determines how the extra two vertices are calculated.
			if a_is_further_side == b_is_further_side {
				if a_is_further_side { //Both closest points on (1,1,1) side

					//One of the two extra points is (1,1,1)
					dx_ext0 = dx0 - 1.0 - 3.0 * SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 - 1.0 - 3.0 * SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 - 1.0 - 3.0 * SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb + 1;
					ysv_ext0 = ysb + 1;
					zsv_ext0 = zsb + 1;

					//Other extra point is based on the shared axis.
					let c = (a_point & b_point) as i8;
					if (c & 0x01) != 0 {
						dx_ext1 = dx0 - 2.0 - 2.0 * SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 2.0 * SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 2.0 * SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb + 2;
						ysv_ext1 = ysb;
						zsv_ext1 = zsb;
					} else if (c & 0x02) != 0 {
						dx_ext1 = dx0 - 2.0 * SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 2.0 - 2.0 * SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 2.0 * SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb;
						ysv_ext1 = ysb + 2;
						zsv_ext1 = zsb;
					} else {
						dx_ext1 = dx0 - 2.0 * SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 2.0 * SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 2.0 - 2.0 * SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb;
						ysv_ext1 = ysb;
						zsv_ext1 = zsb + 2;
					}
				} else {//Both closest points on (0,0,0) side

					//One of the two extra points is (0,0,0)
					dx_ext0 = dx0;
					dy_ext0 = dy0;
					dz_ext0 = dz0;
					xsv_ext0 = xsb;
					ysv_ext0 = ysb;
					zsv_ext0 = zsb;

					//Other extra point is based on the omitted axis.
					let c = (a_point | b_point) as i8;
					if (c & 0x01) == 0 {
						dx_ext1 = dx0 + 1.0 - SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 1.0 - SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 1.0 - SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb - 1;
						ysv_ext1 = ysb + 1;
						zsv_ext1 = zsb + 1;
					} else if (c & 0x02) == 0 {
						dx_ext1 = dx0 - 1.0 - SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 + 1.0 - SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 1.0 - SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb + 1;
						ysv_ext1 = ysb - 1;
						zsv_ext1 = zsb + 1;
					} else {
						dx_ext1 = dx0 - 1.0 - SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 1.0 - SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 + 1.0 - SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb + 1;
						ysv_ext1 = ysb + 1;
						zsv_ext1 = zsb - 1;
					}
				}
			} else { //One point on (0,0,0) side, one point on (1,1,1) side
				let (c1, c2): (i8, i8);
				if a_is_further_side {
					c1 = a_point;
					c2 = b_point;
				} else {
					c1 = b_point;
					c2 = a_point;
				}

				//One contribution is a permutation of (1,1,-1)
				if (c1 & 0x01) == 0 {
					dx_ext0 = dx0 + 1.0 - SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 - 1.0 - SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 - 1.0 - SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb - 1;
					ysv_ext0 = ysb + 1;
					zsv_ext0 = zsb + 1;
				} else if (c1 & 0x02) == 0 {
					dx_ext0 = dx0 - 1.0 - SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 + 1.0 - SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 - 1.0 - SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb + 1;
					ysv_ext0 = ysb - 1;
					zsv_ext0 = zsb + 1;
				} else {
					dx_ext0 = dx0 - 1.0 - SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 - 1.0 - SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 + 1.0 - SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb + 1;
					ysv_ext0 = ysb + 1;
					zsv_ext0 = zsb - 1;
				}

				//One contribution is a permutation of (0,0,2)
				dx_ext1 = dx0 - 2.0 * SQUISH_CONSTANT_3D;
				dy_ext1 = dy0 - 2.0 * SQUISH_CONSTANT_3D;
				dz_ext1 = dz0 - 2.0 * SQUISH_CONSTANT_3D;
				xsv_ext1 = xsb;
				ysv_ext1 = ysb;
				zsv_ext1 = zsb;
				if (c2 & 0x01) != 0 {
					dx_ext1 -= 2.0;
					xsv_ext1 += 2;
				} else if (c2 & 0x02) != 0 {
					dy_ext1 -= 2.0;
					ysv_ext1 += 2;
				} else {
					dz_ext1 -= 2.0;
					zsv_ext1 += 2;
				}
			}

			//Contribution (1,0,0)
			let dx1 = dx0 - 1.0 - SQUISH_CONSTANT_3D;
			let dy1 = dy0 - 0.0 - SQUISH_CONSTANT_3D;
			let dz1 = dz0 - 0.0 - SQUISH_CONSTANT_3D;
			let mut attn1 = 2.0 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
			if attn1 > 0.0 {
				attn1 *= attn1;
				value += attn1 * attn1 * self.extrapolate(xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1);
			}

			//Contribution (0,1,0)
			let dx2 = dx0 - 0.0 - SQUISH_CONSTANT_3D;
			let dy2 = dy0 - 1.0 - SQUISH_CONSTANT_3D;
			let dz2 = dz1;
			let mut attn2 = 2.0 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
			if attn2 > 0.0 {
				attn2 *= attn2;
				value += attn2 * attn2 * self.extrapolate(xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2);
			}

			//Contribution (0,0,1)
			let dx3 = dx2;
			let dy3 = dy1;
			let dz3 = dz0 - 1.0 - SQUISH_CONSTANT_3D;
			let mut attn3 = 2.0 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
			if attn3 > 0.0 {
				attn3 *= attn3;
				value += attn3 * attn3 * self.extrapolate(xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3);
			}

			//Contribution (1,1,0)
			let dx4 = dx0 - 1.0 - 2.0 * SQUISH_CONSTANT_3D;
			let dy4 = dy0 - 1.0 - 2.0 * SQUISH_CONSTANT_3D;
			let dz4 = dz0 - 0.0 - 2.0 * SQUISH_CONSTANT_3D;
			let mut attn4 = 2.0 - dx4 * dx4 - dy4 * dy4 - dz4 * dz4;
			if attn4 > 0.0 {
				attn4 *= attn4;
				value += attn4 * attn4 * self.extrapolate(xsb + 1, ysb + 1, zsb + 0, dx4, dy4, dz4);
			}

			//Contribution (1,0,1)
			let dx5 = dx4;
			let dy5 = dy0 - 0.0 - 2.0 * SQUISH_CONSTANT_3D;
			let dz5 = dz0 - 1.0 - 2.0 * SQUISH_CONSTANT_3D;
			let mut attn5 = 2.0 - dx5 * dx5 - dy5 * dy5 - dz5 * dz5;
			if attn5 > 0.0 {
				attn5 *= attn5;
				value += attn5 * attn5 * self.extrapolate(xsb + 1, ysb + 0, zsb + 1, dx5, dy5, dz5);
			}

			//Contribution (0,1,1)
			let dx6 = dx0 - 0.0 - 2.0 * SQUISH_CONSTANT_3D;
			let dy6 = dy4;
			let dz6 = dz5;
			let mut attn6 = 2.0 - dx6 * dx6 - dy6 * dy6 - dz6 * dz6;
			if attn6 > 0.0 {
				attn6 *= attn6;
				value += attn6 * attn6 * self.extrapolate(xsb + 0, ysb + 1, zsb + 1, dx6, dy6, dz6);
			}
		}
 
		//First extra vertex
		let mut attn_ext0 = 2.0 - dx_ext0 * dx_ext0 - dy_ext0 * dy_ext0 - dz_ext0 * dz_ext0;
		if attn_ext0 > 0.0 {
			attn_ext0 *= attn_ext0;
			value += attn_ext0 * attn_ext0 * self.extrapolate(xsv_ext0, ysv_ext0, zsv_ext0, dx_ext0, dy_ext0, dz_ext0);
		}

		//Second extra vertex
		let mut attn_ext1 = 2.0 - dx_ext1 * dx_ext1 - dy_ext1 * dy_ext1 - dz_ext1 * dz_ext1;
		if attn_ext1 > 0.0 {
			attn_ext1 *= attn_ext1;
			value += attn_ext1 * attn_ext1 * self.extrapolate(xsv_ext1, ysv_ext1, zsv_ext1, dx_ext1, dy_ext1, dz_ext1);
		}
		
		value / NORM_CONSTANT_3D
	}
	
	fn extrapolate(&self, xsb: i32, ysb: i32, zsb: i32, dx: f64, dy: f64, dz: f64) -> f64 {
		let b_sum = xsb + ysb + zsb;
		let xc = (3 * xsb + b_sum) / 18 / self.w6;
		let yc = (3 * ysb + b_sum) / 18 / self.h6;
		let zc = (3 * zsb + b_sum) / 18 / self.d6;
		
		let xsbm = (-5 * self.w6 * xc) + (self.h6 * yc) + (self.d6 * zc) + xsb;
		let ysbm = (self.w6 * xc) + (-5 * self.h6 * yc) + (self.d6 * zc) + ysb;
		let zsbm = (self.w6 * xc) + (self.h6 * yc) + (-5 * self.d6 * zc) + zsb;
		
		let index = self.perm_grad_index_3d[(self.perm[(self.perm[xsbm as usize & 0xFF] as i32 + ysbm) as usize & 0xFF] as i32 + zsbm) as usize & 0xFF];
		return GRADIENTS_3D[index as usize] as f64 * dx
			+ GRADIENTS_3D[index as usize + 1] as f64 * dy
			+ GRADIENTS_3D[index as usize + 2] as f64 * dz;
	}
}
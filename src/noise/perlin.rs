use ultraviolet::Vec3;
use rand::{rngs::ThreadRng, Rng};


pub struct Perlin {
    point_count: u32,
    random_vectors: Vec<Vec3>,
    permutation_x: Vec<i32>,
    permutation_y: Vec<i32>,
    permutation_z: Vec<i32>,
}

impl Perlin {
    pub fn new (rng: &mut ThreadRng, point_count: u32) -> Perlin {
        let mut result = 
            Perlin{
                point_count, 
                random_vectors: Vec::<Vec3>::new(),
                permutation_x: Vec::<i32>::new(),
                permutation_y: Vec::<i32>::new(),
                permutation_z: Vec::<i32>::new(),
            };

            Perlin::populate_random_vectors(rng, &mut result.random_vectors, result.point_count);
            Perlin::generate_permutation(rng, &mut result.permutation_x, result.point_count);
            Perlin::generate_permutation(rng, &mut result.permutation_y, result.point_count);
            Perlin::generate_permutation(rng, &mut result.permutation_z, result.point_count);

        result
    }

    pub fn turbulence_default(&self, point: &Vec3) -> f32 {
        self.turbulence(point, 7)
    }

    pub fn turbulence(&self, point: &Vec3, depth: i32) -> f32 {
        let mut accumulator = 0.0;
        let mut temp_point = *point;
        let mut weight = 1.0;

        for _level in 0..depth {
            accumulator += weight * Perlin::noise(self, &temp_point);
            weight *= 0.5;
            temp_point *= 2.0;
        }

        f32::abs(accumulator)
    }

    // Check this out to see whether they could all just be usize
    // Also look at improving the triple loop
    pub fn noise(&self, point: &Vec3) -> f32 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut samples = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2i32 {
            for dj in 0..2i32 {
                for dk in 0..2i32 {
                    samples[di as usize][dj as usize][dk as usize] = 
                        self.random_vectors[ 
                            (
                                self.permutation_x[((i + di) & 255) as usize] ^ 
                                self.permutation_y[((j + dj) & 255) as usize] ^
                                self.permutation_z[((k + dk) & 255) as usize]
                            ) as usize ];
                }
            }
        }

        Perlin::perlin_interpolation(&samples, u, v, w)
    }

    fn perlin_interpolation(samples: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut sum = 0.0;

        let mut weight_v = Vec3::zero();

        for i in 0..2usize {
            for j in 0..2usize {
                for k in 0..2usize {
                    weight_v.x = u - i as f32;
                    weight_v.y = v - j as f32;
                    weight_v.z = w - k as f32;

                    let i_f = i as f32;
                    let j_f = j as f32;
                    let k_f = k as f32;

                    sum += 
                        (i_f * uu + (1.0 - i_f) * (1.0 - uu)) *
                        (j_f * vv + (1.0 - j_f) * (1.0 - vv)) *
                        (k_f * ww + (1.0 - k_f) * (1.0 - ww)) *
                        samples[i][j][k].dot(weight_v);
                }
            }
        }

        sum
    }

    fn populate_random_vectors(rng: &mut ThreadRng, vector: &mut Vec<Vec3>, desired_element_count: u32) {
        *vector = (0..desired_element_count).into_iter()
            .map(|_| Vec3::new(
                    rng.gen_range(-1.0..1.0), 
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
            )
            .collect();
    }

    fn generate_permutation(rng: &mut ThreadRng, vector: &mut Vec<i32>, desired_element_count: u32) {
        *vector = (0..(desired_element_count as i32)).into_iter().collect();

        for index in (0..(desired_element_count as usize)).rev() {
            let target = if 0 < index { rng.gen_range(0..index) } else { 0 };
            vector.swap(index, target);
        }
    }

}
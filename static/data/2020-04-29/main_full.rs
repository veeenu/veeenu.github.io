use std::ops::*;

const W: usize = 1280;
const H: usize = 720;

type f = f64;
#[derive(Clone, Copy, Debug)]
struct Vec3(f, f, f);
#[derive(Clone, Copy, Debug)]
struct Mat4(
	f, f, f, f,
	f, f, f, f,
	f, f, f, f,
	f, f, f, f,
);

impl Vec3 {
	fn length(self) -> f {
		(self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
	}

	fn abs(self) -> Vec3 {
		Vec3(self.0.abs(), self.1.abs(), self.2.abs())
	}

	fn normal(self) -> Vec3 {
		let l = self.length();
		Vec3(self.0 / l, self.1 / l, self.2 / l)
	}

	fn cross(self, rhs: Vec3) -> Vec3 {
		Vec3(
			self.1 * rhs.2 - self.2 * rhs.1,
			self.2 * rhs.0 - self.0 * rhs.2,
			self.0 * rhs.1 - self.1 * rhs.0
		)
	}

	fn int(self) -> (u8, u8, u8) {
		((255. * self.0) as u8, (255. * self.1) as u8, (255. * self.2) as u8)
	}

	fn dot(self, rhs: Vec3) -> f {
		(self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2).sqrt()
	}
}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;
	fn add(self, rhs: Vec3) -> Vec3 {
		Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
	}
}

impl Sub<Vec3> for Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: Vec3) -> Vec3 {
		Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
	}
}

impl Mul<f> for Vec3 {
	type Output = Vec3;
	fn mul(self, rhs: f) -> Vec3 {
		Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
	}
}

impl From<(f, f, f)> for Vec3 {
	fn from(i: (f, f, f)) -> Vec3 {
		Vec3(i.0, i.1, i.2)
	}
}

impl Mat4 {
	fn id() -> Mat4 {
		Mat4(
			1., 0., 0., 0.,
			0., 1., 0., 0.,
			0., 0., 1., 0.,
			0., 0., 0., 1.,
		)
	}

	fn mul(self, rhs: Vec3) -> Vec3 {
		let Vec3(x, y, z) = rhs;
		let mut w = self.3 * x + self.7 * y + self.11 * z + self.15;
		if w == 0. { w = 1. };

		Vec3(
			(self.0 * x + self.4 * y + self.8 * z + self.12) / w,
			(self.1 * x + self.5 * y + self.9 * z + self.13) / w,
			(self.2 * x + self.6 * y + self.10 * z + self.14) / w,
		)
	}

	fn from_roll(roll: f) -> Mat4 {
		let (s, c) = (roll.sin(), roll.cos());

		Mat4(
			1., 0., 0., 0.,
			0.,  c,  s, 0.,
			0., -s,  c, 0.,
			0., 0., 0., 1.
		)
	}

	fn from_pitch(pitch: f) -> Mat4 {
		let (s, c) = (pitch.sin(), pitch.cos());

		Mat4(
			c, 0., -s, 0.,
			0., 1., 0., 0.,
			s, 0.,  c, 0.,
			0., 0., 0., 1.
		)
	}

	fn from_yaw(yaw: f) -> Mat4 {
		let (s, c) = (yaw.sin(), yaw.cos());

		Mat4(
			c,  s, 0., 0.,
			-s,  c, 0., 0.,
			0., 0., 1., 0.,
			0., 0., 0., 1.
		)
	}

	fn from_rpy(roll_pitch_yaw: Vec3) -> Mat4 {
		let Vec3(roll, pitch, yaw) = roll_pitch_yaw;
		let m_roll = Mat4::from_roll(roll);
		let m_pitch = Mat4::from_pitch(pitch);
		let m_yaw = Mat4::from_yaw(yaw);

		m_yaw.mmul(m_pitch).mmul(m_roll)

		// https://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
	}

	fn mmul(self, rhs: Mat4) -> Mat4 {
		let Mat4(
			a00, a01, a02, a03,
			a10, a11, a12, a13,
			a20, a21, a22, a23,
			a30, a31, a32, a33,
		) = self;

		let Mat4(
			b00, b01, b02, b03,
			b10, b11, b12, b13,
			b20, b21, b22, b23,
			b30, b31, b32, b33,
		) = rhs;

		Mat4(
			b00 * a00 + b01 * a10 + b02 * a20 + b03 * a30,
			b00 * a01 + b01 * a11 + b02 * a21 + b03 * a31,
			b00 * a02 + b01 * a12 + b02 * a22 + b03 * a32,
			b00 * a03 + b01 * a13 + b02 * a23 + b03 * a33,

			b10 * a00 + b11 * a10 + b12 * a20 + b13 * a30,
			b10 * a01 + b11 * a11 + b12 * a21 + b13 * a31,
			b10 * a02 + b11 * a12 + b12 * a22 + b13 * a32,
			b10 * a03 + b11 * a13 + b12 * a23 + b13 * a33,

			b20 * a00 + b21 * a10 + b22 * a20 + b23 * a30,
			b20 * a01 + b21 * a11 + b22 * a21 + b23 * a31,
			b20 * a02 + b21 * a12 + b22 * a22 + b23 * a32,
			b20 * a03 + b21 * a13 + b22 * a23 + b23 * a33,

			b30 * a00 + b31 * a10 + b32 * a20 + b33 * a30,
			b30 * a01 + b31 * a11 + b32 * a21 + b33 * a31,
			b30 * a02 + b31 * a12 + b32 * a22 + b33 * a32,
			b30 * a03 + b31 * a13 + b32 * a23 + b33 * a33,
		)
	}

	fn rotate(self, rhs: Vec3) -> Mat4 {
		self.mmul(Mat4::from_rpy(rhs))
	}

	fn translate(self, rhs: Vec3) -> Mat4 {
		let Mat4(
			a00, a01, a02, a03,
			a10, a11, a12, a13,
			a20, a21, a22, a23,
			a30, a31, a32, a33,
		) = self;
		let Vec3(x, y, z) = rhs; // Z and Y inverted because column major

		Mat4(
			a00, a01, a02, a03,
			a10, a11, a12, a13,
			a20, a21, a22, a23,

			a00 * x + a10 * y + a20 * z + a30,
			a01 * x + a11 * y + a21 * z + a31,
			a02 * x + a12 * y + a22 * z + a32,
			a03 * x + a13 * y + a23 * z + a33,
		)
	}

	fn inv(self) -> Mat4 {
		let Mat4(
			a00, a01, a02, a03,
			a10, a11, a12, a13,
			a20, a21, a22, a23,
			a30, a31, a32, a33,
		) = self;
		
		let b00 = a00 * a11 - a01 * a10;
		let b01 = a00 * a12 - a02 * a10;
		let b02 = a00 * a13 - a03 * a10;
		let b03 = a01 * a12 - a02 * a11;
		let b04 = a01 * a13 - a03 * a11;
		let b05 = a02 * a13 - a03 * a12;
		let b06 = a20 * a31 - a21 * a30;
		let b07 = a20 * a32 - a22 * a30;
		let b08 = a20 * a33 - a23 * a30;
		let b09 = a21 * a32 - a22 * a31;
		let b10 = a21 * a33 - a23 * a31;
		let b11 = a22 * a33 - a23 * a32;

		// Big assumption of non-null determinant
		let det = 1. / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);

		Mat4(
			(a11 * b11 - a12 * b10 + a13 * b09) * det,
			(a02 * b10 - a01 * b11 - a03 * b09) * det,
			(a31 * b05 - a32 * b04 + a33 * b03) * det,
			(a22 * b04 - a21 * b05 - a23 * b03) * det,

			(a12 * b08 - a10 * b11 - a13 * b07) * det,
			(a00 * b11 - a02 * b08 + a03 * b07) * det,
			(a32 * b02 - a30 * b05 - a33 * b01) * det,
			(a20 * b05 - a22 * b02 + a23 * b01) * det,
			
			(a10 * b10 - a11 * b08 + a13 * b06) * det,
			(a01 * b08 - a00 * b10 - a03 * b06) * det,
			(a30 * b04 - a31 * b02 + a33 * b00) * det,
			(a21 * b02 - a20 * b04 - a23 * b00) * det,
			
			(a11 * b07 - a10 * b09 - a12 * b06) * det,
			(a00 * b09 - a01 * b07 + a02 * b06) * det,
			(a31 * b01 - a30 * b03 - a32 * b00) * det,
			(a20 * b03 - a21 * b01 + a22 * b00) * det,
		)
	}
}

fn rad(deg: f) -> f {
	deg * std::f64::consts::PI / 180.
}

fn max(lhs: Vec3, rhs: Vec3) -> Vec3 {
	Vec3(
		f::max(lhs.0, rhs.0),
		f::max(lhs.1, rhs.1),
		f::max(lhs.2, rhs.2),
	)
}

///
/// Eye ray transform
///

fn eye_ray_transform(ro: Vec3, ta: Vec3, roll: f) -> Mat4 {
	let ww = (ta - ro).normal();
	let uu = (ww.cross(Vec3(roll.sin(), roll.cos(), 0.))).normal();
	let vv = uu.cross(ww).normal();
	Mat4(
		uu.0, vv.0, ww.0, 0.,
		uu.1, vv.1, ww.1, 0.,
		uu.2, vv.2, ww.2, 0.,
		0., 0., 0., 1.
	)
}

///
/// SDFs
///

fn sdf_sphere(ray: Vec3, pos: Vec3, rad: f) -> f {
	let dist = (pos - ray).length();
	dist - rad
}

fn sdf_box(ray: Vec3, pos: Vec3, size: Vec3) -> f {
	let adj = ray - pos;
	let dist_vec = adj.abs() - size;
	let max_dist = f::max(dist_vec.0, f::max(dist_vec.1, dist_vec.2));
	let box_dist = f::min(max_dist, 0.) + max(dist_vec, Vec3(0., 0., 0.)).length();

	box_dist
}

fn sdf_octah(ray: Vec3, pos: Vec3, s: f) -> f {
	let p = (pos - ray).abs();
	(p.0 + p.1 + p.2 - s) * 0.57735027
}

fn sdf_cyl(ray: Vec3, h: f, r: f) -> f {
	let p = ray.abs();
	let x = (p.0 * p.0 + p.2 * p.2).sqrt().abs() - h;
	let y = p.1 - r;
	let (cx, cy) = (f::max(x, 0.), f::max(y, 0.));
	f::min(f::max(x, y), 0.) + (cx * cx + cy * cy).sqrt()
}

///
/// Distance operations
///

fn clamp(a: f, min: f, max: f) -> f {
	f::max(min, f::min(max, a))
}

fn lerp(a: f, b: f, t: f) -> f {
	a * t + b * (1. - t)
}

fn op_union(a: f, b: f) -> f {
	f::min(a, b)
}

fn op_subtraction(src: f, dest: f) -> f {
	f::max(-src, dest)
}

fn op_intersection(src: f, dest: f) -> f {
	f::max(src, dest)
}

fn op_union_smooth(d1: f, d2: f, k: f) -> f {
	let h = clamp(0.5 + 0.5 * (d2 - d1) / k, 0., 1.);
	lerp(d1, d2, h) - k * h * (1. - h)
}

fn op_subtract_smooth(d1: f, d2: f, k: f) -> f {
	let h = clamp(0.5 - 0.5 * (d2 + d1) / k, 0., 1.);
	lerp(-d1, d2, h) + k * h * (1. - h)
}

fn op_intersect_smooth(d1: f, d2: f, k: f) -> f {
	let h = clamp(0.5 - 0.5 * (d2 - d1) / k, 0., 1.);
	lerp(d1, d2, h) + k * h * (1. - h)
}

//
//
//

#[derive(Debug)]
struct SceneMatrices {
	v: (Mat4, Mat4, Mat4, Mat4),
	e: (Mat4, Mat4, Mat4),
	n: (Mat4,),
	u: (Mat4, Mat4,),
}

/*fn build_scene_matrices() -> SceneMatrices {
	SceneMatrices {
		v: (
			Mat4::id().rotate(Vec3(0., 0., rad(60.))).translate(Vec3(1., 0., 0.)).inv(),
			Mat4::id().rotate(Vec3(0., 0., rad(120.))).translate(Vec3(1., 0., 0.)).inv(),
			Mat4::id().rotate(Vec3(rad(90.), 0., 0.)).translate(Vec3(0., 0., 0.)).inv(),
			Mat4::id().translate(Vec3(-1.5, -0.5, 0.)).inv(),
		),
		e: (
			Mat4::id().rotate(Vec3(rad(90.), 0., 0.)).inv(),
			Mat4::id().translate(Vec3(0.5, 0., 0.2)).rotate(Vec3(0., rad(45.), 0.)).inv(),
			Mat4::id().translate(Vec3(0., 0., 0.)).inv(),
		),
		n: (
			Mat4::id().translate(Vec3(1.5, 0., 0.)).inv(),
		),
		u: (
			Mat4::id().rotate(Vec3(rad(180.), 0., 0.)).inv(),
			Mat4::id().translate(Vec3(3., 0., 0.)).inv(),
		)
	}
}*/

///
/// Get closest element in scene
///

fn find_distance(
	ray: Vec3
)  -> (f, usize) {
	const DEPTH: f = 0.5;
	const S45: f = 0.7071067811865475;
	const S60: f = 0.8660254037844386;
	const SM: SceneMatrices = SceneMatrices { 
		v: (
			Mat4(0.5, -S60, 0., 0., S60, 0.5, 0., 0., 0., 0., 1., 0., -1., 0., 0., 1.),
			Mat4(-0.5, -S60, 0., 0., S60, -0.5, 0., 0., 0., 0., 1., 0., -1., 0., 0., 1.), 
			Mat4(1., 0., 0., 0., 0., 0., -1., 0., 0., 1., 0., -0., 0., 0., 0., 1.), 
			Mat4(1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 1.5, 0.5, 0., 1.)
		), 
		e: (
			Mat4(1., 0., 0., 0., 0., 0., -1., 0., 0., 1., 0., -0., 0., 0., 0., 1.), 
			Mat4(S45, -0., S45, 0., 0., 1., 0., 0., -S45, 0., S45, 0., -0.2121320343559642, 0., -0.5, 1.), 
			Mat4(1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.)
		),
		n: (
			Mat4(1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., -1.5, 0., 0., 1.),
		), 
		u: (
			Mat4(1., 0., 0., 0., 0., -1., -0., 0., 0., 0., -1., 0., -0., 0., 0., 1.), 
			Mat4(1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., -3., 0., 0., 1.)
		)
	};

	let (v0m, v1m, v2m, v_tr) = SM.v;
	let (e0m, e1m, e_tr) = SM.e;
	let (n_tr,) = SM.n;
	let (u0m, u_tr) = SM.u;

	let v_ray = v_tr.mul(ray);

	let v0 = sdf_box(v0m.mul(v_ray), Vec3(0., 0., 0.), Vec3(1., 0.1, DEPTH));
	let v1 = sdf_box(v1m.mul(v_ray), Vec3(0., 0., 0.), Vec3(1., 0.1, DEPTH));
	let v2 = sdf_cyl(v2m.mul(v_ray), 0.1, DEPTH);

	let v = op_union(op_union(v0, v1), v2);

	let e0_ray = e_tr.mul(e0m.mul(ray));
	let e1_ray = e1m.mul(e0_ray);
	let e = op_subtraction(sdf_cyl(e0_ray, 0.4, DEPTH * 2.), sdf_cyl(e0_ray, 0.6, DEPTH));
	let e_half_box = op_subtraction(
		sdf_box(e0_ray, Vec3(0.5, 0., -0.1), Vec3(0.5, DEPTH * 2., 0.2)),
		sdf_box(e1_ray, Vec3(0., 0., 0.), Vec3(0.2, DEPTH * 2., 0.2)),
	);
	let e = op_subtraction(e_half_box, e);
	let e = op_union(e, sdf_box(e0_ray, Vec3(0., 0., 0.), Vec3(0.5, DEPTH, 0.1)));

	let n_ray = n_tr.mul(e0_ray);

	let n = op_subtraction(sdf_cyl(n_ray, 0.4, DEPTH * 2.), sdf_cyl(n_ray, 0.6, DEPTH));
	let n = op_subtraction(sdf_box(n_ray, Vec3(0., 0., 0.6), Vec3(0.6, DEPTH * 2., 0.6)), n);
	let n = op_union(sdf_box(n_ray, Vec3(-0.5, 0., 0.), Vec3(0.1, DEPTH, 0.6)), n);
	let n = op_union(sdf_box(n_ray, Vec3(0.5, 0., 0.3), Vec3(0.1, DEPTH, 0.3)), n);

	let u_ray = u_tr.mul(u0m.mul(e0_ray));

	let u = op_subtraction(sdf_cyl(u_ray, 0.4, DEPTH * 2.), sdf_cyl(u_ray, 0.6, DEPTH));
	let u = op_subtraction(sdf_box(u_ray, Vec3(0., 0., 0.6), Vec3(0.6, DEPTH * 2., 0.6)), u);
	let u = op_union(sdf_box(u_ray, Vec3(0.5, 0., 0.), Vec3(0.1, DEPTH, 0.6)), u);
	let u = op_union(sdf_box(u_ray, Vec3(-0.5, 0., 0.3), Vec3(0.1, DEPTH, 0.3)), u);

	[(v, 1), (e, 2), (n, 3), (u, 4)]
		.iter()
		.fold((6., 0), |t, o| {
			if t.0 <= o.0 {
				t
			} else {
				*o
			}
		})
}

///
/// Raymarch and calculate
///

fn calc_intersection(eye_pos: Vec3, ray_dir: Vec3) -> (f, usize) {
	const MARCH_ITERS: usize = 100;
	const INTERS_PRECISION: f = 0.001;
	const MAX_DIST: f = 30.;
	let mut dist_to_surface: f = INTERS_PRECISION * 2.;
	let mut total_dist_traveled: f = 0.;
	let mut target: (f, usize) = (-1., 0);

	for _ in 0..MARCH_ITERS {
		if dist_to_surface < INTERS_PRECISION ||
			 total_dist_traveled > MAX_DIST {
			break;
		}

		let curr_ray_pos = eye_pos + ray_dir * total_dist_traveled;
		target = find_distance(curr_ray_pos);
		dist_to_surface = target.0;
		total_dist_traveled += target.0;
	}

	if total_dist_traveled < MAX_DIST {
		(total_dist_traveled, target.1)
	} else {
		(MAX_DIST, 0)
	}
}

///
/// Calculate normal to a point
///

fn calc_normal(hit: Vec3) -> Vec3 {
	let dx = Vec3(0.001, 0., 0.);
	let dy = Vec3(0., 0.001, 0.);
	let dz = Vec3(0., 0., 0.001);

	let dx = find_distance(hit + dx).0 - find_distance(hit - dx).0;
	let dy = find_distance(hit + dy).0 - find_distance(hit - dy).0;
	let dz = find_distance(hit + dz).0 - find_distance(hit - dz).0;
	Vec3(dx, dy, dz).normal()
}

///
/// Calculate normal to a point
///

fn calc_light(hit: Vec3, normal: Vec3) -> f {
	const LIGHT_POS: Vec3 = Vec3(4., 2., 2.);
	let light_dir = (LIGHT_POS - hit).normal();
	f::max(0., light_dir.dot(normal))
}

///
/// Render a pixel
///

fn render_pixel(x: f, y: f) -> Vec3 {
	let pos: (f, f) = (
		(-(W as f) + 2. * x) / (H as f),
		(-(H as f) + 2. * y) / (H as f)
	);
	let ray_origin = Vec3(3., 2., 4.);
	let ray_target = Vec3(1.25, 0., 0.);
	let ert = eye_ray_transform(ray_origin, ray_target, 0.);
	let ray_dir = ert.mul(Vec3(pos.0, pos.1, 2.)).normal();
	let target = calc_intersection(ray_origin, ray_dir);

	let hit_pos = ray_origin + (ray_dir * target.0);
	let nv = calc_normal(hit_pos);
	let lit = f::max(0.1, calc_light(hit_pos, nv)) / 255.;

	match target.1 {
		1 => Vec3(6., 81., 67.) * lit,
		2 => Vec3(85., 111., 68.) * lit,
		3 => Vec3(101., 155., 94.) * lit,
		4 => Vec3(149., 191., 116.) * lit,
		_ => Vec3(0., 0., 0.),
	}
}

//
// Call render for each pixel and print the PPM
//

fn main() {
	print!("P3 {} {} 255 ", W, H);
	for y in (1..=H).rev() {
		for x in 0..W {
			let px = render_pixel(x as _, y as _).int();
			print!("{} {} {} ", px.0, px.1, px.2);
		}
	}
}

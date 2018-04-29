use game::camera::Camera;
use cgmath::{vec3, Vector2, Vector3};
use color::*;

pub struct Tunel {
    // Size of tunel, x, y, z is the width, height, and deep of the
    // tunel.
    pub size: Vector3<f64>,
    // Color to draw tunel
    pub color: Color,
    // Road's dividers
    pub divider: Vector2<f64>,
    pub divider_state: f64,
    // Tunel's decorations
    pub decor_distance: f64,
    pub decor_state: f64,
}

impl Tunel {
    pub fn new(config: &::game::GameConfig) -> Tunel {
        Tunel {
            size: config.tunel_size.into(),
            color: BLUE,
            divider: config.divider_size.into(),
            divider_state: config.divider_size[1],
            decor_distance: config.decor_distance,
            decor_state: config.decor_distance,
        }
    }

    pub fn render(&self, camera: &Camera) -> Vec<([Vector2<f64>; 2], Color)> {
        vec![
            ((0., 0., 0.), (0., 0., self.size.z)),
            ((self.size.x, 0., 0.), (self.size.x, 0., self.size.z)),
            ((0., self.size.y, 0.), (0., self.size.y, self.size.z)),
            (
                (self.size.x, self.size.y, 0.),
                (self.size.x, self.size.y, self.size.z),
            ),
        ].into_iter()
            .map(|(a, b)| camera.render_line(&a.into(), &b.into()))
            .filter_map(|x| x.map(|x| (x, self.color)))
            .chain(self.divider_render(camera))
            .chain(self.decor_render(camera))
            .collect()
    }

    pub fn update(&mut self, dt: f64, speed: f64) {
        self.divider_state -= dt * speed;
        if self.divider_state < 0. {
            self.divider_state += 2. * self.divider.y;
        }
        self.decor_state -= dt * speed;
        if self.decor_state < 0. {
            self.decor_state += self.decor_distance;
        }
    }

    fn divider_render(&self, camera: &Camera) -> ::Rendered {
        let mut points = [vec3(self.size.x / 2., 0., self.divider_state); 4];
        points[2].z -= self.divider.y;
        points[3].z -= self.divider.y;
        points[0].x -= self.divider.x / 2.;
        points[3].x -= self.divider.x / 2.;
        points[1].x += self.divider.x / 2.;
        points[2].x += self.divider.x / 2.;

        let mut ret = Vec::new();
        {
            let mut r = |p: &[Vector3<f64>; 4]| {
                let iter = p.iter()
                    .zip(p.iter().cycle().skip(1))
                    .map(|(x, y)| camera.render_line(x, y))
                    .filter_map(|x| x.map(|x| (x, self.color)));
                ret.append(&mut iter.collect());
            };
            while points[0].z <= self.size.z {
                r(&points);
                for p in &mut points {
                    p.z += 2. * self.divider.y;
                }
            }
            r(&points);
        }
        ret
    }
    fn decor_render(&self, camera: &Camera) -> ::Rendered {
        let mut data = [
            vec3(0., 0., self.decor_state),
            vec3(0., self.size.y, self.decor_state),
            vec3(self.size.x, self.size.y, self.decor_state),
            vec3(self.size.x, 0., self.decor_state),
        ];
        let mut ret = Vec::new();
        while data[0].z <= self.size.z {
            for (x, y) in data.iter().zip(data.iter().skip(1)) {
                if let Some(rendered) = camera.render_line(x, y) {
                    ret.push((rendered, self.color));
                }
            }
            for x in &mut data {
                x.z += self.decor_distance;
            }
        }
        ret
    }
}

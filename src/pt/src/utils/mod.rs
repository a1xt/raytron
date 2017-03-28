pub mod consts;



pub mod sample_surfaces {

    use std::f32::consts::PI;
    use traits::{Surface};
    use {SurfacePoint};
    use math::{self, Point3f, Coord};
    use std::borrow::BorrowMut;
    use color;

    use rand::{self, Closed01};

    /// @return (point at surface, pdf)
    pub fn by_area<'s, T>(mut surfaces: T, view_point: &Point3f) -> Option<(SurfacePoint<'s>, Coord)>
        where T: IntoIterator<Item = &'s Surface, IntoIter = Box<Iterator<Item = &'s Surface> + 's>> + Clone + 's
    {
        // let mut total_area = 0.0;
        // for s in surfaces.clone().into_iter() {
        //     total_area += s.area();
        // }

        // let Closed01(e0) = rand::random::<Closed01<Coord>>();
        // let e = e0 * total_area;

        // let mut prev_area = 0.0;
        // let mut res = None;
        // for s in surfaces.clone().into_iter() {
        //     if s.area() + prev_area > e {
        //         let (sp, pdf) = s.sample_surface(view_point);
        //         let pdf_res = (s.area() / total_area) * pdf;

        //         res = Some((sp, pdf_res));
        //         break;
        //     }
        //     prev_area += s.area();
        // }

        // res

        let s_num = surfaces.clone().into_iter().count();

        if s_num > 0 {
            let i = rand::random::<usize>() % s_num;
            let s = surfaces.into_iter().nth(i).unwrap();

            let (sp, pdf) = s.sample_surface(view_point);

            Some((sp, pdf / s_num as Coord))
        } else {
            None
        }

    }

    pub fn by_area_pdf<'s, T>(surface: &'s Surface, mut scene_surfaces: T, point_at_surface: &Point3f, view_point: &Point3f) -> Coord
        where T: IntoIterator<Item = &'s Surface, IntoIter = Box<Iterator<Item = &'s Surface> + 's>> + Clone + 's
    {
        let s_num = scene_surfaces.into_iter().count();
        let pdf = surface.pdf(point_at_surface, view_point);

        pdf / s_num as Coord
    }

    pub fn illumination<'s, T>(mut surfaces: T, view_point: &Point3f) -> Option<(SurfacePoint<'s>, Coord)>
        where T: IntoIterator<Item = &'s Surface, IntoIter = Box<Iterator<Item = &'s Surface> + 's>> + Clone + 's
    {
        // let mut total_illumination = 0.0;
        // for s in surfaces.clone().into_iter() {
        //     total_illumination += color::rgb_to_illumination(&s.total_emittance().unwrap()) as Coord;
        // }

        // let Closed01(e0) = rand::random::<Closed01<Coord>>();
        // let e = e0 * total_illumination;

        // let mut prev_ill = 0.0;
        // let mut res = None;
        // for s in surfaces.clone().into_iter() {
        //     let ill = color::rgb_to_illumination(&s.total_emittance().unwrap()) as Coord;
        //     if ill + prev_ill > e {
        //         let (sp, pdf) = s.sample_surface(view_point);            
        //         let pdf_res = (ill / total_illumination) * pdf;

        //         res = Some((sp, pdf_res));
        //         break;
        //     }
        //     prev_ill += ill;
        // }

        // res
        by_area(surfaces, view_point)
    }
}


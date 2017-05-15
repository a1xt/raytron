pub mod consts;

pub mod sample_surfaces {

    use traits::{Surface};
    use {SurfacePoint};
    use math::{Point3f, Vector3f, Real};

    use rand::{self, Closed01};

    /// @return (point at surface, pdf)
    pub fn by_area<'s, 'p: 's, T, F>(surfaces: T, view_point: (&'p Point3f, &'p Vector3f), sample_fn: F) -> Option<(SurfacePoint<'s>, Real)>
        where T: IntoIterator<Item = &'s Surface, IntoIter = Box<Iterator<Item = &'s Surface> + 's>> + Clone + 's,
              F: Fn(&'s Surface, (&'p Point3f, &'p Vector3f)) -> (SurfacePoint<'s>, Real),
    {
        // let mut total_area = 0.0;
        // for s in surfaces.clone().into_iter() {
        //     total_area += s.area();
        // }

        // let Closed01(e0) = rand::random::<Closed01<Real>>();
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

            //let (sp, pdf) = s.sample_surface_p(view_point);
            let (sp, pdf) = sample_fn(s, view_point);

            Some((sp, pdf / s_num as Real))
        } else {
            None
        }

    }

    pub fn by_area_pdf<'s, 'p: 's, T, F>(surface: &'s Surface,
                              scene_surfaces: T,
                              point_at_surface: (&'p Point3f, &'p Vector3f), 
                              view_point: (&'p Point3f, &'p Vector3f),
                              pdf_fn: F) 
                              -> Real
        where T: IntoIterator<Item = &'s Surface, IntoIter = Box<Iterator<Item = &'s Surface> + 's>> + Clone + 's,
              F: Fn(&'s Surface, (&'p Point3f, &'p Vector3f), (&'p Point3f, &'p Vector3f)) -> Real,

    {
        let s_num = scene_surfaces.into_iter().count();
        //let pdf = surface.pdf_p(point_at_surface, view_point);
        let pdf = pdf_fn(surface, point_at_surface, view_point);

        pdf / s_num as Real
    }

}


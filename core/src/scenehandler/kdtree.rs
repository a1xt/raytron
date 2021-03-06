use super::{LightSourcesHandler, LuminairesSampler, UniformSampler};
use SurfacePoint;
use aabb::{intersection_aabb, Aabb3};
use math::{Ray3f, Real};
use num::Float;
use std::cmp::max;
use std::sync::Arc;
use traits::{BoundedSurface, HasBounds, SceneHandler, Surface};
use utils::consts;

pub const KDTREE_DEPTH_MAX: usize = 512;

#[derive(Copy, Clone, Debug)]
pub struct KdTreeSetup {
    pub splits_num: usize,
    pub sah: Sah,
    pub max_depth: usize,
}

impl KdTreeSetup {
    pub fn new(splits_num: usize, max_depth: usize, sah: Sah) -> Self {
        Self {
            splits_num,
            sah,
            max_depth,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Sah {
    pub cost_t: Real,
    pub cost_i: Real,
}

impl Sah {
    pub fn new(cost_traverse: Real, cost_intersection: Real) -> Self {
        Self {
            cost_t: cost_traverse,
            cost_i: cost_intersection,
        }
    }

    pub fn eval(
        &self,
        (bbox0, num0): (&Aabb3, usize),
        (bbox1, num1): (&Aabb3, usize),
        bbox: &Aabb3,
    ) -> Real {
        self.cost_t +
            self.cost_i * (bbox0.volume() * num0 as Real + bbox1.volume() * num1 as Real) /
                bbox.volume()
    }

    pub fn eval_short(&self, (l0, num0): (Real, usize), (l1, num1): (Real, usize)) -> Real {
        l0 * num0 as Real + l1 * num1 as Real
    }
}

pub struct KdTree<'a, T>
where
    T: HasBounds + ?Sized + 'a,
{
    head: Node<'a, T>,
    depth: usize,
    bbox: Aabb3,
}

impl<'a, T> KdTree<'a, T>
where
    T: HasBounds + ?Sized + 'a,
{
    pub fn build(objs: Vec<(Aabb3, &'a T)>, setup: KdTreeSetup) -> Self {
        let mut bbox = objs[0].0;
        for &(ref aabb, _) in &objs {
            bbox.merge(aabb);
        }


        let (head, depth) = Node::build(objs, &setup, &bbox, 1);
        // let head = Node::build_median(objs, &bbox, 0, setup.max_depth);
        // let depth = setup.max_depth;

        KdTree { head, depth, bbox }
    }

    pub fn traverse_iter(&self, ray: &Ray3f) -> TraverseIter<T> {
        if let Some(t) = intersection_aabb(&self.bbox, ray) {
            TraverseIter {
                ray: *ray,
                nodes: {
                    let mut v = Vec::with_capacity(self.depth + 1);
                    v.push((&self.head, t));
                    v
                },
            }
        } else {
            TraverseIter {
                ray: *ray,
                nodes: Vec::new(),
            }
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

#[derive(Copy, Clone, Debug)]
struct NodeData {
    //pub bbox: Aabb3,
    pub split: (usize, Real),
}

impl NodeData {
    pub fn new(_: Aabb3, split_plane: (usize, Real)) -> NodeData {
        assert!(split_plane.0 < 3);
        Self {
            //bbox: aabb,
            split: split_plane,
        }
    }
}

enum Node<'a, T>
where
    T: HasBounds + ?Sized + 'a,
{
    Tree(Box<Node<'a, T>>, Box<Node<'a, T>>, NodeData),
    Leaf(Vec<&'a T>),
}

impl<'a, T> Node<'a, T>
where
    T: HasBounds + ?Sized + 'a,
{
    pub fn build(
        objs: Vec<(Aabb3, &'a T)>,
        setup: &KdTreeSetup,
        self_bbox: &Aabb3,
        depth: usize,
    ) -> (Self, usize) {
        let splits_num = setup.splits_num;
        let sah = &setup.sah;
        let mut split = (
            Real::max_value(),
            (0, 0.0),
            (*self_bbox, 0),
            (*self_bbox, 0),
        );
        for i in 0..3 {
            // search split plane
            let mut bins_l = Vec::with_capacity(splits_num);
            let mut bins_h = Vec::with_capacity(splits_num);
            for _ in 0..splits_num {
                bins_l.push(0);
                bins_h.push(0);
            }

            let pos_min = self_bbox.mins()[i];
            let pos_max = self_bbox.maxs()[i];
            let pos_step = (pos_max - pos_min) / ((splits_num + 1) as Real);
            if pos_step < consts::REAL_EPSILON {
                continue;
            }
            for &(ref aabb, _) in &objs {
                let ix_l: Real = (aabb.mins()[i] - pos_min) / pos_step;
                let ix_h: Real = splits_num as Real - ((pos_max - aabb.maxs()[i]) / pos_step);

                let il = if ix_l < 0.0 {
                    0usize
                } else if ix_l >= splits_num as Real {
                    splits_num - 1
                } else {
                    ix_l as usize
                };

                let ih = if ix_h < 0.0 {
                    0usize
                } else if ix_h >= splits_num as Real {
                    splits_num - 1
                } else {
                    ix_h as usize
                };

                bins_l[il] += 1;
                bins_h[ih] += 1;
            }

            for n in 1..splits_num {
                bins_l[n] += bins_l[n - 1];
                bins_h[splits_num - n - 1] += bins_h[splits_num - n];
            }
            // find min sah
            let (_, split_pos, bin_ix) = (0..splits_num)
                .map(|i| {
                    let x_left = pos_step * (i + 1) as Real;
                    let x_right = (pos_max - pos_min) - x_left;
                    let sah_i_short = sah.eval_short((x_left, bins_l[i]), (x_right, bins_h[i]));
                    (sah_i_short, x_left + pos_min, i)
                })
                .min_by(|&(ref sah0, _, _), &(ref sah1, _, _)| {
                    sah0.partial_cmp(sah1).unwrap()
                })
                .unwrap();

            // calculate full sah_i
            let mut split_pos_left = *self_bbox.maxs();
            split_pos_left[i] = split_pos;
            let mut split_pos_right = *self_bbox.mins();
            split_pos_right[i] = split_pos;

            let bbox_left = Aabb3::new(*self_bbox.mins(), split_pos_left);
            let bbox_right = Aabb3::new(split_pos_right, *self_bbox.maxs());
            let n_left = bins_l[bin_ix];
            let n_right = bins_h[bin_ix];
            let sah_i = sah.eval((&bbox_left, n_left), (&bbox_right, n_right), self_bbox);

            if i == 0 || sah_i < split.0 {
                split = (
                    sah_i,
                    (i, split_pos),
                    (bbox_left, n_left),
                    (bbox_right, n_right),
                );
            }
        }

        // build node
        let (sah_min, split_plane, (bbox_left, n_left), (bbox_right, n_right)) = split;
        if sah_min < setup.sah.cost_i * (objs.len() as Real) {
            let mut objs_l = Vec::with_capacity(n_left);
            let mut objs_r = Vec::with_capacity(n_right);
            for &(ref bbox, obj) in &objs {
                if bbox_left.intersects(bbox) {
                    objs_l.push((*bbox, obj));
                }
                if bbox_right.intersects(bbox) {
                    objs_r.push((*bbox, obj));
                }
            }
            let (node_left, depth_left) = Self::build(objs_l, setup, &bbox_left, depth + 1);
            let (node_right, depth_right) = Self::build(objs_r, setup, &bbox_right, depth + 1);

            (
                Node::Tree(
                    box node_left,
                    box node_right,
                    NodeData::new(*self_bbox, split_plane),
                ),
                max(depth_left, depth_right),
            )

        } else {

            if objs.len() > 200 {
                println!("kd-tree warning: num of items in leaf: {}", objs.len());
                println!("    leaf items num: {}", objs.len());
                println!("    min sah: {}", sah_min);
                println!(
                    "    parent sah: {}",
                    setup.sah.cost_i * (objs.len() as Real)
                );
                println!("    n_left: {}, area: {}", n_left, bbox_left.volume());
                println!("    n_right: {}, area: {}", n_right, bbox_right.volume());
                println!("    parent area: {}", self_bbox.volume());

            }

            let mut leaf_objs = Vec::with_capacity(objs.len());
            for &(_, o) in &objs {
                leaf_objs.push(o);
            }

            (Node::Leaf(leaf_objs), depth)
        }


    }

    #[allow(dead_code)]
    pub fn build_median(
        objs: Vec<(Aabb3, &'a T)>,
        self_bbox: &Aabb3,
        depth: usize,
        max_depth: usize,
    ) -> Self {
        if depth < max_depth && objs.len() > 5 {
            let (axis, _) = (0..3)
                .map(|i| (i, self_bbox.maxs()[i] - self_bbox.mins()[i]))
                .max_by(|&(_, l0), &(_, l1)| l0.partial_cmp(&l1).unwrap())
                .unwrap();
            let split_pos = 0.5 * (self_bbox.maxs()[axis] + self_bbox.mins()[axis]);

            // node
            let mut split_pos_left = *self_bbox.maxs();
            split_pos_left[axis] = split_pos;
            let bbox_left = Aabb3::new(*self_bbox.mins(), split_pos_left);
            let mut split_pos_right = *self_bbox.mins();
            split_pos_right[axis] = split_pos;
            let bbox_right = Aabb3::new(split_pos_right, *self_bbox.maxs());
            let mut objs_l = Vec::new();
            let mut objs_r = Vec::new();
            for &(ref bbox, o) in &objs {
                if bbox_left.intersects(bbox) {
                    objs_l.push((*bbox, o));
                }
                if bbox_right.intersects(bbox) {
                    objs_r.push((*bbox, o));
                }
            }

            let node_left = box Self::build_median(objs_l, &bbox_left, depth + 1, max_depth);
            let node_right = box Self::build_median(objs_r, &bbox_right, depth + 1, max_depth);

            Node::Tree(
                node_left,
                node_right,
                NodeData::new(*self_bbox, (axis, split_pos)),
            )

        } else {
            // leaf
            let mut leaf_objs = Vec::with_capacity(objs.len());
            for &(_, o) in &objs {
                leaf_objs.push(o);
            }
            if leaf_objs.len() > 100 {
                println!(" -- leaf objs num: {}", leaf_objs.len());
            }
            Node::Leaf(leaf_objs)
        }
    }
}

#[derive(Clone)]
pub struct TraverseIter<'a, T: HasBounds + ?Sized + 'a> {
    ray: Ray3f,
    nodes: Vec<(&'a Node<'a, T>, (Real, Real))>,
}

impl<'a, T: HasBounds + ?Sized + 'a> Iterator for TraverseIter<'a, T> {
    type Item = (&'a Vec<&'a T>, Real, Real);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, (t_min, t_max))) = self.nodes.pop() {
            match *node {
                Node::Tree(ref left_node, ref right_node, ref node_data) => {
                    let (d, split_pos) = node_data.split;
                    let t_split = if self.ray.dir[d].abs() > Real::epsilon() {
                        (split_pos - self.ray.origin[d]) / self.ray.dir[d]
                    } else {
                        Real::min_value()
                    };
                    let tmin_pos = self.ray.origin[d] + self.ray.dir[d] * t_min;
                    let (fst_node, snd_node) = if t_min > 0.0 {
                        if tmin_pos > split_pos {
                            (right_node, left_node)
                        } else {
                            (left_node, right_node)
                        }
                    } else if self.ray.origin[d] < split_pos {
                        (left_node, right_node)
                    } else {
                        (right_node, left_node)
                    };
                    if t_split > 0.0 && t_min < t_split && t_split < t_max {
                        self.nodes.push((snd_node, (t_split, t_max)));
                        self.nodes.push((fst_node, (t_min, t_split)));

                    } else {
                        self.nodes.push((fst_node, (t_min, t_max)));
                    }
                }
                Node::Leaf(ref objs) => {
                    return Some((objs, t_min, t_max));
                }
            }
        }

        None
    }
}

pub struct KdTreeS<'a, T, S = UniformSampler<'a>>
where
    T: BoundedSurface + ?Sized + 'a,
    S: LuminairesSampler<'a> + for<'s> From<&'s [&'a Surface]> + 'a,
{
    kdtree: KdTree<'a, T>,
    light_sources: Vec<&'a Surface>,
    sampler: Arc<S>,
}

impl<'a, T, S> KdTreeS<'a, T, S>
where
    T: BoundedSurface + ?Sized + 'a,
    S: LuminairesSampler<'a> + for<'s> From<&'s [&'a Surface]> + 'a,
{
    pub fn new<I, U>(obj_iter: U, setup: KdTreeSetup) -> Self
    where
        I: Iterator<Item = &'a T> + 'a,
        U: IntoIterator<Item = I::Item, IntoIter = I> + 'a,
    {
        let mut ls = Vec::new();
        let mut objs = Vec::new();
        for s in obj_iter {
            if s.is_emitter() {
                ls.push(s.as_surface());
            }
            objs.push((s.aabb(), s));
        }
        let sampler = S::from(ls.as_slice());
        Self {
            kdtree: KdTree::build(objs, setup),
            light_sources: ls,
            sampler: Arc::new(sampler),
        }

    }

    pub fn depth(&self) -> usize {
        self.kdtree.depth()
    }
}

impl<'a, T, S> SceneHandler for KdTreeS<'a, T, S>
where
    T: BoundedSurface + ?Sized + 'a,
    S: LuminairesSampler<'a> + for<'s> From<&'s [&'a Surface]> + 'a,
{
    fn intersection(&self, ray: &Ray3f) -> Option<SurfacePoint> {
        let leaf_iter = self.kdtree.traverse_iter(ray);
        let mut t_min = Real::max_value();
        let mut res = None;
        'outer: for (leaf, t_near, t_far) in leaf_iter {
            for s in leaf.iter() {
                if let Some((t, sp)) = s.intersection(ray) {
                    if t > 0.0 && t < t_min {
                        t_min = t;
                        res = Some(sp);
                    }
                }
            }
            if res.is_some() {
                if t_min < t_far {
                    break 'outer;
                }
                assert!(t_min > t_near);
            }
        }

        if let Some(ref mut sp) = res {
            sp.position += sp.normal * consts::POSITION_EPSILON;
        }
        res
    }


    fn light_sources_iter<'s>(&'s self) -> Box<Iterator<Item = &'s Surface> + 's> {
        box self.light_sources.iter().cloned()
    }

    fn light_sources(&self) -> LightSourcesHandler {
        LightSourcesHandler {
            scene: self,
            //sampler: Arc::new(UniformSampler::from(self.light_sources.as_slice()))
            //sampler: Arc::new(LinearSampler::from(self.light_sources.as_slice()))
            sampler: super::lt_arc_trait_hack(self.sampler.clone()),
        }
    }
}

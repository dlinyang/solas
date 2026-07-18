use super::*;

#[derive(Debug, Clone, Copy)]
pub struct FixedVec<T, const L: usize> {
    pub len: usize,
    pub data: [T;L]
}

impl<T: Default, const L: usize> FixedVec<T, L> {
    pub fn new() -> Self {
        Self { len: 0, data: std::array::from_fn(|_| Default::default()) }
    }

    pub fn push(&mut self, value: T) {
        if self.len < L {
            self.data[self.len] = value;
            self.len += 1;
        } else {
            panic!("FixedVec overflow {}", L);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_full(&self) -> bool {
        self.len == L
    }
}

impl<B: Bound + Clone + Copy, T: Clone + Default, const L: usize> BVHTree<B, FixedVec<T,L>> {
    pub fn sah_build_multi_index_in_leaf(boxes: Vec<(T, B)>) -> Self {
        match boxes.len() {
            0 => panic!("Cannot build BVH tree from empty list"),
            _ => {
                if boxes.len() <= L {
                    let mut box_opt = None;
                    let mut fixed_vec = FixedVec::new();
                    for i in 0..boxes.len() {
                        let (index, abox) = boxes[i].clone();
                        if let Some(bbox) = &box_opt {
                            box_opt = Some(Bound::surround(&abox, bbox));
                            fixed_vec.push(index);
                        }
                        else {
                            box_opt = Some(abox);
                        }
                    }
                    return Self::new(box_opt.unwrap(), Some(fixed_vec));
                }
            }
        }

        let centers: Vec<Vec3> = boxes.iter().map(|x| x.1.centroid()).collect();
        let mut min = centers[0];
        let mut max = min;
        for c in &centers {
            min.x = min.x.min(c.x);
            max.x = max.x.max(c.x);
            min.y = min.y.min(c.y);
            max.y = max.y.max(c.y);
            min.z = min.z.min(c.z);
            max.z = max.z.max(c.z);
        }

        let distance = max - min;

        struct Split {
            left: Vec<usize>,
            right: Vec<usize>,
            cost: f32,
        }

        let mut splits: [Split;3] = std::array::from_fn(|_| Split { left: vec![], right: vec![], cost: 0.0});
        // x y z
        for axie in 0..3 {
            if distance[axie] <= 0.0 {
                // TODO: need check all axies is the boxes is very close and the tree to a array
                splits[axie].cost = f32::MAX;
                continue;
            }
            // 12 Bins
            let bin_width = distance[axie] / 12.0;

            // Initialize bins: count of boxes, and union bound for each bin
            struct Bin<B: Bound> {
                indices: Vec<usize>,
                bounds: Option<B>,
            }

            let mut bins: [Bin<B>; 12] = std::array::from_fn(|_| Bin {
                indices: Vec::new(),
                bounds: None,
            });

            for (idx, c) in centers.iter().enumerate() {
                let axis_val = c[axie];
                let bin_idx = ((axis_val - min[axie]) / bin_width).floor() as usize;
                let bin_idx = bin_idx.min(11); // clamp to last bin

                bins[bin_idx].indices.push(idx);
                bins[bin_idx].bounds = if let Some(b) = &bins[bin_idx].bounds {
                    Some(Bound::surround(&boxes[idx].1, &b))
                } else {
                    Some(boxes[idx].1.clone())
                };
            }

            // option<(bin_idx, cost)>
            let mut select_bin: Option<(usize, f32)> = None;

            for bin_idx in 0..12 {
                let mut left_box_opt = None;
                let mut left_count = 0;
                let mut right_box_opt = None;
                let mut right_count = 0;
                for left_idx in 0..(bin_idx + 1) {
                    if let Some(left_box) = &bins[left_idx].bounds {
                        if let Some(left_box_pre) = &left_box_opt {
                            left_box_opt = Some(Bound::surround(left_box, left_box_pre));
                        }
                        else {
                            left_box_opt = Some(left_box.clone())
                        }
                    }
                    left_count += bins[left_idx].indices.len();
                }

                for right_idx in (bin_idx + 1)..12 {
                    if let Some(right_box) = &bins[right_idx].bounds {
                        if let Some(right_box_pre) = &right_box_opt {
                            right_box_opt = Some(Bound::surround(right_box, right_box_pre));
                        }
                        else {
                            right_box_opt = Some(right_box.clone())
                        }
                    }
                    right_count += bins[right_idx].indices.len();
                }

                if let Some(left_box) = left_box_opt && let Some(right_box) = right_box_opt {
                    let cost = left_box.surface_area() * (left_count as f32) + right_box.surface_area() * (right_count as f32);
                    if let Some((_, select_cost)) = select_bin {
                        if cost < select_cost {
                            select_bin = Some((bin_idx, cost));
                        }
                    }
                    else {
                        select_bin = Some((bin_idx, cost));
                    }
                }
            }

            if let Some((select_bin_idx, cost)) = select_bin {
                for left_idx in 0..(select_bin_idx + 1) {
                    splits[axie].left.append(&mut bins[left_idx].indices);
                }

                for right_idx in (select_bin_idx + 1)..12 {
                    splits[axie].right.append(&mut bins[right_idx].indices);
                }

                splits[axie].cost = cost;
            }
            else {
                panic!("select bins wrong axie:{} width:{} bin-width:{} box-num: {}", axie, distance[axie], bin_width, boxes.len());
            }
        }

        let select_axie = if splits[0].cost < splits[1].cost && splits[0].cost < splits[2].cost {
            0
        }
        else if splits[1].cost < splits[2].cost {
            1
        }
        else {
            2
        };

        let split = &splits[select_axie];

        let left_boxes: Vec<(T, B)> = split.left.iter().map(|x| boxes[*x].clone()).collect();
        let right_boxes: Vec<(T, B)> = split.right.iter().map(|x| boxes[*x].clone()).collect();
        let left_tree = Self::sah_build_multi_index_in_leaf(left_boxes);
        let right_tree = Self::sah_build_multi_index_in_leaf(right_boxes);
        Self::new(Bound::surround(&left_tree.node, &right_tree.node), None)
            .with_left(left_tree)
            .with_right(right_tree)
    }
}

impl<B: Bound + Clone + Copy, T: Clone + Default, const L: usize> FlatBVHTree<B, FixedVec<T,L>> {
    pub fn sah_build_multi_index_in_leaf(boxes: Vec<(T, B)>) -> Self {
        let mut bvh = Self::new();
        let top_index = Self::sah_build_multi_index_in_leaf_inner(boxes, &mut bvh);
        bvh.top = Some(top_index);
        bvh
    }

    fn sah_build_multi_index_in_leaf_inner(boxes: Vec<(T, B)>, bvh: &mut Self) -> usize {
        match boxes.len() {
            0 => panic!("Cannot build BVH tree from empty list"),
            _ => {
                if boxes.len() <= L {
                    let mut box_opt = None;
                    let mut fixed_vec = FixedVec::new();
                    for i in 0..boxes.len() {
                        let (index, abox) = boxes[i].clone();
                        if let Some(bbox) = &box_opt {
                            box_opt = Some(Bound::surround(&abox, bbox));
                            fixed_vec.push(index);
                        }
                        else {
                            box_opt = Some(abox);
                        }
                    }
                    return bvh.push(BVHTreeNode::leaf(box_opt.unwrap(), fixed_vec));
                }
            }
        }

        let centers: Vec<Vec3> = boxes.iter().map(|x| x.1.centroid()).collect();
        let mut min = centers[0];
        let mut max = min;
        for c in &centers {
            min.x = min.x.min(c.x);
            max.x = max.x.max(c.x);
            min.y = min.y.min(c.y);
            max.y = max.y.max(c.y);
            min.z = min.z.min(c.z);
            max.z = max.z.max(c.z);
        }

        let distance = max - min;

        struct Split {
            left: Vec<usize>,
            right: Vec<usize>,
            cost: f32,
        }

        let mut splits: [Split;3] = std::array::from_fn(|_| Split { left: vec![], right: vec![], cost: 0.0});
        // x y z
        for axie in 0..3 {
            if distance[axie] <= 0.0 {
                // TODO: need check all axies is the boxes is very close and the tree to a array
                splits[axie].cost = f32::MAX;
                continue;
            }
            // 12 Bins
            let bin_width = distance[axie] / 12.0;

            // Initialize bins: count of boxes, and union bound for each bin
            struct Bin<B: Bound> {
                indices: Vec<usize>,
                bounds: Option<B>,
            }

            let mut bins: [Bin<B>; 12] = std::array::from_fn(|_| Bin {
                indices: Vec::new(),
                bounds: None,
            });

            for (idx, c) in centers.iter().enumerate() {
                let axis_val = c[axie];
                let bin_idx = ((axis_val - min[axie]) / bin_width).floor() as usize;
                let bin_idx = bin_idx.min(11); // clamp to last bin

                bins[bin_idx].indices.push(idx);
                bins[bin_idx].bounds = if let Some(b) = &bins[bin_idx].bounds {
                    Some(Bound::surround(&boxes[idx].1, &b))
                } else {
                    Some(boxes[idx].1.clone())
                };
            }

            // option<(bin_idx, cost)>
            let mut select_bin: Option<(usize, f32)> = None;

            for bin_idx in 0..12 {
                let mut left_box_opt = None;
                let mut left_count = 0;
                let mut right_box_opt = None;
                let mut right_count = 0;
                for left_idx in 0..(bin_idx + 1) {
                    if let Some(left_box) = &bins[left_idx].bounds {
                        if let Some(left_box_pre) = &left_box_opt {
                            left_box_opt = Some(Bound::surround(left_box, left_box_pre));
                        }
                        else {
                            left_box_opt = Some(left_box.clone())
                        }
                    }
                    left_count += bins[left_idx].indices.len();
                }

                for right_idx in (bin_idx + 1)..12 {
                    if let Some(right_box) = &bins[right_idx].bounds {
                        if let Some(right_box_pre) = &right_box_opt {
                            right_box_opt = Some(Bound::surround(right_box, right_box_pre));
                        }
                        else {
                            right_box_opt = Some(right_box.clone())
                        }
                    }
                    right_count += bins[right_idx].indices.len();
                }

                if let Some(left_box) = left_box_opt && let Some(right_box) = right_box_opt {
                    let cost = left_box.surface_area() * (left_count as f32) + right_box.surface_area() * (right_count as f32);
                    if let Some((_, select_cost)) = select_bin {
                        if cost < select_cost {
                            select_bin = Some((bin_idx, cost));
                        }
                    }
                    else {
                        select_bin = Some((bin_idx, cost));
                    }
                }
            }

            if let Some((select_bin_idx, cost)) = select_bin {
                for left_idx in 0..(select_bin_idx + 1) {
                    splits[axie].left.append(&mut bins[left_idx].indices);
                }

                for right_idx in (select_bin_idx + 1)..12 {
                    splits[axie].right.append(&mut bins[right_idx].indices);
                }

                splits[axie].cost = cost;
            }
            else {
                panic!("select bins wrong axie:{} width:{} bin-width:{} box-num: {}", axie, distance[axie], bin_width, boxes.len());
            }
        }

        let select_axie = if splits[0].cost < splits[1].cost && splits[0].cost < splits[2].cost {
            0
        }
        else if splits[1].cost < splits[2].cost {
            1
        }
        else {
            2
        };

        let split = &splits[select_axie];

        let left_boxes: Vec<(T, B)> = split.left.iter().map(|x| boxes[*x].clone()).collect();
        let right_boxes: Vec<(T, B)> = split.right.iter().map(|x| boxes[*x].clone()).collect();
        let left_idx = Self::sah_build_multi_index_in_leaf_inner(left_boxes, bvh);
        let right_idx = Self::sah_build_multi_index_in_leaf_inner(right_boxes, bvh);
        let node = BVHTreeNode::branch(
            Bound::surround(&bvh.nodes[left_idx].node, &bvh.nodes[right_idx].node),
            left_idx,
            right_idx,
        );
        bvh.push(node)
    }
}

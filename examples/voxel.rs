use fixed_vectors::Vector3;
use core::ops::Index;
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
struct Voxel {
    pub position: Vector3<u32>,
}

#[derive(Debug, PartialEq, Eq)]
struct VoxelChunk {
    voxels: Vec<Voxel>,
    size: Vector3<u32>,
}

impl VoxelChunk {
    pub fn new(size: Vector3<u32>) -> Self {
        let mut voxels = Vec::with_capacity((size.x * size.y * size.z) as usize);

        for x in 0 .. size.x {
            for y in 0 .. size.y {
                for z in 0 .. size.z {
                    let new_voxel = Voxel { position: Vector3::new(x, y, z) };
                    voxels.push(new_voxel);
                }
            }
        }

        return Self {
            voxels,
            size,
        };
    }
}

impl Index<Vector3<u32>> for VoxelChunk {
    type Output = Voxel;

    fn index(&self, index: Vector3<u32>) -> &Self::Output {
        return &self.voxels[position_as_index(self.size, index)];
    }
}

/*
    NOTE: Because of the order a [`VoxelChunk`] is constructed, we can use this formula to estimate an index
        of a [`Voxel`] within the `voxels` [`Vec`].
*/
const fn position_as_index(size: Vector3<u32>, position: Vector3<u32>) -> usize {
    return (position.z + (position.y * size.z) + (position.x * size.z * size.y)) as usize;
}

fn main() {
    let chunk = VoxelChunk::new(Vector3::new(8, 16, 8));
    let position = Vector3::<u32>::new(4, 8, 4);
    assert_eq!(chunk[position].position, position);
}
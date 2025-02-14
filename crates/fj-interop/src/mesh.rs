//! A triangle mesh

use std::{collections::HashMap, hash::Hash};

use fj_math::Point;

/// A triangle mesh
#[derive(Clone, Debug)]
pub struct Mesh<V> {
    vertices: Vec<V>,
    indices: Vec<Index>,

    indices_by_vertex: HashMap<V, Index>,
    triangles: Vec<Triangle>,
}

impl<V> Mesh<V>
where
    V: Copy + Eq + Hash,
{
    /// Construct a new instance of `Mesh`
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a vertex to the mesh
    pub fn push_vertex(&mut self, vertex: V) {
        let index =
            *self.indices_by_vertex.entry(vertex).or_insert_with(|| {
                let index = self.vertices.len();
                self.vertices.push(vertex);
                index as u32
            });

        self.indices.push(index);
    }

    /// Determine whether the mesh contains the provided triangle
    ///
    /// Returns true, if a triangle with any combination of the provided points
    /// is part of the mesh.
    pub fn contains_triangle(
        &self,
        triangle: impl Into<fj_math::Triangle<3>>,
    ) -> bool {
        let triangle = triangle.into().normalize();

        for t in &self.triangles {
            let t = t.inner.normalize();
            if triangle == t {
                return true;
            }
        }

        false
    }

    /// Access the vertices of the mesh
    pub fn vertices(&self) -> impl Iterator<Item = V> + '_ {
        self.vertices.iter().copied()
    }

    /// Access the indices of the mesh
    pub fn indices(&self) -> impl Iterator<Item = Index> + '_ {
        self.indices.iter().copied()
    }

    /// Access the triangles of the mesh
    pub fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.triangles.iter().copied()
    }
}

impl Mesh<Point<3>> {
    /// Add a triangle to the mesh
    pub fn push_triangle(
        &mut self,
        triangle: impl Into<fj_math::Triangle<3>>,
        color: Color,
    ) {
        let triangle = triangle.into();

        for point in triangle.points() {
            self.push_vertex(point);
        }

        self.triangles.push(Triangle {
            inner: triangle,
            color,
        });
    }
}

// This needs to be a manual implementation. Deriving `Default` would require
// `V` to be `Default` as well, even though that is not necessary.
impl<V> Default for Mesh<V> {
    fn default() -> Self {
        Self {
            vertices: Vec::default(),
            indices: Vec::default(),
            indices_by_vertex: HashMap::default(),
            triangles: Vec::default(),
        }
    }
}

/// An index that refers to a vertex in a mesh
pub type Index = u32;

/// A triangle
///
/// Extension of [`fj_math::Triangle`] that also includes a color.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Triangle {
    /// The points of the triangle
    pub inner: fj_math::Triangle<3>,

    /// The color of the triangle
    pub color: Color,
}

/// RGBA color
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Color(pub [u8; 4]);

impl Default for Color {
    fn default() -> Self {
        // The default color is red. This is an arbitrary choice.
        Self([255, 0, 0, 255])
    }
}

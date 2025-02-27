use petgraph::Graph;

/**
 *    Trait IndexType: Copy + Default + Hash + Ord + Debug
 *    new(x: usize) -> Self
 *    index(&self) -> usize
 *    max() -> Self
 *    This trait is implemented for usize,u32,u16,u8 etc.
 *    DefaultIx = u32
 */


/**
 *     Struct NodeIndex<Ix=DefaultIx>(Ix)
 *     This struct also implements IndexType when Ix is IndexType
 *
   pub fn new(x: usize) -> Self {
       NodeIndex(IndexType::new(x))
   }

   pub fn index(self) -> usize {
       self.0.index()
   }

   pub fn end() -> Self {
       NodeIndex(IndexType::max())
   }

   fn _into_edge(self) -> EdgeIndex<Ix> {
       EdgeIndex(self.0)
   }
*/

/**
 *  EdgeIndex is defined exactly as NodeIndex
 *
 */

/**
 * struct Node<N, Ix=DefaultIx>
        pub weight: N,
        next: [EdgeIndex<Ix>; 2]

        impl<E, Ix> Clone for Node<E, Ix>
        where
            E: Clone,
            Ix: Copy,
        {
            clone_fields!(Node, weight, next,);
        }

        impl<N, Ix: IndexType> Node<N, Ix> {
            /// Accessor for data structure internals: the first edge in the given direction.
            pub fn next_edge(&self, dir: Direction) -> EdgeIndex<Ix> {
                self.next[dir.index()]
            }
        }
*/

/** Edge<E, Ix>
 *
pub struct Edge<E, Ix = DefaultIx> {
    /// Associated edge data.
    pub weight: E,
    /// Next edge in outgoing and incoming edge lists.
    next: [EdgeIndex<Ix>; 2],
    /// Start and End node index
    node: [NodeIndex<Ix>; 2],
}

impl<E, Ix> Clone for Edge<E, Ix>
where
    E: Clone,
    Ix: Copy,
{
    clone_fields!(Edge, weight, next, node,);
}

impl<E, Ix: IndexType> Edge<E, Ix> {
    /// Accessor for data structure internals: the next edge for the given direction.
    pub fn next_edge(&self, dir: Direction) -> EdgeIndex<Ix> {
        self.next[dir.index()]
    }

    /// Return the source node index.
    pub fn source(&self) -> NodeIndex<Ix> {
        self.node[0]
    }

    /// Return the target node index.
    pub fn target(&self) -> NodeIndex<Ix> {
        self.node[1]
    }
}
*/



use crate::consts::MAX_PHYSICAL_PAGES;
use spin::Mutex;

pub struct SegmentTreeAllocator {
    nodes: [u8; MAX_PHYSICAL_PAGES << 1], // allocate double max nodes
    leaf_begin: usize, // our real pages are at leaf node, which is not indexed at 0
    total: usize,      // how many pages we have in total
    offset: usize,     // since physical page could start at non-zero
}

// root at one as routine
// inline helper funtion, they can be better to be written in marco or association function
impl SegmentTreeAllocator {
    #[inline]
    fn left_child(&self, idx: usize) -> usize {
        idx << 1
    }
    #[inline]
    fn right_child(&self, idx: usize) -> usize {
        (idx << 1) | 1
    }
    #[inline]
    fn update(&mut self, idx: usize) {
        let mut temp = idx >> 1;
        while temp > 0 {
            // if left child and right child is full, then nodes[p] is full
            self.nodes[temp] =
                self.nodes[self.left_child(temp)] & self.nodes[self.right_child(temp)];
            temp >>= 1;
        }
    }
}

// consturct a complete binary tree and put all real node into leaves.
// so [l,r) length assignment needs pow(2, log(2,r-l)+1) leaves to store, from lfet most child to right
impl SegmentTreeAllocator {
    // init [l,r) interval
    pub fn init(&mut self, l: usize, r: usize) {
        self.offset = l;
        self.total = r - l; // total may be not a power of 2
                            // compute the first key index
        self.leaf_begin = 1;
        // why add 2?
        // one for root start from one instead of zero
        // another one for total number is contained, say we have 10 number,
        // naturally, the condition shall be less than 11, you need to take 10 into account
        while self.leaf_begin < self.total + 2 {
            self.leaf_begin <<= 1;
        }
        // since [l,r) may not be aligned, we need to let extra element marked as invalid
        // on level of leaf, you have pow(2,leaf_begin)'s leaves.
        // so total elements of this complete binary tree should be
        //  1 + 2 + 4 +  ... + pow(2,leaf_begin)
        // pow(2,leaf_begin+1) - 1, namely range from [1,pow(2,leaf_begin+1)
        for i in 1..(self.leaf_begin << 1) {
            self.nodes[i] = 1;
        }
        // avaiable part, please notice it shall start from left child, which is even
        // different from origin version,
        // so you can see above self.offset is l, not l-1
        for i in 0..self.total {
            self.nodes[self.leaf_begin + i] = 0;
        }
        // update parents who are available
        for i in (1..self.leaf_begin).rev() {
            self.nodes[i] = self.nodes[self.left_child(i)] & self.nodes[self.right_child(i)];
        }
    }

    // allocate a physical page, return a physical page number
    pub fn alloc(&mut self) -> usize {
        // assume that we never run out of physical memory
        if self.nodes[1] == 1 {
            panic!("physical memory depleted!");
        }
        // from top, find first available page
        let mut p = 1;
        while p < self.leaf_begin {
            if self.nodes[self.left_child(p)] == 0 {
                p = self.left_child(p);
            } else {
                p = self.right_child(p);
            }
        }
        // since it has offset, we need to compute
        let result = (p - self.leaf_begin) + self.offset;
        // it has been allocated, we need to mark it as used
        self.nodes[p] = 1;
        // update its ancestor
        self.update(p);
        result
    }

    // recycle the target page
    pub fn dealloc(&mut self, idx: usize) {
        // notice there is a difference between alloc and de-alloc
        // the operation order is inversed.
        let pos_in_tree = (idx - self.offset) + self.leaf_begin;
        // we recycle a used one
        assert!(self.nodes[pos_in_tree] == 1);
        self.nodes[pos_in_tree] = 0;
        self.update(pos_in_tree);
    }
}

// singleton
pub static SEGMENT_TREE_ALLOCATOR: Mutex<SegmentTreeAllocator> = Mutex::new(SegmentTreeAllocator {
    nodes: [0; MAX_PHYSICAL_PAGES << 1],
    leaf_begin: 0,
    total: 0,
    offset: 0,
});

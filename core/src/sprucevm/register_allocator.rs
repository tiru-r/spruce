/// Advanced register allocator with liveness analysis
/// 
/// Implements graph-coloring register allocation for optimal register usage
use anyhow::Result;
use std::collections::{HashMap, HashSet, BTreeSet};

/// Variable identifier
pub type VarId = u32;

/// Register identifier  
pub type RegId = u8;

/// Instruction position in bytecode
pub type InstrPos = u32;

/// Liveness interval for a variable
#[derive(Debug, Clone, PartialEq)]
pub struct LiveInterval {
    /// Variable this interval represents
    pub var_id: VarId,
    /// Start position (inclusive)
    pub start: InstrPos,
    /// End position (inclusive) 
    pub end: InstrPos,
    /// Instruction positions where variable is used
    pub use_positions: BTreeSet<InstrPos>,
    /// Assigned register (if any)
    pub assigned_register: Option<RegId>,
    /// Spill cost (higher = more expensive to spill)
    pub spill_cost: f32,
}

impl LiveInterval {
    pub fn new(var_id: VarId, start: InstrPos, end: InstrPos) -> Self {
        Self {
            var_id,
            start,
            end,
            use_positions: BTreeSet::new(),
            assigned_register: None,
            spill_cost: 0.0,
        }
    }

    /// Check if this interval overlaps with another
    pub fn overlaps_with(&self, other: &LiveInterval) -> bool {
        !(self.end < other.start || other.end < self.start)
    }

    /// Length of the live interval
    pub fn length(&self) -> u32 {
        self.end - self.start + 1
    }

    /// Add a use position and update spill cost
    pub fn add_use(&mut self, pos: InstrPos, weight: f32) {
        self.use_positions.insert(pos);
        self.spill_cost += weight;
    }
}

/// Interference graph for register allocation
#[derive(Debug)]
pub struct InterferenceGraph {
    /// Adjacency list representation
    adj_list: HashMap<VarId, HashSet<VarId>>,
    /// All variables in the graph
    variables: HashSet<VarId>,
}

impl InterferenceGraph {
    pub fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
            variables: HashSet::new(),
        }
    }

    /// Add a variable to the graph
    pub fn add_variable(&mut self, var_id: VarId) {
        self.variables.insert(var_id);
        self.adj_list.entry(var_id).or_insert_with(HashSet::new);
    }

    /// Add interference edge between two variables
    pub fn add_interference(&mut self, var1: VarId, var2: VarId) {
        if var1 != var2 {
            self.adj_list.entry(var1).or_default().insert(var2);
            self.adj_list.entry(var2).or_default().insert(var1);
        }
    }

    /// Get neighbors of a variable
    pub fn neighbors(&self, var_id: VarId) -> Option<&HashSet<VarId>> {
        self.adj_list.get(&var_id)
    }

    /// Get degree of a variable (number of neighbors)
    pub fn degree(&self, var_id: VarId) -> usize {
        self.neighbors(var_id).map(|n| n.len()).unwrap_or(0)
    }

    /// Remove a variable and all its edges
    pub fn remove_variable(&mut self, var_id: VarId) {
        if let Some(neighbors) = self.adj_list.remove(&var_id) {
            for neighbor in neighbors {
                if let Some(neighbor_set) = self.adj_list.get_mut(&neighbor) {
                    neighbor_set.remove(&var_id);
                }
            }
        }
        self.variables.remove(&var_id);
    }

    /// Get all variables
    pub fn variables(&self) -> &HashSet<VarId> {
        &self.variables
    }
}

/// Advanced register allocator
#[derive(Debug)]
pub struct RegisterAllocator {
    /// Number of available registers
    num_registers: u8,
    /// Live intervals for all variables
    intervals: Vec<LiveInterval>,
    /// Interference graph
    interference_graph: InterferenceGraph,
    /// Final register assignment
    allocation: HashMap<VarId, RegId>,
    /// Variables that need to be spilled
    spilled_vars: HashSet<VarId>,
}

impl RegisterAllocator {
    /// Create new register allocator
    pub fn new(num_registers: u8) -> Self {
        Self {
            num_registers,
            intervals: Vec::new(),
            interference_graph: InterferenceGraph::new(),
            allocation: HashMap::new(),
            spilled_vars: HashSet::new(),
        }
    }

    /// Add a live interval
    pub fn add_interval(&mut self, interval: LiveInterval) {
        self.interference_graph.add_variable(interval.var_id);
        self.intervals.push(interval);
    }

    /// Build interference graph from live intervals
    pub fn build_interference_graph(&mut self) {
        // Clear existing graph
        self.interference_graph = InterferenceGraph::new();
        
        // Add all variables
        for interval in &self.intervals {
            self.interference_graph.add_variable(interval.var_id);
        }

        // Add interference edges for overlapping intervals
        for i in 0..self.intervals.len() {
            for j in i + 1..self.intervals.len() {
                if self.intervals[i].overlaps_with(&self.intervals[j]) {
                    self.interference_graph.add_interference(
                        self.intervals[i].var_id,
                        self.intervals[j].var_id,
                    );
                }
            }
        }
    }

    /// Perform graph-coloring register allocation
    pub fn allocate_registers(&mut self) -> Result<()> {
        // Build interference graph
        self.build_interference_graph();

        // Try graph coloring first
        if self.try_graph_coloring() {
            return Ok(());
        }

        // If coloring fails, use spilling
        self.allocate_with_spilling()
    }

    /// Try to color the graph with available registers
    fn try_graph_coloring(&mut self) -> bool {
        let mut graph = self.interference_graph.clone();
        let mut stack = Vec::new();
        let k = self.num_registers as usize;

        // Simplification phase - remove nodes with degree < k
        loop {
            let low_degree_node = graph.variables()
                .iter()
                .find(|&&var| graph.degree(var) < k)
                .copied();

            if let Some(node) = low_degree_node {
                stack.push(node);
                graph.remove_variable(node);
            } else if !graph.variables().is_empty() {
                // No more low-degree nodes but graph not empty
                // Need to spill - for now just return false
                return false;
            } else {
                break;
            }
        }

        // Selection phase - assign colors
        self.allocation.clear();
        let mut used_registers = HashMap::new();

        while let Some(var_id) = stack.pop() {
            // Find available register
            let mut available = (0..self.num_registers).collect::<HashSet<_>>();
            
            // Remove registers used by neighbors
            if let Some(neighbors) = self.interference_graph.neighbors(var_id) {
                for &neighbor in neighbors {
                    if let Some(&reg) = self.allocation.get(&neighbor) {
                        available.remove(&reg);
                    }
                }
            }

            if let Some(&reg) = available.iter().next() {
                self.allocation.insert(var_id, reg);
                used_registers.insert(var_id, reg);
            } else {
                // No available register - spilling needed
                return false;
            }
        }

        true
    }

    /// Allocate registers with spilling when necessary
    fn allocate_with_spilling(&mut self) -> Result<()> {
        // Linear scan allocation as fallback
        self.linear_scan_allocation()
    }

    /// Linear scan register allocation algorithm  
    fn linear_scan_allocation(&mut self) -> Result<()> {
        // Sort intervals by start position
        self.intervals.sort_by_key(|interval| interval.start);

        let mut active: Vec<usize> = Vec::new(); // indices into intervals
        self.allocation.clear();
        self.spilled_vars.clear();

        for (i, interval) in self.intervals.iter().enumerate() {
            // Expire old intervals
            active.retain(|&j| {
                if self.intervals[j].end < interval.start {
                    // Free the register
                    false
                } else {
                    true
                }
            });

            if active.len() < self.num_registers as usize {
                // Assign free register
                let mut used_regs = HashSet::new();
                for &j in &active {
                    if let Some(reg) = self.intervals[j].assigned_register {
                        used_regs.insert(reg);
                    }
                }

                for reg in 0..self.num_registers {
                    if !used_regs.contains(&reg) {
                        self.allocation.insert(interval.var_id, reg);
                        active.push(i);
                        break;
                    }
                }
            } else {
                // Need to spill
                let spill_candidate = self.choose_spill_candidate(&active, i);
                
                if let Some(spill_idx) = spill_candidate {
                    let spill_var = self.intervals[spill_idx].var_id;
                    self.spilled_vars.insert(spill_var);
                    self.allocation.remove(&spill_var);
                    
                    // Remove from active list
                    active.retain(|&j| j != spill_idx);
                    
                    // Assign register to current interval
                    let freed_reg = self.intervals[spill_idx].assigned_register.unwrap();
                    self.allocation.insert(interval.var_id, freed_reg);
                    active.push(i);
                } else {
                    // Spill current interval
                    self.spilled_vars.insert(interval.var_id);
                }
            }
        }

        Ok(())
    }

    /// Choose which variable to spill based on spill cost
    fn choose_spill_candidate(&self, active: &[usize], current: usize) -> Option<usize> {
        let current_interval = &self.intervals[current];
        
        // Find interval with lowest spill cost among active intervals
        let mut best_candidate = None;
        let mut lowest_cost = current_interval.spill_cost;

        for &idx in active {
            let interval = &self.intervals[idx];
            if interval.spill_cost < lowest_cost {
                lowest_cost = interval.spill_cost;
                best_candidate = Some(idx);
            }
        }

        best_candidate
    }

    /// Get register assignment for a variable
    pub fn get_register(&self, var_id: VarId) -> Option<RegId> {
        self.allocation.get(&var_id).copied()
    }

    /// Check if variable is spilled
    pub fn is_spilled(&self, var_id: VarId) -> bool {
        self.spilled_vars.contains(&var_id)
    }

    /// Get all spilled variables
    pub fn spilled_variables(&self) -> &HashSet<VarId> {
        &self.spilled_vars
    }

    /// Get register allocation statistics
    pub fn get_stats(&self) -> AllocationStats {
        AllocationStats {
            total_variables: self.intervals.len(),
            allocated_variables: self.allocation.len(),
            spilled_variables: self.spilled_vars.len(),
            register_utilization: self.allocation.len() as f32 / self.num_registers as f32,
        }
    }
}

impl Clone for InterferenceGraph {
    fn clone(&self) -> Self {
        Self {
            adj_list: self.adj_list.clone(),
            variables: self.variables.clone(),
        }
    }
}

/// Register allocation statistics
#[derive(Debug)]
pub struct AllocationStats {
    pub total_variables: usize,
    pub allocated_variables: usize,
    pub spilled_variables: usize,
    pub register_utilization: f32,
}

/// Liveness analysis for bytecode
pub struct LivenessAnalyzer {
    /// Variable definitions (var_id -> instruction position)
    defs: HashMap<VarId, InstrPos>,
    /// Variable uses (var_id -> set of instruction positions)
    uses: HashMap<VarId, BTreeSet<InstrPos>>,
}

impl LivenessAnalyzer {
    pub fn new() -> Self {
        Self {
            defs: HashMap::new(),
            uses: HashMap::new(),
        }
    }

    /// Record variable definition
    pub fn define_var(&mut self, var_id: VarId, pos: InstrPos) {
        self.defs.insert(var_id, pos);
    }

    /// Record variable use
    pub fn use_var(&mut self, var_id: VarId, pos: InstrPos) {
        self.uses.entry(var_id).or_default().insert(pos);
    }

    /// Compute live intervals for all variables
    pub fn compute_live_intervals(&self) -> Vec<LiveInterval> {
        let mut intervals = Vec::new();

        for (&var_id, &def_pos) in &self.defs {
            let use_positions = self.uses.get(&var_id).cloned().unwrap_or_default();
            
            let start = def_pos;
            let end = use_positions.iter().last().copied().unwrap_or(def_pos);

            let mut interval = LiveInterval::new(var_id, start, end);
            
            // Add use positions with weights
            for &use_pos in &use_positions {
                interval.add_use(use_pos, 1.0); // Basic weight, could be loop-adjusted
            }

            intervals.push(interval);
        }

        intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_interval_overlap() {
        let interval1 = LiveInterval::new(1, 0, 10);
        let interval2 = LiveInterval::new(2, 5, 15);
        let interval3 = LiveInterval::new(3, 20, 30);

        assert!(interval1.overlaps_with(&interval2));
        assert!(!interval1.overlaps_with(&interval3));
        assert!(!interval2.overlaps_with(&interval3));
    }

    #[test]
    fn test_register_allocation() {
        let mut allocator = RegisterAllocator::new(4);
        
        // Add some intervals
        allocator.add_interval(LiveInterval::new(1, 0, 10));
        allocator.add_interval(LiveInterval::new(2, 5, 15));
        allocator.add_interval(LiveInterval::new(3, 12, 20));

        let result = allocator.allocate_registers();
        assert!(result.is_ok());

        // Should allocate registers successfully
        assert!(allocator.get_register(1).is_some());
        assert!(allocator.get_register(2).is_some());
        assert!(allocator.get_register(3).is_some());
    }
}
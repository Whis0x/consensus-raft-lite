#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role { Follower, Candidate, Leader }

#[derive(Debug, Clone)]
pub struct RaftNode {
    pub id: u64,
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub role: Role,
    pub votes_received: usize,
}

impl RaftNode {
    pub fn new(id: u64) -> Self {
        Self { id, current_term: 0, voted_for: None, role: Role::Follower, votes_received: 0 }
    }
    pub fn start_election(&mut self) {
        self.current_term += 1;
        self.role = Role::Candidate;
        self.voted_for = Some(self.id);
        self.votes_received = 1;
    }
    pub fn handle_vote_request(&mut self, candidate_id: u64, term: u64) -> bool {
        if term > self.current_term {
            self.current_term = term;
            self.role = Role::Follower;
            self.voted_for = None;
        }
        if term == self.current_term && (self.voted_for.is_none() || self.voted_for == Some(candidate_id)) {
            self.voted_for = Some(candidate_id);
            return true;
        }
        false
    }
    pub fn receive_vote(&mut self, cluster_size: usize) {
        if self.role == Role::Candidate {
            self.votes_received += 1;
            if self.votes_received > cluster_size / 2 { self.role = Role::Leader; }
        }
    }
}

pub fn check_election_safety(nodes: &[RaftNode]) -> bool {
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            if nodes[i].role == Role::Leader && nodes[j].role == Role::Leader && nodes[i].current_term == nodes[j].current_term {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enforces_quorum_and_election_safety() {
        let mut n1 = RaftNode::new(1);
        let mut n2 = RaftNode::new(2);
        let n3 = RaftNode::new(3);
        n1.start_election();
        assert!(n2.handle_vote_request(1, n1.current_term));
        n1.receive_vote(3);
        assert_eq!(n1.role, Role::Leader);
        assert!(!n2.handle_vote_request(3, 1));
        assert!(check_election_safety(&[n1, n2, n3]));
    }
}

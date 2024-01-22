use std::collections::HashMap;

#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    clans: HashMap<String, Vec<String>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        // unimplemented!();
        ClanSystem {clans: HashMap::new()}
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        // unimplemented!();
        match self.clans.get(clan_id) {
            Some(v) => v.clone(),
            None => Vec::new(),
        }
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        // unimplemented!();
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        // unimplemented!();
        match self.clans.get(clan_id) {
            Some(v) => v.len(),
            None => 0,
        }
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        // unimplemented!();
        let mut largest_clan_id: Option<String> = None;
        let mut largest_clan_size = 0;

        for (id, members) in &self.clans {
            let current_clan_size = members.len();
            if current_clan_size > largest_clan_size {
                largest_clan_size = current_clan_size;
                largest_clan_id = Some(id.clone());
            }
        }

        largest_clan_id

    }

    pub fn is_crab_in_any_clan(&self, crab_name: &str) -> bool {
        for members in self.clans.values() {
            if members.contains(&crab_name.to_string()) {
                return true;
            }
        }
        false
}

    pub fn add_member(&mut self, clan_id: &str, crab_name: String) {
        self.clans.entry(clan_id.to_string())
            .or_insert_with(Vec::new)
            .push(crab_name);
    }
}
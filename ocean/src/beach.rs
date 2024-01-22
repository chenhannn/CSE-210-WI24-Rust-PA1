use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    pub list: Vec<Crab>,
    pub clan_system: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        // unimplemented!();
        Beach {list: Vec::new(), clan_system: ClanSystem::new()}
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        // unimplemented!();
        self.list.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        // unimplemented!();
        self.list.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        // unimplemented!();
        &self.list[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        // unimplemented!();
        self.list.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        // unimplemented!();
        if self.list.len() == 0 {
            return None;
        }
        self.list.iter().max_by_key(|crab| crab.speed)
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        // unimplemented!();
        self.list.iter().filter(|crab| crab.name() == name).collect()
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        // unimplemented!();
        let parent1 = self.list.get(i).expect("Index i out of bounds");
        let parent2 = self.list.get(j).expect("Index j out of bounds");

        let baby = parent1.breed(parent2, name);

        self.list.push(baby);
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        // unimplemented!();
        &self.clan_system
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        // unimplemented!();
        if !self.clan_system.is_crab_in_any_clan(crab_name) {
            self.clan_system.add_member(clan_id, crab_name.to_string());
        }
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        // unimplemented!();
        let avg_speed_1 = self.calculate_average_clan_speed(id1)?;
        let avg_speed_2 = self.calculate_average_clan_speed(id2)?;

        if avg_speed_1 > avg_speed_2 {
            Ok(Some(id1.to_string()))
        } else if avg_speed_2 > avg_speed_1 {
            Ok(Some(id2.to_string()))
        } else {
            Ok(None)
        }
    }

    fn calculate_average_clan_speed(&self, clan_id: &str) -> Result<f32, String> {
        let clan_members = self.clan_system.get_clan_member_names(clan_id);

        if clan_members.is_empty() {
            return Err(format!("Clan with ID '{}' does not exist or has no members", clan_id));
        }

        let mut total_speed = 0;
        let mut member_count = 0;

        for member_name in clan_members {
            let crabs = self.find_crabs_by_name(&member_name);
            for crab in crabs {
                total_speed += crab.speed();
                member_count += 1;
            }
        }

        if member_count == 0 {
            return Err(format!("No crabs found for clan ID '{}'", clan_id));
        }

        let avg_speed = total_speed as f32 / member_count as f32;
        Ok(avg_speed)
    }

}

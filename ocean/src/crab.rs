use crate::color::Color;
use crate::cookbook::{Cookbook, Recipe};
use crate::diet::Diet;
use crate::prey::Prey;
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Crab {
    // TODO: Add fields here (some in part 1, some in part 2)
    pub name: String,
    pub speed: u32,
    pub color: Color,
    pub diet: Diet,
    pub reefs: Vec<Rc<RefCell<Reef>>>,
}

// Do NOT implement Copy for Crab.
impl Crab {
    pub fn new(name: String, speed: u32, color: Color, diet: Diet) -> Crab {
        // unimplemented!();
        Crab {name, speed, color, diet, reefs: Vec::new()}
    }

    pub fn name(&self) -> &str {
        // unimplemented!();
        &self.name
    }

    pub fn speed(&self) -> u32 {
        // unimplemented!();
        self.speed
    }

    pub fn color(&self) -> &Color {
        // unimplemented!();
        &self.color
    }

    pub fn diet(&self) -> Diet {
        // unimplemented!();
        self.diet
    }

    pub fn breed(&self, partner: &Crab, name: String) -> Crab {
        let new_color = Color::cross(&self.color, &partner.color);
        let new_diet = Diet::random_diet();
        Crab::new(name, 1, new_color, new_diet) 
    }

    // PART 2 BELOW
    // ------------

    /**
     * Have this crab discover a new reef, adding it to its list of reefs.
     */
    pub fn discover_reef(&mut self, reef: Rc<RefCell<Reef>>) {
        // unimplemented!();
        self.reefs.push(reef);
    }

    /**
     * Returns Some prey from one of the reefs this crab feeds from,
     * and the index of that reef in self.reefs if able to find Some prey
     * using the `take_prey` method of Reef.
     *
     * If `take_prey` returns None, try the next reef. Try each reef only once.
     *
     * If all reefs are empty, or this crab has no reefs, return None.
     */
    fn catch_prey(&mut self) -> Option<(Box<dyn Prey>, usize)> {
        // unimplemented!();
        for (i, reef) in self.reefs.iter().enumerate() {
            if let Some(prey) = reef.borrow_mut().take_prey() {
                return Some((prey, i));
            }
        }
        None
    }

    /**
     * Releases the given prey back into the reef at the given index.
     */
    fn release_prey(&mut self, prey: Box<dyn Prey>, reef_index: usize) {
        // unimplemented!();
        self.reefs[reef_index].borrow_mut().add_prey(prey);
    }

    /**
     * Have this crab go hunting.
     *
     * A crab will keep trying to catch prey until it succeeds,
     * or runs out of remaining prey to try to catch.
     *
     * You should keep track of all escaped prey in a local.
     *
     * Once you have finished hunting, release all escaped prey back into
     * the reefs from whence they came _before_ you return if prey was caught.
     *
     * Your algorithm might look something like this pseudocode. The challenge
     * in this task is not intended to be in figuring out the algorithm, but
     * in figuring out _how to write it in Rust_ using what you've learned so far.
     *
     * ```text
     *     there are no escaped prey yet
     *     prey has not been caught
     *     while prey can be caught
     *       if prey escapes
     *         mark the prey as escaped
     *         try again
     *     
     *       if prey is not edible by this crab
     *         mark the prey as escaped
     *         try again
     *       
     *       prey has been caught
     *       stop trying
     *     
     *     release each escaped prey back to its reef
     *     was prey caught?
     * ```
     *
     * Note: this pseudocode reads like a terrible poem.
     */
    pub fn hunt(&mut self) -> bool {
        // unimplemented!();
        let mut escaped_prey = Vec::new();
        let mut prey_caught = false;
        while let Some((mut prey, index)) = self.catch_prey() {
            if prey.try_escape(self) {
                escaped_prey.push((prey, index));
            } else if self.can_eat(&prey) {
                prey_caught = true;
                break;
            } else {
                escaped_prey.push((prey, index));
            }
            if prey_caught {
                break;
            }
        }
        for (prey, index) in escaped_prey {
            self.release_prey(prey, index);
        }
       

        // Release all the escaped prey back to their respective reefs
        // for (prey, index) in escaped_prey {
        //     if let Some(reef) = self.reefs.get(index) {
        //         reef.borrow_mut().add_prey(prey);
        //     }
        // }

        prey_caught
    }

    // Helper method to determine if the crab can eat the prey.
    // This would depend on the implementation details of your `Crab` and `Prey`.
    fn can_eat(&self, prey: &Box<dyn Prey>) -> bool {
        // Assume `Prey` trait has a method `diet` that returns the diet of the prey.
        prey.diet() == self.diet
    }
    

    /**
     * Returns Some of any recipe from the given cookbook that matches the crab's diet
     * preferences, or None if no such recipe exists.
     *
     * IMPORTANT: you will need to add lifetime parameters to this function. It is
     * up to you to figure out which ones and where. Do not make any other changes
     * to the signature.
     */
    pub fn choose_recipe<'a>(&self, cookbook: &'a Cookbook) -> Option<&'a Recipe> {
        // unimplemented!();
        cookbook.recipes().find(|&recipe| recipe.diet() == self.diet)
    }
}

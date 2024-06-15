// Libs

// Spanwed pokemon
#[derive(Debug)]
pub struct SpawnedPokemon {
    pub id: u16,
    pub is_shiny: bool,
}

impl SpawnedPokemon {
    /**
    A method to create a new SpawnedPokemon.
    */
    pub fn new(id: u16, is_shiny: bool) -> Self {
        Self { id, is_shiny }
    }
}

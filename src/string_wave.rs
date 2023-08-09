use std::vec;

use egui::{emath::RectTransform, *};
//wave on a string. Modeled as a vector of displacement values
pub struct StringWave {
    past_string_pos: Vec<f32>,
    current_string_pos: Vec<f32>,
    num_points: usize,
    pub initial_num_points: usize,
    pub starting_pos_index: usize,
    pub starting_displacement: f32,
    pub c:f32
}


impl StringWave {
    ///Create a new StringWave
    /// num_points: number of points to model the string with
    /// starting_pos_index: index of the starting displacement
    /// starting_displacement: the starting offest value at the starting_pos_index and starting_pos_index + 1
    /// c: wave speed?
    pub fn new(num_points: usize, starting_pos_index: usize,starting_displacement: f32,c:f32) -> Self {
        //string_pos[starting_pos_index + 1] = starting_displacement * 0.51;
        let mut s = StringWave {
            num_points,
            initial_num_points: num_points,
            starting_pos_index,
            past_string_pos: Vec::new(),
            current_string_pos: Vec::new(),
            starting_displacement,
            c
        };
        s.set_initial_conditions();
        s
    }
    //reset wave
    pub fn set_initial_conditions(&mut self){
        self.num_points=self.initial_num_points;
        let mut string_pos = vec![0.0; self.num_points];
        //pluck string
        for i in 0..self.starting_pos_index {
            string_pos[i] = self.starting_displacement*0.9
        }
        self.past_string_pos = string_pos.clone();
        self.current_string_pos = string_pos.clone();
    }
    ///Step the simulation forward one step
    pub fn _update_heat(&mut self) {
        //update string_pos by averaging neighbors
        let mut new_string_pos = vec![0.0; self.num_points];
        for i in 1..self.num_points - 2 {
            new_string_pos[i] = (self.current_string_pos[i - 1] + self.current_string_pos[i + 1]) / 2.0;
        }
        self.current_string_pos = new_string_pos;
    }

    ///Step the simulation forward one step
    pub fn update_wave(&mut self) {
        //update string_pos by averaging neighbors
        let mut new_string_pos = vec![0.0; self.num_points];
        for i in 1..self.num_points - 2 {
            
            new_string_pos[i] = self.c * 
            (self.current_string_pos[i - 1] + self.current_string_pos[i + 1] - 2.0 * self.current_string_pos[i]) +
             2.0 * self.current_string_pos[i] - self.past_string_pos[i];
        }
        self.past_string_pos = self.current_string_pos.clone();
        self.current_string_pos = new_string_pos;
    }

    /// returns points with following properties:
    /// domain: [0, num_points - 1] 
    /// range: [0, starting_displacement]
    pub fn get_points(&self, rect_transform: RectTransform) -> Vec<Pos2> {
        (0..self.num_points)
            .map(|i| {
                let x = i as f32;
                let y = self.current_string_pos[i];
                rect_transform.transform_pos(pos2(x, y))
            })
            .collect()
    }
    /// returns the bounding rect of the string
    /// used to set the size of the canvas
    /// Flips the y axis since egui has y axis going down
    pub fn get_bounding_rect(&self) -> Rect {
        let min_y = -self.starting_displacement;
        let max_y = self.starting_displacement;
        let min_x = 0.0;
        let max_x = self.num_points as f32 - 1.0;
        //flip y axis since egui has y axis going down
        Rect::from_min_max(pos2(min_x, -min_y), pos2(max_x, -max_y))
    }
}

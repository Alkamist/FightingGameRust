pub struct FightingStage {
    grounds: Vec<[f32; 2]>,
    ceilings: Vec<[f32; 2]>,
    left_walls: Vec<[f32; 2]>,
    right_walls: Vec<[f32; 2]>,
}

impl FightingStage {
    pub fn new() -> FightingStage {
        FightingStage{
            grounds: vec![
                [-56.0, -3.5],
                [-39.0, 0.0],
                [39.0, 0.0],
                [56.0, -3.5],
            ],
            ceilings: vec![
            ],
            left_walls: vec![
                [-56.0, -3.5],
                [-56.0, -7.0],
                [-55.0, -8.0],
                [-54.0, -11.0],
                [-53.0, -12.0],
                [-53.0, -27.0],
                [-54.0, -28.0],
                [-54.0, -30.0],
                [-53.0, -31.0],
                [-53.0, -46.0],
                [-54.0, -47.0],
                [-54.0, -100.0],
            ],
            right_walls: vec![
                [56.0, -3.5],
                [56.0, -7.0],
                [55.0, -8.0],
                [54.0, -11.0],
                [53.0, -12.0],
                [53.0, -27.0],
                [54.0, -28.0],
                [54.0, -30.0],
                [53.0, -31.0],
                [53.0, -46.0],
                [54.0, -47.0],
                [54.0, -100.0],
            ],
        }
    }

    pub fn grounds(&self) -> &Vec<[f32; 2]> { &self.grounds }
    pub fn ceilings(&self) -> &Vec<[f32; 2]> { &self.ceilings }
    pub fn left_walls(&self) -> &Vec<[f32; 2]> { &self.left_walls }
    pub fn right_walls(&self) -> &Vec<[f32; 2]> { &self.right_walls }
}

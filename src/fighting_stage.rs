pub struct FightingStage {
    grounds: Vec<Vec<[f64; 2]>>,
    ceilings: Vec<Vec<[f64; 2]>>,
    left_walls: Vec<Vec<[f64; 2]>>,
    right_walls: Vec<Vec<[f64; 2]>>,
    platforms: Vec<Vec<[f64; 2]>>,
}

impl FightingStage {
    pub fn new() -> FightingStage {
        FightingStage{
            grounds: vec![
                vec![
                    [-56.0, -3.5],
                    [-39.0, 0.0],
                    [39.0, 0.0],
                    [56.0, -3.5],
                ],
            ],
            ceilings: vec![
            ],
            left_walls: vec![
                vec![
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
            ],
            right_walls: vec![
                vec![
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
            ],
            platforms: vec![
                vec![
                    [-59.5, 23.45],
                    [-28.0, 23.45],
                ],
                vec![
                    [28.0, 23.45],
                    [59.5, 23.45],
                ],
                vec![
                    [-15.75, 42.0],
                    [15.75, 42.0],
                ],
            ]
        }
    }

    pub fn grounds(&self) -> &Vec<Vec<[f64; 2]>> { &self.grounds }
    pub fn ceilings(&self) -> &Vec<Vec<[f64; 2]>> { &self.ceilings }
    pub fn left_walls(&self) -> &Vec<Vec<[f64; 2]>> { &self.left_walls }
    pub fn right_walls(&self) -> &Vec<Vec<[f64; 2]>> { &self.right_walls }
    pub fn platforms(&self) -> &Vec<Vec<[f64; 2]>> { &self.platforms }
}

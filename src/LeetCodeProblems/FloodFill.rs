//region LeetCode

pub struct Solution{

}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut img = image.clone();
        Self::dfs(&mut img ,sr as usize, sc as usize, image[sr as usize][sc as usize], color);
        img
    }

    fn dfs(image: &mut Vec<Vec<i32>>, sr: usize, sc : usize, src_color : i32, target_color: i32){
        let curr_color = image[sr][sc];
        if curr_color != src_color || curr_color == target_color{
            return;
        }

        image[sr][sc] = target_color;

        //Recursive call direction
        let directions  = [(1,0), (-1,0), (0,1), (0,-1)].iter(); //Down, Up, Right, Left
        for (row_move, col_move) in directions{
            let n_row = sr as isize + row_move;
            let n_col = sc as isize + col_move;

            if n_row >= 0 && n_row < image.len() as isize && n_col >= 0 && n_col < image[0].len() as isize{
                Self::dfs(image, n_row as usize, n_col as usize, src_color, target_color);
            }
        }
      
    }
}

//endregion

//region Driver And Tester
pub struct Driver{
}
impl Driver{

    pub fn run_tests() {
        // Test case 1
        let image1 = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr1 = 1;
        let sc1 = 1;
        let color1 = 2;
        let expected1 = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(Solution::flood_fill(image1, sr1, sc1, color1), expected1);

        // Test case 2
        let image2 = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let sr2 = 0;
        let sc2 = 0;
        let color2 = 0;
        let expected2 = vec![vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::flood_fill(image2, sr2, sc2, color2), expected2);

        // Test case 3
        let image3 = vec![vec![0, 0, 0], vec![0, 1, 1]];
        let sr3 = 1;
        let sc3 = 1;
        let color3 = 1;
        let expected3 = vec![vec![0, 0, 0], vec![0, 1, 1]];
        assert_eq!(Solution::flood_fill(image3, sr3, sc3, color3), expected3);

        // Test case 4
        let image4 = vec![vec![1, 2, 2], vec![2, 3, 3], vec![2, 2, 1]];
        let sr4 = 0;
        let sc4 = 0;
        let color4 = 3;
        let expected4 = vec![vec![3, 2, 2], vec![2, 3, 3], vec![2, 2, 1]];
        assert_eq!(Solution::flood_fill(image4, sr4, sc4, color4), expected4);

        // Test case 5
        let image5 = vec![vec![0]];
        let sr5 = 0;
        let sc5 = 0;
        let color5 = 1;
        let expected5 = vec![vec![1]];
        assert_eq!(Solution::flood_fill(image5, sr5, sc5, color5), expected5);

        println!("FloodFill :: All tests passed!");
    }
}
//endregion
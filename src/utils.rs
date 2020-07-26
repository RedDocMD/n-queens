pub fn is_in_line(pos1: &Position, pos2: &Position) -> bool {
  let x1 = pos1.0 as isize;
  let x2 = pos2.0 as isize;
  let y1 = pos1.1 as isize;
  let y2 = pos2.1 as isize;

  x1 == x2 || y1 == y2 || (x1 - x2).abs() == (y1 - y2).abs()
}

pub type Position = (usize, usize);
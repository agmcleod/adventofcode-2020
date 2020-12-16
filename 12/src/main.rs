use std::f32::consts::PI;

use read_input::read_text;

fn rotate_vec(vec: &mut (i32, i32), deg: i32) {
    let rad = (deg as f32) * PI / 180.0;
    let x = vec.0 as f32;
    let y = vec.1 as f32;
    vec.0 = (x * rad.cos() - y * rad.sin()).round() as i32;
    vec.1 = (x * rad.sin() + y * rad.cos()).round() as i32;
}

fn main() {
    let text = read_text("12/input.txt").unwrap();

    let mut ship_pos = (0, 0);
    let mut ship_dir = (1, 0);

    for line in text.lines() {
        let mut iter = line.chars();
        let cmd = iter.next().unwrap();
        let n: i32 = iter.collect::<String>().parse().unwrap();

        // apply_command(cmd, &mut ship_pos, n, &mut ship_dir, &mut ship_pos);

        match cmd {
            'F' => {
                ship_pos.0 += ship_dir.0 * n;
                ship_pos.1 += ship_dir.1 * n;
            }
            'N' => {
                ship_pos.1 -= n;
            }
            'E' => {
                ship_pos.0 += n;
            }
            'S' => {
                ship_pos.1 += n;
            }
            'W' => {
                ship_pos.0 -= n;
            }
            'L' => {
                rotate_vec(&mut ship_dir, -1 * n);
            }
            'R' => {
                rotate_vec(&mut ship_dir, n);
            }
            _ => {
                panic!("Unmatched cmd {}", cmd);
            }
        }
    }

    println!("{}", ship_pos.0.abs() + ship_pos.1.abs());

    // p2
    ship_pos.0 = 0;
    ship_pos.1 = 0;

    let mut beacon = (10, -1);
    for line in text.lines() {
        let mut iter = line.chars();
        let cmd = iter.next().unwrap();
        let n: i32 = iter.collect::<String>().parse().unwrap();

        match cmd {
            'F' => {
                ship_pos.0 += beacon.0 * n;
                ship_pos.1 += beacon.1 * n;
            }
            'N' => {
                beacon.1 -= n;
            }
            'E' => {
                beacon.0 += n;
            }
            'S' => {
                beacon.1 += n;
            }
            'W' => {
                beacon.0 -= n;
            }
            'L' => {
                rotate_vec(&mut beacon, -1 * n);
            }
            'R' => {
                rotate_vec(&mut beacon, n);
            }
            _ => {
                panic!("Unmatched cmd {}", cmd);
            }
        }
    }

    println!("{}", ship_pos.0.abs() + ship_pos.1.abs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotations() {
        let mut ship_dir = (1, 0);

        rotate_vec(&mut ship_dir, 90);
        assert_eq!(ship_dir, (0, 1));

        rotate_vec(&mut ship_dir, -90);
        assert_eq!(ship_dir, (1, 0));

        rotate_vec(&mut ship_dir, -180);
        assert_eq!(ship_dir, (-1, 0));

        rotate_vec(&mut ship_dir, 270);
        assert_eq!(ship_dir, (0, 1));

        let mut pos = (10, -4);
        rotate_vec(&mut pos, 90);
        assert_eq!(pos, (4, 10));
    }
}

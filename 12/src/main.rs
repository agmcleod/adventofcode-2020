use std::f32::consts::PI;

use read_input::read_text;

struct Ship {
    pos: (i32, i32),
    direction: (i32, i32),
}

impl Ship {
    fn rotate(&mut self, deg: i32) {
        let rad = (deg as f32) * PI / 180.0;
        let (x, y) = self.direction;
        let x = x as f32;
        let y = y as f32;
        self.direction = (
            (x as f32 * rad.cos() - y * rad.sin()) as i32,
            (x as f32 * rad.sin() + y * rad.cos()) as i32,
        );
    }
}

fn main() {
    let text = read_text("12/input.txt").unwrap();

    let mut ship = Ship {
        pos: (0, 0),
        direction: (1, 0),
    };

    // vec.x * cos(ang) - vec.y * sin(ang),
    // vec.x * sin(ang) + vec.y * cos(ang)

    for line in text.lines() {
        let mut iter = line.chars();
        let cmd = iter.next().unwrap();
        let n: i32 = iter.collect::<String>().parse().unwrap();

        match cmd {
            'F' => {
                ship.pos.0 += ship.direction.0 * n;
                ship.pos.1 += ship.direction.1 * n;
            }
            'N' => {
                ship.pos.1 -= n;
            }
            'E' => {
                ship.pos.0 += n;
            }
            'S' => {
                ship.pos.1 += n;
            }
            'W' => {
                ship.pos.0 -= n;
            }
            'L' => {
                ship.rotate(-1 * n);
            }
            'R' => {
                ship.rotate(n);
            }
            _ => {
                panic!("Unmatched cmd {}", cmd);
            }
        }
    }

    println!("{}", ship.pos.0.abs() + ship.pos.1.abs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotations() {
        let mut ship = Ship {
            pos: (0, 0),
            direction: (1, 0),
        };

        ship.rotate(90);
        assert_eq!(ship.direction, (0, 1));

        ship.rotate(-90);
        assert_eq!(ship.direction, (1, 0));

        ship.rotate(-180);
        assert_eq!(ship.direction, (-1, 0));

        ship.rotate(270);
        assert_eq!(ship.direction, (0, 1));
    }
}

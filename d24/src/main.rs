use glam::{DVec2, DVec3};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

trait XYDVec {
    fn xy(&self) -> DVec2;
}

#[derive(Clone)]
struct Hailstone {
    origin: DVec3,
    velocity: DVec3,
}

#[derive(Debug, Clone, PartialEq)]
struct Intersection {
    position: DVec3,
}

impl Hailstone {
    fn parse(input: &str) -> nom::IResult<&str, Hailstone> {
        let (input, (position, velocity)) = separated_pair(
            separated_list1(
                tag(", "),
                preceded(
                    multispace0,
                    map_res(recognize(preceded(opt(tag("-")), digit1)), |d: &str| {
                        d.parse()
                    }),
                ),
            ),
            tag(" @ "),
            separated_list1(
                tag(", "),
                preceded(
                    multispace0,
                    map_res(recognize(preceded(opt(tag("-")), digit1)), |d: &str| {
                        d.parse()
                    }),
                ),
            ),
        )(input)?;
        Ok((
            input,
            Hailstone {
                origin: DVec3::from((position[0], position[1], position[2])),
                velocity: DVec3::from((velocity[0], velocity[1], velocity[2])),
            },
        ))
    }

    fn position_at_time(&self, time: f64) -> DVec3 {
        self.origin + self.velocity * time
    }

    fn intersection_xy(&self, other: &Hailstone) -> Option<Intersection> {
        let cross_product = self.velocity.x * other.velocity.y - self.velocity.y * other.velocity.x;
        if cross_product < 1e-6 {
            return None; // rays are parallel
        }

        let t1 = (other.origin.x - self.origin.x) * other.velocity.y
            - (other.origin.y - self.origin.y) * other.velocity.x / cross_product;

        let t2 = (other.origin.x - self.origin.x) * self.velocity.y
            - (other.origin.y - self.origin.y) * self.velocity.x / cross_product;

        if t1 >= 0.0 && t2 >= 0.0 {
            // Calculate the intersection point using the parameter value
            let intersection_point = self.position_at_time(t1);
            Some(Intersection {
                position: intersection_point,
            })
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hailstone {{ position: {:?}, velocity: {:?} }}",
            self.origin, self.velocity
        )
    }
}

impl XYDVec for DVec3 {
    fn xy(&self) -> DVec2 {
        DVec2::from((self.x, self.y))
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    let (_, hailstones) = separated_list1(line_ending, Hailstone::parse)(input).unwrap();
    hailstones
}

fn part1(input: &str) -> usize {
    let hailstones = parse(input);
    number_of_collisions_within_bounds(
        hailstones,
        (
            DVec2::new(200000000000000.0, 200000000000000.0),
            DVec2::new(400000000000000.0, 400000000000000.0),
        ),
    )
}

fn number_of_collisions_within_bounds(hailstones: Vec<Hailstone>, bounds: (DVec2, DVec2)) -> usize {
    let intersections = hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, hailstone_a)| {
            hailstones
                .iter()
                .enumerate()
                .filter_map(move |(j, hailstone_b)| {
                    if i == j {
                        None
                    } else {
                        hailstone_a.intersection_xy(hailstone_b)
                    }
                })
        })
        .collect::<Vec<_>>();
    find_intersections_within_bounds(intersections, bounds).len()
}

fn find_intersections_within_bounds(
    intersections: Vec<Intersection>,
    bounds: (DVec2, DVec2),
) -> Vec<Intersection> {
    intersections
        .iter()
        .filter(|intersection| {
            let position = intersection.position.xy();
            position.x >= bounds.0.x
                && position.x <= bounds.1.x
                && position.y >= bounds.0.y
                && position.y <= bounds.1.y
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    #[test]
    fn test_number_of_collisions_within_bounds() {
        let hailstones = parse(EXAMPLE);
        assert_eq!(
            number_of_collisions_within_bounds(
                hailstones,
                (DVec2::new(7.0, 7.0), DVec2::new(27.0, 27.0),),
            ),
            2
        );
    }
}

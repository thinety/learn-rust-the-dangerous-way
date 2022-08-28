struct Body {
    position: [f64; 3],
    velocity: [f64; 3],
    mass: f64,
}

const BODIES_COUNT: usize = 5;

const SOLAR_MASS: f64 = 4.0 * std::f64::consts::PI * std::f64::consts::PI;
const DAYS_PER_YEAR: f64 = 365.24;

const INTERACTIONS: usize = BODIES_COUNT * (BODIES_COUNT - 1) / 2;

const INITIAL_STATE: [Body; BODIES_COUNT] = [
    Body {
        position: [0.0; 3],
        velocity: [0.0; 3],
        mass: SOLAR_MASS,
    },
    Body {
        position: [
            4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01,
        ],
        velocity: [
            1.66007664274403694e-03 * DAYS_PER_YEAR,
            7.69901118419740425e-03 * DAYS_PER_YEAR,
            -6.90460016972063023e-05 * DAYS_PER_YEAR,
        ],
        mass: 9.54791938424326609e-04 * SOLAR_MASS,
    },
    Body {
        position: [
            8.34336671824457987e+00,
            4.12479856412430479e+00,
            -4.03523417114321381e-01,
        ],
        velocity: [
            -2.76742510726862411e-03 * DAYS_PER_YEAR,
            4.99852801234917238e-03 * DAYS_PER_YEAR,
            2.30417297573763929e-05 * DAYS_PER_YEAR,
        ],
        mass: 2.85885980666130812e-04 * SOLAR_MASS,
    },
    Body {
        position: [
            1.28943695621391310e+01,
            -1.51111514016986312e+01,
            -2.23307578892655734e-01,
        ],
        velocity: [
            2.96460137564761618e-03 * DAYS_PER_YEAR,
            2.37847173959480950e-03 * DAYS_PER_YEAR,
            -2.96589568540237556e-05 * DAYS_PER_YEAR,
        ],
        mass: 4.36624404335156298e-05 * SOLAR_MASS,
    },
    Body {
        position: [
            1.53796971148509165e+01,
            -2.59193146099879641e+01,
            1.79258772950371181e-01,
        ],
        velocity: [
            2.68067772490389322e-03 * DAYS_PER_YEAR,
            1.62824170038242295e-03 * DAYS_PER_YEAR,
            -9.51592254519715870e-05 * DAYS_PER_YEAR,
        ],
        mass: 5.15138902046611451e-05 * SOLAR_MASS,
    },
];

fn advance(bodies: &mut [Body; BODIES_COUNT]) {
    let position_deltas = {
        let mut position_deltas = [[0.0; 3]; INTERACTIONS];

        let mut k = 0;
        for i in 0..BODIES_COUNT - 1 {
            for j in i + 1..BODIES_COUNT {
                for m in 0..3 {
                    position_deltas[k][m] = bodies[i].position[m] - bodies[j].position[m];
                }
                k += 1;
            }
        }

        position_deltas
    };

    let magnitudes = {
        let mut magnitudes = [0.0; INTERACTIONS];

        for (i, magnitude) in magnitudes.iter_mut().enumerate() {
            let distance = (position_deltas[i][0].powi(2)
                + position_deltas[i][1].powi(2)
                + position_deltas[i][2].powi(2))
            .sqrt();
            *magnitude = 0.01 / distance.powi(3);
        }

        magnitudes
    };

    {
        let mut k = 0;
        for i in 0..BODIES_COUNT - 1 {
            for j in i + 1..BODIES_COUNT {
                let i_mass_magnitude = bodies[i].mass * magnitudes[k];
                let j_mass_magnitude = bodies[j].mass * magnitudes[k];
                for m in 0..3 {
                    bodies[i].velocity[m] -= position_deltas[k][m] * j_mass_magnitude;
                    bodies[j].velocity[m] += position_deltas[k][m] * i_mass_magnitude;
                }
                k += 1;
            }
        }
    }

    for body in bodies {
        for m in 0..3 {
            body.position[m] += 0.01 * body.velocity[m];
        }
    }
}

fn offset_momentum(bodies: &mut [Body; BODIES_COUNT]) {
    let (sun, planets) = bodies.split_first_mut().unwrap();
    for planet in planets {
        for m in 0..3 {
            sun.velocity[m] -= planet.velocity[m] * planet.mass / SOLAR_MASS;
        }
    }
}

fn output_energy(bodies: &mut [Body; BODIES_COUNT]) {
    let mut energy = 0.0;

    for (i, body) in bodies.iter().enumerate() {
        energy += 0.5
            * body.mass
            * (body.velocity[0].powi(2) + body.velocity[1].powi(2) + body.velocity[2].powi(2));

        for other_body in &bodies[i + 1..] {
            energy -= body.mass * other_body.mass
                / ((body.position[0] - other_body.position[0]).powi(2)
                    + (body.position[1] - other_body.position[1]).powi(2)
                    + (body.position[2] - other_body.position[2]).powi(2))
                .sqrt();
        }
    }

    println!("{:.9}", energy);
}

fn main() {
    let c = std::env::args().nth(1).unwrap().parse().unwrap();

    let mut solar_bodies = INITIAL_STATE;

    offset_momentum(&mut solar_bodies);
    output_energy(&mut solar_bodies);
    for _ in 0..c {
        advance(&mut solar_bodies);
    }
    output_energy(&mut solar_bodies);
}

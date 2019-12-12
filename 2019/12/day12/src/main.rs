use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
struct Moon
{
    // position
    x : isize,
    y : isize,
    z : isize,

    // velocity
    vel_x : isize,
    vel_y : isize,
    vel_z : isize,
}

impl Moon
{
    fn apply_gravity(&mut self, other_moon: &mut Moon)
    {
        // compare each axis
        // X
        match self.x.cmp(&other_moon.x)
        {
            Ordering::Less =>
            {
                self.vel_x += 1;
                // other_moon.vel_x -= 1;
            },
            Ordering::Equal =>
            {
                // do nothing
            },
            Ordering::Greater =>
            {
                self.vel_x -= 1;
                // other_moon.vel_x += 1;
            }
        }

        // Y
        match self.y.cmp(&other_moon.y)
        {
            Ordering::Less =>
            {
                self.vel_y += 1;
                // other_moon.vel_y -= 1;
            },
            Ordering::Equal =>
            {
                // do nothing
            },
            Ordering::Greater =>
            {
                self.vel_y -= 1;
                // other_moon.vel_y += 1;
            }
        }

        // Z
        match self.z.cmp(&other_moon.z)
        {
            Ordering::Less =>
            {
                self.vel_z += 1;
                // other_moon.vel_z -= 1;
            },
            Ordering::Equal =>
            {
                // do nothing
            },
            Ordering::Greater =>
            {
                self.vel_z -= 1;
                // other_moon.vel_z += 1;
            }
        }
    }

    fn apply_velocity(&mut self)
    {
        self.x += self.vel_x;
        self.y += self.vel_y;
        self.z += self.vel_z;
    }

    fn get_energy(&mut self) -> isize
    {
        self.get_kinetic_energy() * self.get_potential_energy()
    }

    fn get_potential_energy(&mut self) -> isize
    {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn get_kinetic_energy(&mut self) -> isize
    {
        self.vel_x.abs() + self.vel_y.abs() + self.vel_z.abs()
    }
}

fn main() {
    // example();
    // solution();
    solution_part2();
    //example_part2();
}

fn solution_part2()
{
    let mut moons = vec![
        Moon { x: 17, y: 5, z: 1, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: -2, y: -8, z: 8, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 7, y: -6, z: 14, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 1, y: -10, z: 4, vel_x: 0,  vel_y: 0, vel_z: 0 },
    ];

    let moons_initial = vec![
        Moon { x: 17, y: 5, z: 1, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: -2, y: -8, z: 8, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 7, y: -6, z: 14, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 1, y: -10, z: 4, vel_x: 0,  vel_y: 0, vel_z: 0 },
    ];

    // strategy for this will be to 
    // find the period in which moon 0 x vel_x is the same
    // then use that in subsequent iterations to find repetition

    // 4686774924
    // 582622237229761

    for iter in (1..0xFFFFFFFFFFFFFFFFu64)
    {
        println!("iter {}", iter);
        if iter > 582622237229761
        {
            println!("past solution");
            break;
        }

        // apply gravity
        for idx in 0..moons.len()
        {
            // could optimize this so that it doesn't do N^2 moons
            // but for some reason the ref to moons[index] wasn't working
            // and the m2 would not be updated
            // for idx2 in (idx+1)..moons.len()
            for idx2 in 0..moons.len()
            {
                // if idx2 <= idx
                if idx2 == idx
                {
                    continue;
                }
                // println!("moon 2 before {:?}", moons[idx2]);
                // println!("applying grav to {} and {}", idx, idx2);
                let mut m2 = moons[idx2];
                moons[idx].apply_gravity(&mut m2);
                // println!("moon 2 after {:?}", moons[idx2]);
            }
        }

        let mut dimensions = 0;
        let mut x_dim = 0;
        let mut y_dim = 0;
        let mut z_dim = 0;

        // apply velocity
        for idx in 0..moons.len()
        {
            moons[idx].apply_velocity();
            // println!("moon {} {:?}", idx, moons[idx]);

            // find similarity to the initial state
            let current = moons[idx];
            let initial_moon = moons_initial[idx];
            let mut current_dim = 0;

            if current.x == initial_moon.x && current.vel_x == initial_moon.vel_x
            {
                current_dim += 1;
                dimensions += 1;
                x_dim += 1;
                println!("idx {} match x after {}", idx, iter);
            }

            if current.y == initial_moon.y && current.vel_y == initial_moon.vel_y
            {
                current_dim += 1;
                dimensions += 1;
                y_dim += 1;
                println!("idx {} match y after {}", idx, iter);
            }

            if current.z == initial_moon.z && current.vel_z == initial_moon.vel_z
            {
                current_dim += 1;
                dimensions += 1;
                z_dim += 1;
                println!("idx {} match z after {}", idx, iter);
            }

            if current_dim == 3
            {
                println!("current dim is {}", current_dim);
                break;
            }
        }

        if dimensions >= 12
        {
            println!("solution: {}", iter);
            break;
        }

        // x repeats after 2028, 4056

        // if dimensions >= 4
        if x_dim >= 4
        {
            println!("found {} matching x", x_dim);
            // break;
            // 286332
        }

        // y repeats after 5898

        if y_dim >= 4
        {
            println!("found {} matching y", y_dim);
            // break;
            // 167624
        }

        // 4702

        if z_dim >= 4
        {
            println!("found {} matching z", z_dim);
            break;
            // 102356
            // 43
        }

        // once each of the matches for x y and z were found
        // find the least common multiple of all of these
        // (using wolfram alpha)
        // least common multiple 286332, 167624, 102356
    }
}

fn example_part2()
{
    let mut moons = vec![
        // Moon { x: -8, y: -10, z: 0, vel_x: 0,  vel_y: 0, vel_z: 0 },
        // Moon { x: 5, y: 5, z: 10, vel_x: 0,  vel_y: 0, vel_z: 0 },
        // Moon { x: 2, y: -7, z: 3, vel_x: 0,  vel_y: 0, vel_z: 0 },
        // Moon { x: 9, y: -8, z: -3, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: -1, y: 0, z: 2, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 2, y: -10, z: -7, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 4, y: -8, z: 8, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 3, y: 5, z: -1, vel_x: 0,  vel_y: 0, vel_z: 0 },
    ];

    let moons_initial = vec![
        // Moon { x: -8, y: -10, z: 0, vel_x: 0,  vel_y: 0, vel_z: 0 },
        // Moon { x: 5, y: 5, z: 10, vel_x: 0,  vel_y: 0, vel_z: 0 },
        // Moon { x: 2, y: -7, z: 3, vel_x: 0,  vel_y: 0, vel_z: 0 },
        // Moon { x: 9, y: -8, z: -3, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: -1, y: 0, z: 2, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 2, y: -10, z: -7, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 4, y: -8, z: 8, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 3, y: 5, z: -1, vel_x: 0,  vel_y: 0, vel_z: 0 },
    ];

    // strategy for this will be to 
    // find the period in which moon 0 x vel_x is the same
    // then use that in subsequent iterations to find repetition

    // 4686774924
    // 582622237229761

    // for iter in (1..0xFFFFFFFFFFFFFFFFu64).step_by(4702)
    for iter in (0..9999u64)
    {
        println!("iter {}", iter);
        if iter > 2800
        {
            println!("past solution");
            break;
        }

        // println!("iteration {}", iter);
        // apply gravity
        for idx in 0..moons.len()
        {
            // could optimize this so that it doesn't do N^2 moons
            // but for some reason the ref to moons[index] wasn't working
            // and the m2 would not be updated
            // for idx2 in (idx+1)..moons.len()
            for idx2 in 0..moons.len()
            {
                // if idx2 <= idx
                if idx2 == idx
                {
                    continue;
                }
                // println!("moon 2 before {:?}", moons[idx2]);
                // println!("applying grav to {} and {}", idx, idx2);
                let mut m2 = moons[idx2];
                moons[idx].apply_gravity(&mut m2);
                // println!("moon 2 after {:?}", moons[idx2]);
            }
        }

        let mut dimensions = 0;
        let mut x_dim = 0;
        let mut y_dim = 0;
        let mut z_dim = 0;

        // apply velocity
        for idx in 0..moons.len()
        {
            moons[idx].apply_velocity();
            // println!("moon {} {:?}", idx, moons[idx]);

            // find similarity to the initial state
            let current = moons[idx];
            let initial_moon = moons_initial[idx];

            if current.x == initial_moon.x && current.vel_x == initial_moon.vel_x
            {
                dimensions += 1;
                x_dim += 1;
                println!("idx {} match x after {}", idx, iter);
            }

            if current.y == initial_moon.y && current.vel_y == initial_moon.vel_y
            {
                dimensions += 1;
                y_dim += 1;
                println!("idx {} match y after {}", idx, iter);
            }

            if current.z == initial_moon.z && current.vel_z == initial_moon.vel_z
            {
                dimensions += 1;
                z_dim += 1;
                println!("idx {} match z after {}", idx, iter);
            }
        }

        if dimensions >= 12
        {
            println!("solution: {}", iter);
            break;
        }

        // x repeats after 2028, 4056

        // if dimensions >= 4
        if x_dim >= 4
        {
            println!("found {} matching x", x_dim);
            // break;
            // 17
        }

        // y repeats after 5898

        if y_dim >= 4
        {
            println!("found {} matching y", y_dim);
            // break;
            // 27
        }

        // 4702

        if z_dim >= 4
        {
            println!("found {} matching z", z_dim);
            // break;
            // 43
        }
    }
}

fn solution()
{
    let mut moons = vec![
        Moon { x: 17, y: 5, z: 1, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: -2, y: -8, z: 8, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 7, y: -6, z: 14, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 1, y: -10, z: 4, vel_x: 0,  vel_y: 0, vel_z: 0 },
    ];

    for iter in 1..1001
    {
        println!("iteration {}", iter);
        // apply gravity
        for idx in 0..moons.len()
        {
            // could optimize this so that it doesn't do N^2 moons
            // but for some reason the ref to moons[index] wasn't working
            // and the m2 would not be updated
            // for idx2 in (idx+1)..moons.len()
            for idx2 in 0..moons.len()
            {
                // if idx2 <= idx
                if idx2 == idx
                {
                    continue;
                }
                // println!("moon 2 before {:?}", moons[idx2]);
                // println!("applying grav to {} and {}", idx, idx2);
                let mut m2 = moons[idx2];
                moons[idx].apply_gravity(&mut m2);
                // println!("moon 2 after {:?}", moons[idx2]);
            }
        }

        let mut energy = 0;
        // apply velocity
        for idx in 0..moons.len()
        {
            moons[idx].apply_velocity();
            energy += moons[idx].get_energy();
            println!("moon {} {:?}", idx, moons[idx]);
        }
        println!("total energy: {}", energy);
    }
}

fn example()
{
    let mut moons = vec![
        Moon { x: -1, y: 0, z: 2, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 2, y: -10, z: -7, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 4, y: -8, z: 8, vel_x: 0,  vel_y: 0, vel_z: 0 },
        Moon { x: 3, y: 5, z: -1, vel_x: 0,  vel_y: 0, vel_z: 0 },
    ];

    for iter in 1..11
    {
        println!("iteration {}", iter);
        // apply gravity
        for idx in 0..moons.len()
        {
            // could optimize this so that it doesn't do N^2 moons
            // but for some reason the ref to moons[index] wasn't working
            // and the m2 would not be updated
            // for idx2 in (idx+1)..moons.len()
            for idx2 in 0..moons.len()
            {
                // if idx2 <= idx
                if idx2 == idx
                {
                    continue;
                }
                // println!("moon 2 before {:?}", moons[idx2]);
                // println!("applying grav to {} and {}", idx, idx2);
                let mut m2 = moons[idx2];
                moons[idx].apply_gravity(&mut m2);
                // println!("moon 2 after {:?}", moons[idx2]);
            }
        }

        let mut energy = 0;
        // apply velocity
        for idx in 0..moons.len()
        {
            moons[idx].apply_velocity();
            energy += moons[idx].get_energy();
            println!("moon {} {:?}", idx, moons[idx]);
        }
        println!("total energy: {}", energy);
    }
}
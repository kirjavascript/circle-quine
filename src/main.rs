fn main(){let q:&[u8]=&[

80,82,73,78,84,76,78,01,08,02,70,78,00,77,65,73,78,08,09,91,91,76,69,84,00,81,26,06,59,85,24,61,29,06,59,02,09,27,00,76,69,84,00,65,82,69,65,00,29,00,08,81,14,76,69,78,08,09,00,10,19,09,27,00,76,69,84,00,82,65,68,73,85,83,00,29,00,08,65,82,69,65,00,65,83,00,70,22,20,00,15,00,83,84,68,26,26,70,22,20,26,26,67,79,78,83,84,83,26,26,48,41,09,14,83,81,82,84,08,09,00,65,83,00,73,19,18,27,00,76,69,84,00,77,85,84,00,66,79,85,78,68,83,26,00,54,69,67,28,08,73,19,18,12,00,73,19,18,09,30,00,29,00,54,69,67,26,26,78,69,87,08,09,27,00,76,69,84,00,88,67,00,29,00,82,65,68,73,85,83,27,00,76,69,84,00,89,67,00,29,00,82,65,68,73,85,83,27,00,76,69,84,00,82,88,00,29,00,82,65,68,73,85,83,27,00,76,69,84,00,82,89,00,29,00,82,65,68,73,85,83,27,00,76,69,84,00,77,85,84,00,88,00,29,00,16,27,00,76,69,84,00,77,85,84,00,89,00,29,00,82,89,27,00,76,69,84,00,77,85,84,00,80,00,29,00,08,82,89,10,82,89,09,13,08,82,88,10,82,88,10,82,89,09,11,08,08,82,88,10,82,88,09,15,20,09,27,00,87,72,73,76,69,08,08,18,10,88,10,82,89,10,82,89,09,28,08,18,10,89,10,82,88,10,82,88,09,09,00,91,00,66,79,85,78,68,83,14,80,85,83,72,08,08,88,67,11,88,12,89,67,13,89,09,09,27,00,66,79,85,78,68,83,14,80,85,83,72,08,08,88,67,13,88,12,89,67,11,89,09,09,27,00,66,79,85,78,68,83,14,80,85,83,72,08,08,88,67,11,88,12,89,67,11,89,09,09,27,00,66,79,85,78,68,83,14,80,85,83,72,08,08,88,67,13,88,12,89,67,13,89,09,09,27,00,73,70,08,80,28,16,09,00,91,00,88,29,88,11,17,27,00,80,29,80,11,08,18,10,82,89,10,82,89,10,88,09,11,08,82,89,10,82,89,09,27,00,93,00,69,76,83,69,00,91,00,88,29,88,11,17,27,00,89,29,89,13,17,27,00,80,29,80,11,08,18,10,82,89,10,82,89,10,88,11,82,89,10,82,89,09,13,08,18,10,82,88,10,82,88,10,89,09,27,00,93,00,93,00,80,29,08,08,88,00,65,83,00,70,22,20,11,16,14,21,09,10,08,88,00,65,83,00,70,22,20,11,16,14,21,09,09,00,65,83,00,73,19,18,00,10,82,89,10,82,89,11,08,89,13,17,09,10,08,89,13,17,09,10,82,88,10,82,88,13,82,88,10,82,88,10,82,89,10,82,89,27,00,87,72,73,76,69,08,89,30,29,16,09,00,91,00,66,79,85,78,68,83,14,80,85,83,72,08,08,88,67,11,88,12,89,67,13,89,09,09,27,00,66,79,85,78,68,83,14,80,85,83,72,08,08,88,67,13,88,12,89,67,11,89,09,09,27,00,66,79,85,78,68,83,14,80,85,83,72,08,08,88,67,11,88,12,89,67,11,89,09,09,27,00,66,79,85,78,68,83,14,80,85,83,72,08,08,88,67,13,88,12,89,67,13,89,09,09,27,00,73,70,08,80,30,16,09,00,91,00,89,29,89,13,17,27,00,80,29,80,13,08,18,10,82,88,10,82,88,10,89,09,11,08,82,88,10,82,88,09,27,00,93,00,69,76,83,69,00,91,00,89,29,89,13,17,27,00,88,29,88,11,17,27,00,80,29,80,11,08,18,10,82,89,10,82,89,10,88,09,13,08,18,10,82,88,10,82,88,10,89,09,13,08,82,88,10,82,88,09,27,00,93,00,93,00,76,69,84,00,77,85,84,00,84,79,84,65,76,00,29,00,16,27,00,76,69,84,00,77,85,84,00,84,00,29,00,81,14,73,84,69,82,08,09,14,67,76,79,78,69,68,08,09,27,00,70,79,82,00,89,00,73,78,00,17,14,14,82,65,68,73,85,83,10,18,00,91,00,76,69,84,00,76,00,29,00,66,79,85,78,68,83,14,73,84,69,82,08,09,14,70,73,76,84,69,82,08,92,88,92,00,88,14,17,00,29,29,00,89,09,14,77,65,80,08,92,88,92,00,88,14,16,09,14,67,79,76,76,69,67,84,26,26,28,54,69,67,28,73,19,18,30,30,08,09,27,00,76,69,84,00,77,65,88,00,29,00,76,14,73,84,69,82,08,09,14,67,76,79,78,69,68,08,09,14,70,79,76,68,08,16,12,00,73,19,18,26,26,77,65,88,09,27,00,76,69,84,00,77,73,78,00,29,00,76,14,73,84,69,82,08,09,14,67,76,79,78,69,68,08,09,14,70,79,76,68,08,17,16,16,16,12,00,73,19,18,26,26,77,73,78,09,27,00,80,82,73,78,84,76,78,01,08,02,91,93,00,91,93,00,91,93,02,12,00,77,65,88,12,00,77,73,78,12,00,89,09,27,00,76,69,84,00,78,85,77,00,29,00,08,77,65,88,00,13,00,77,73,78,09,27,00,84,79,84,65,76,00,11,29,00,78,85,77,27,00,80,82,73,78,84,01,08,02,91,26,16,20,93,00,00,02,12,00,78,85,77,09,27,00,70,79,82,00,63,00,73,78,00,16,14,14,77,73,78,00,91,00,80,82,73,78,84,01,08,02,00,02,09,27,00,93,00,70,79,82,00,63,00,73,78,00,16,14,14,08,78,85,77,15,19,09,00,91,00,80,82,73,78,84,01,08,02,91,26,16,18,93,12,02,12,00,84,14,78,69,88,84,08,09,14,85,78,87,82,65,80,08,09,09,27,00,93,00,80,82,73,78,84,01,08,02,60,78,02,09,27,00,93,

];

    println!("fn main(){{let q:&[u8]=&[");
    let area = q.len();
    let radius = (area as f64 / std::f64::consts::PI).sqrt() as i32 + 1;
    let mut bounds: Vec<(i32, i32)> = Vec::new();


    let mut x0 = radius;
    let mut y0 = radius;
    let mut x = radius-1;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 1;
    let mut err = dx - (radius << 1);
    while (x >= y)
    {
        bounds.push((x0 + x, y0 + y));
        bounds.push((x0 + y, y0 + x));
        bounds.push((x0 - y, y0 + x));
        bounds.push((x0 - x, y0 + y));
        bounds.push((x0 - x, y0 - y));
        bounds.push((x0 - y, y0 - x));
        bounds.push((x0 + y, y0 - x));
        bounds.push((x0 + x, y0 - y));

        if (err <= 0)
        {
            y += 1;
            err += dy;
            dy += 2;
        }

        if (err > 0)
        {
            x -= 1;
            dx += 2;
            err += dx - (radius << 1);
        }
    }
    let mut total = 0;
    let mut t = q.iter().cloned();
    for y in 1..radius*2 {
        let l = bounds.iter().filter(|x| x.1 == y).map(|x| x.0).collect::<Vec<i32>>();
        let max = l.iter().cloned().fold(0, i32::max);
        let min = l.iter().cloned().fold(1000, i32::min);
        // println!("{} {} {}", max, min, y);
        let num = (max - min);
        total += num;
        print!("{:04}  ", num);
        for _ in 0..min {
            print!("   ");
        }
        for _ in 0..(num) {
            if let Some(v) = t.next() {
                print!("{:02},", v);
            }
        }
        print!("\n");
    }
    while let Some(v) = t.next() {
        print!("{:02},", v);
    }
    println!("\n{:#?}", q.len()*3);
    println!("{:#?}", total);

    // once you have the bounds, subtract xstart from xend to get number of elements to paste

    for n in q.iter() {
        print!("{}", (*n + 32) as char);
    }
    print!("\n");

    // encode();
}


fn encode() {
    let s = String::from(r###"println!("fn main(){{let q:&[u8]=&["); let area = (q.len() *3); let radius = (area as f64 / std::f64::consts::PI).sqrt() as i32; let mut bounds: Vec<(i32, i32)> = Vec::new(); let xc = radius; let yc = radius; let rx = radius; let ry = radius; let mut x = 0; let mut y = ry; let mut p = (ry*ry)-(rx*rx*ry)+((rx*rx)/4); while((2*x*ry*ry)<(2*y*rx*rx)) { bounds.push((xc+x,yc-y)); bounds.push((xc-x,yc+y)); bounds.push((xc+x,yc+y)); bounds.push((xc-x,yc-y)); if(p<0) { x=x+1; p=p+(2*ry*ry*x)+(ry*ry); } else { x=x+1; y=y-1; p=p+(2*ry*ry*x+ry*ry)-(2*rx*rx*y); } } p=((x as f64+0.5)*(x as f64+0.5)) as i32 *ry*ry+(y-1)*(y-1)*rx*rx-rx*rx*ry*ry; while(y>=0) { bounds.push((xc+x,yc-y)); bounds.push((xc-x,yc+y)); bounds.push((xc+x,yc+y)); bounds.push((xc-x,yc-y)); if(p>0) { y=y-1; p=p-(2*rx*rx*y)+(rx*rx); } else { y=y-1; x=x+1; p=p+(2*ry*ry*x)-(2*rx*rx*y)-(rx*rx); } } let mut total = 0; let mut t = q.iter().cloned(); for y in 1..radius*2 { let l = bounds.iter().filter(|x| x.1 == y).map(|x| x.0).collect::<Vec<i32>>(); let max = l.iter().cloned().fold(0, i32::max); let min = l.iter().cloned().fold(1000, i32::min); println!("{} {} {}", max, min, y); let num = (max - min); total += num; print!("{:04}  ", num); for _ in 0..min { print!(" "); } for _ in 0..(num/3) { print!("{:02},", t.next().unwrap()); } print!("\n"); }"###);
    let c = s.as_bytes().iter().map(|x| x - 32).collect::<Vec<u8>>();
    for x in c.iter() {
        print!("{:02},", x);
    }
    print!("\n");
}
// 20:40:28 < cythrawll> Kirjava well if you think about the circle and characters width is a unit of measurement.
// 20:40:50 < Kirjava> cythrawll: so the character lengh is really the area?
// 20:41:21 < cythrawll> so from there you can get circumference, and radius
//https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
//20:57:48 < cythrawll> Kirjava you can use it to figure out the bounds of the circle
// 20:58:00 < cythrawll> then once you have that for each row, you can easily fill that right?
//
//
//     let xc = radius;
//     let yc = radius;
//     let rx = radius;
//     let ry = radius;
//     let mut x = 0;
//     let mut y = ry;
//     let mut p = (ry*ry)-(rx*rx*ry)+((rx*rx)/4);
//     while((2*x*ry*ry)<(2*y*rx*rx))
//     {
//         bounds.push((xc+x,yc-y));
//         bounds.push((xc-x,yc+y));
//         bounds.push((xc+x,yc+y));
//         bounds.push((xc-x,yc-y));

//         if(p<0)
//         {
//             x=x+1;
//             p=p+(2*ry*ry*x)+(ry*ry);
//         }
//         else
//         {
//             x=x+1;
//             y=y-1;
//             p=p+(2*ry*ry*x+ry*ry)-(2*rx*rx*y);
//         }
//     }
//     p=((x as f64+0.5)*(x as f64+0.5)) as i32 *ry*ry+(y-1)*(y-1)*rx*rx-rx*rx*ry*ry;

//     while(y>=0)
//     {
//         bounds.push((xc+x,yc-y));
//         bounds.push((xc-x,yc+y));
//         bounds.push((xc+x,yc+y));
//         bounds.push((xc-x,yc-y));

//         if(p>0)
//         {
//             y=y-1;
//             p=p-(2*rx*rx*y)+(rx*rx);

//         }
//         else
//         {
//             y=y-1;
//             x=x+1;
//             p=p+(2*ry*ry*x)-(2*rx*rx*y)-(rx*rx);
//         }
//     }

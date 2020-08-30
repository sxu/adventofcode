pub fn day8(input_path: &str) {
    let inputs: Vec<u32> = std::fs::read_to_string(input_path)
        .unwrap_or_else(|e| panic!("Failed to open {}: {}", input_path, e))
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const LAYER_STRIDE: usize = WIDTH * HEIGHT;
    let mut min_layer = 0;
    let mut min_num_0 = usize::MAX;
    for i in (0..inputs.len()).step_by(LAYER_STRIDE) {
        let num_0 = inputs[i..(i + LAYER_STRIDE)]
            .iter()
            .filter(|&x| *x == 0)
            .count();
        if num_0 < min_num_0 {
            min_num_0 = num_0;
            min_layer = i;
        }
    }
    let num_1 = inputs[min_layer..(min_layer + LAYER_STRIDE)]
        .iter()
        .filter(|&x| *x == 1)
        .count();
    let num_2 = inputs[min_layer..(min_layer + LAYER_STRIDE)]
        .iter()
        .filter(|&x| *x == 2)
        .count();
    assert_eq!(num_1 * num_2, 1548);

    let mut image: Vec<u32> = Vec::new();
    image.resize(LAYER_STRIDE, 2);
    for i in (0..inputs.len()).step_by(LAYER_STRIDE).rev() {
        for j in 0..LAYER_STRIDE {
            let pixel = inputs[i + j];
            if pixel != 2 {
                image[j] = pixel;
            }
        }
    }
    let image = image
        .iter()
        .map(|&x| if x == 1 { '#' } else { ' ' })
        .collect::<String>();
    println!("");
    for i in (0..image.len()).step_by(WIDTH) {
        println!("{}", &image[i..(i + WIDTH)]);
    }
}

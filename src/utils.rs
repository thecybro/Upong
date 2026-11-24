pub const fn const_sqrt(x: f32) -> f32 {
    if x < 0.0 {
        return f32::NAN;
    }
    if x == 0.0 {
        return 0.0;
    }
    let mut guess = x;
    let mut i = 0;
    while i < 50 {
        guess = 0.5 * (guess + x / guess);
        i += 1;
    }
    guess
}

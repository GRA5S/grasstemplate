pub struct Pid {
    // constants
    kp: f64,
    ki: f64,
    kd: f64,
    // calculated
    p: f64,
    i: f64,
    d: f64,
    
    target: f64,

    output: f64,

    // arrive is wheteher to stop after you reach the target. arrived is if u have reached the target
    arrived: bool,
    arrive: bool,

    current_error:f64,
    prev_error:f64,
    total_error:f64,

    small_error_tolerance: f64,
    big_error_tolerance: f64,

    small_error_duration: f64,
    big_error_duration: f64,

    small_check_time: f64,
    big_check_time: f64,

    first_run: bool,

    integral_range: f64,
    integral_max: f64,

    derivitive_tolerence: f64,
    error_tolerence: f64,
}

impl Pid {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self { kp, ki, kd }
    }
    pub fn set_constants(&mut self, p: f64, i: f64, d: f64) {  }
    pub fn thing(&mut self, thing: f64) { self.thing = thing; }
    pub fn thing(&mut self, thing: f64) { self.thing = thing; }
    pub fn thing(&mut self, thing: f64) { self.thing = thing; }
    pub fn thing(&mut self, thing: f64) { self.thing = thing; }
    pub fn thing(&mut self, thing: f64) { self.thing = thing; }
    pub fn thing(&mut self, thing: f64) { self.thing = thing; }

}
pub fn sigmoid(a: f64, b: f64) -> f64 {
    1.0 / (1.0 + (-a/b).exp())
}

pub trait Neuron {
    fn get_value(&self) -> f64;
    fn update(&mut self);
}

pub struct Input {
    pub value: f64,
}

impl Neuron for Input {
    fn get_value(&self) -> f64 {
        self.value
    }

    fn update(&mut self) {}
}

pub struct LayerNeuron<'a, T>
where
    T: 'a + Neuron,
{
    coefficients: Vec<f64>,
    pub function: fn(f64) -> f64,
    prev: Vec<&'a T>,
    value: f64,
}

impl<'a, T> LayerNeuron<'a, T>
where
    T: 'a + Neuron,
{
    pub fn new(c: Vec<f64>, f: fn(f64) -> f64, p: Vec<&'a T>) -> LayerNeuron<'a, T> {
        if c.len() != p.len() {
            panic!("Different length of coefficients and prev neurons")
        }
        LayerNeuron {
            coefficients: c,
            function: f,
            prev: p,
            value: 0.0,
        }
    }
}

impl<'a, T> Neuron for LayerNeuron<'a, T>
where
    T: 'a + Neuron,
{
    fn get_value(&self) -> f64 {
        self.value
    }

    fn update(&mut self) {
        self.value = 0.0;
        for i in 0..self.prev.len() {
            self.value += self.prev[i].get_value() * self.coefficients[i];
        }
        self.value = (self.function)(self.value);
    }
}

pub struct Layer<T>
where
    T: Neuron,
{
    neurons: Vec<T>,
}

impl<T> Layer<T>
where
    T: Neuron,
{
    pub fn new(a: Vec<T>) -> Layer<T> {
        Layer { neurons: a }
    }

    pub fn update(&mut self) {
        for i in &mut self.neurons {
            i.update();
        }
    }
}

use kodanu_ecs::{Plugin, Registry, Stage};

use burn::{
    module::Module,
    nn::{Linear, LinearConfig, Relu, loss::CrossEntropyLoss},
    optim::{AdamConfig, GradientsParams},
    tensor::{Device, Int, Tensor},
};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry.with_system(Stage::Startup, ai_startup_system);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NpcAction {
    Attack,
    Defend,
    Retreat,
}

impl NpcAction {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Attack),
            1 => Some(Self::Defend),
            2 => Some(Self::Retreat),
            _ => None,
        }
    }
}

#[derive(Module, Debug)]
pub struct NpcBrain {
    hidden: Linear,
    activation: Relu,
    output: Linear,
}

impl NpcBrain {
    pub fn new(device: &Device) -> Self {
        Self {
            hidden: LinearConfig::new(5, 16).init(device),
            activation: Relu::new(),
            output: LinearConfig::new(16, 3).init(device),
        }
    }

    pub fn forward(&self, state: Tensor<2>) -> Tensor<2> {
        let x = self.hidden.forward(state);
        let y = self.activation.forward(x);
        self.output.forward(y)
    }
}

fn ai_startup_system() {
    let device = Device::flex().autodiff();

    let mut brain = NpcBrain::new(&device);
    let mut optimizer = AdamConfig::new().init();

    let state = Tensor::<2>::from_data(
        [
            [0.9, 0.7, 0.2, 1.0, 0.3],
            [0.2, 0.9, 0.1, 1.0, 0.8],
            [0.8, 0.4, 0.9, 0.0, 0.7],
        ],
        &device,
    );

    let targets = Tensor::<1, Int>::from_data([0, 1, 2], &device);

    let learning_rate = 0.01;
    let loss_fn = CrossEntropyLoss::new(None, &device);

    for epoch in 0..501 {
        let output = brain.forward(state.clone());
        let loss = loss_fn.forward(output, targets.clone());

        if epoch % 50 == 0 {
            let value: f32 = loss.clone().into_scalar();
            println!("Epoch {epoch}: loss = {value:.4}");
        }

        let grads = loss.backward();
        let grads = GradientsParams::from_grads(grads, &brain);

        brain = optimizer.step(learning_rate, brain, grads)
    }

    println!("Training complete!");

    let output = brain.forward(state);
    let predictions = output.argmax(1);

    println!("Predictions:\n{predictions}");
    println!("Expected:\n{targets}");

    let first_prediction = predictions.clone().slice(0..1).into_scalar::<i64>() as usize;

    match NpcAction::from_index(first_prediction) {
        Some(action) => println!("First NPC action: {action:?}"),
        None => println!("Invalid action index: {first_prediction}"),
    }
}

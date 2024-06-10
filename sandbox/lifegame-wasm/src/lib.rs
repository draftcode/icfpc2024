use lifegame::LifeGame;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LifeGameWasm(LifeGame);

#[wasm_bindgen]
impl LifeGameWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(h: isize, w: isize) -> Self {
        Self(LifeGame::new(h, w))
    }

    // wasm-bindgen doesn't support tuples, so return a vec instead.
    pub fn size(&self) -> Vec<isize> {
        let (h, w) = self.0.size();
        vec![h, w]
    }

    pub fn get(&self, i: isize, j: isize) -> bool {
        self.0.get(i, j)
    }

    pub fn set(&mut self, i: isize, j: isize, b: bool) {
        self.0.set(i, j, b)
    }

    pub fn tick(&mut self) {
        self.0.tick()
    }

    // wasm-bindgen doesn't support Vec<Vec<bool>> nor Vec<bool>, so
    // return a flattened JsValue array instead.
    pub fn world(&self) -> Vec<JsValue> {
        self.0
            .world()
            .clone()
            .concat()
            .into_iter()
            .map(JsValue::from_bool)
            .collect()
    }
}

wasm_bindgen('calories_calc_bg.wasm');
document.documentElement.addEventListener("click", wasm_bindgen.handle_click);

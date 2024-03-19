const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

import './size.js';
import { finger_payload } from './finger_payload.js';

const canvas = document.getElementById('main_canvas');
const bottom_info = document.getElementById('bottom_info');

const ctx = canvas.getContext('2d');

let coordinate_x = 0;
let coordinate_y = 0;

ctx.fillRect(0, 0, 100, 100);

invoke("start_background_worker").then(() => console.log("Background worker started"));

const finger_update = await listen('finger_update', (event) => {
      const payload = finger_payload.deserializePayload(event.payload);
      console.log(payload.toString());
      bottom_info.innerHTML = payload.toBottomInfo();

      coordinate_x += 10;
      coordinate_y += 10;
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.fillRect(coordinate_x, coordinate_y, 100, 100);
});
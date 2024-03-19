const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

import './size.js';
import { finger_payload } from './finger_payload.js';

const canvas = document.getElementById('main_canvas');
const bottom_info = document.getElementById('bottom_info');

const ctx = canvas.getContext('2d');

let coordinate_x = 0;
let coordinate_y = 0;

invoke("start_background_worker").then(() => console.log("Background worker started"));

const finger_update = await listen('finger_update', (event) => {
      const payload = finger_payload.deserializePayload(event.payload);
      console.log(payload.toString());
      bottom_info.innerHTML = payload.toBottomInfo();
      const denormalizedCoordinates = denormalizeCoordinates(payload.coordinates);

      coordinate_x = denormalizedCoordinates[0];
      coordinate_y = denormalizedCoordinates[1];
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.fillRect(coordinate_x, coordinate_y, 10, 10);
});

function denormalizeCoordinates(normalizedCoordinates) {
      const minX = 0;
      const minY = 0;
      const maxX = canvas.width;
      const maxY = canvas.height;
      const denormalizedX = normalizedCoordinates[0] * (maxX - minX) + minX;
      const denormalizedY = normalizedCoordinates[1] * (maxY - minY) + minY;
      return [denormalizedX, denormalizedY];
}
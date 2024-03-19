const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

import './size.js';
import { finger_payload, Status } from './finger_payload.js';
import { finger } from './finger.js';

const canvas = document.getElementById('main_canvas');
const bottom_info = document.getElementById('bottom_info');

const ctx = canvas.getContext('2d');

invoke("start_background_worker").then(() => console.log("Background worker started"));

let fingers = [finger];

await listen('finger_update', (event) => {
      const payload = finger_payload.deserializePayload(event.payload);
      bottom_info.innerHTML = payload.toBottomInfo();
      const coordinates = denormalizeCoordinates(payload.coordinates);

      let current_finger = fingers.find(finger => finger.id === payload.id);
      if (current_finger === undefined) {
            current_finger = new finger(payload.id, payload.status, coordinates, ctx, 10);
            fingers.push(current_finger);
      } else {
            if (payload.status === Status.Delete ){
                  fingers = fingers.filter(finger => finger.id !== payload.id);
            }
            current_finger.coordinates = coordinates;
            current_finger.status = payload.status;
      }

      // current_finger.draw_finger_to_canvas(payload.status);

      ctx.clearRect(0, 0, window.innerWidth, window.innerHeight);

      for (let finger of fingers) {
            let coords = finger.coordinates;
            if (coords !== undefined) {
                  ctx.fillRect(finger.coordinates[0], finger.coordinates[1], 10, 10);
            }
      }


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
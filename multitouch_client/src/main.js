const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

import './size.js';
import { finger_payload } from './finger_payload.js';
import { finger } from './finger.js';

const canvas = document.getElementById('main_canvas');
const bottom_info = document.getElementById('bottom_info');

const ctx = canvas.getContext('2d');

invoke("start_background_worker").then(() => console.log("Background worker started"));

let fingers= [finger];

await listen('finger_update', (event) => {
      const payload = finger_payload.deserializePayload(event.payload);
      bottom_info.innerHTML = payload.toBottomInfo();
      const coordinates = denormalizeCoordinates(payload.coordinates);

      const current_finger = fingers.find(finger => finger.id === payload.id);
      if (current_finger === undefined) {
            fingers.push(new finger(payload.id, payload.status, coordinates, undefined));
      } else {
            current_finger.previousCoordinates = current_finger.coordinates;
            current_finger.coordinates = coordinates;
      }

        finger.draw_finger_to_canvas(ctx, canvas, payload.status, coordinates, current_finger.previousCoordinates);
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
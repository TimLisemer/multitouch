const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

import './size.js';
import { Finger, Status } from './Finger.js';
import {Button} from "./Button.js";

const canvas = document.getElementById('main_canvas');
const ctx = canvas.getContext('2d');

const bottom_info = document.getElementById('bottom_info');

let fingers = [Finger];
let buttons = [Button];

invoke("start_background_worker").then(() => console.log("Background worker started"));

await listen('finger_update', (event) => {
      const payload_finger = Finger.deserializePayload(event.payload)
      const coordinates = denormalizeCoordinates(payload_finger.coordinates);

      const current_finger = fingers.find(finger => finger.id === payload_finger.id);
      if (current_finger === undefined) {
            fingers.push(new Finger(payload_finger.id, coordinates, payload_finger.status, payload_finger.color));
      } else {
            current_finger.coordinates = coordinates;
            current_finger.status = payload_finger.status;
      }

      bottom_info.innerHTML = "Finger: " + payload_finger.id + " Status: " + payload_finger.status + " Coordinates: " + coordinates;

      ctx.clearRect(0, 0, window.innerWidth, window.innerHeight);

      for (let finger of fingers) {
            const { coordinates, status, color } = finger;
            if (status === Status.Create || status === Status.Update) {
                  ctx.fillStyle = color;
                  ctx.fillRect(coordinates[0], coordinates[1], 10, 10);
            }
      }


      for (let button of buttons) {
            drawButton(button);
      }


});


await listen('button_create', (event) => {
      const payload_button = Button.deserializePayload(event.payload);
      buttons.push(payload_button);
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

function drawButton(button) {
      const {coordinates, dimensions, label, color} = button;
      console.log("Drawing button " + button.label + " at " + button.coordinates);
      console.log(coordinates, dimensions, label, color);
      /*
      // Draw a button-like rectangle
      ctx.fillStyle = color;
      ctx.fillRect(coordinates[0], coordinates[1], dimensions[0], dimensions[1]);

      // Add text to the button
      ctx.fillStyle = 'white'; // White text color
      ctx.font = 'bold 14px Arial';
      ctx.fillText(label, coordinates[0] + 10, coordinates[1] + 20); // Adjust text position according to button size
      */
}
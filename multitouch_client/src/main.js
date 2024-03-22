const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

import './size.js';
import { Finger, Status } from './Finger.js';
import {Button} from "./Button.js";
import {Shape} from "./Shape.js";

const canvas = document.getElementById('main_canvas');
const ctx = canvas.getContext('2d');

const bottom_info = document.getElementById('bottom_info');

let fingers = [];
let buttons = [];
let shapes = [];
let bottom_info_text = ""

await listen('finger_update', (event) => {
      const payload_finger = Finger.deserializePayload(event.payload)
      const coordinates = denormalize(payload_finger.coordinates);

      const current_finger = fingers.find(finger => finger.id === payload_finger.id);
      if (current_finger === undefined) {
            fingers.push(new Finger(payload_finger.id, coordinates, payload_finger.status, payload_finger.color));
      } else {
            current_finger.coordinates = coordinates;
            current_finger.status = payload_finger.status;
      }

      printBottomInfo();

      ctx.clearRect(0, 0, window.innerWidth, window.innerHeight);

      for (let button of buttons) {
            drawButton(button);
      }

      for (let shape of shapes) {
            drawShape(ctx, shape.vertices, shape.color);
      }

      for (let finger of fingers) {
            const { coordinates, status, color } = finger;
            if (status === Status.Create || status === Status.Update) {
                  ctx.fillStyle = color;
                  ctx.fillRect(coordinates[0], coordinates[1], 10, 10);
            }
      }

});

await listen('button_create', (event) => {
      console.log("Button create event received");
      const payload_button = Button.deserializePayload(event.payload);
      buttons.push(payload_button);
      for (let button of buttons) {
            drawButton(button);
      }
      setBottomInfo("Click the button to draw a shape");
});

const update_button_color = await listen('update_button_color', (event) => {
        console.log("Update button color event received");
      let button = buttons.find(button => button.id === event.payload.id);
      update_button_mode(button);
});

function update_button_mode(button) {
      if (button !== undefined) {
            button.color = button.mode_color;
            button.mode = !button.mode;
            for (let button of buttons) {
                  drawButton(button);
            }
            if (button.mode) {
                  setBottomInfo("Draw your shape")
            } else {
                  setBottomInfo("Click the button to draw a shape");
            }
      }
}

const create_shape = await listen('create_shape', (event) => {
      let vertices = event.payload.vertices;
      for (let i = 0; i < vertices.length; i++) {
            vertices[i] = denormalize(vertices[i]);
      }

      let shape = Shape.deserializePayload(event.payload, vertices);
      shapes.push(shape);
      //shape.print();
      drawShape(ctx, shape.vertices, shape.color);
});

const update_shape = await listen('update_shape', (event) => {
        let vertices = event.payload.vertices;
        for (let i = 0; i < vertices.length; i++) {
                vertices[i] = denormalize(vertices[i]);
        }

        let shape = shapes.find(shape => shape.id === event.payload.id);
        if (shape !== undefined) {
                shape.vertices = vertices;
                drawShape(ctx, shape.vertices, shape.color);
        }
});

const detect_shape = await listen('detect_shape', (event) => {
      let finger = fingers.find(finger => finger.id === event.payload.id);
      if (finger === undefined) {
            setBottomInfo("Error Detecting Shape (Finger Error)");
      } else {
            setBottomInfo("Detected Shape: ");
            update_button_mode(buttons.at(0));
      }
});

function setBottomInfo(text) {
      bottom_info_text = text;
      printBottomInfo();
}

function printBottomInfo(){
      bottom_info.innerHTML = bottom_info_text;
}


function denormalize(normalizedCoordinates) {
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
      // Denormalize coordinates and dimensions
      const denormalizedCoordinates = denormalize(coordinates);
      const denormalizedDimensions = denormalize(dimensions);

      // Draw a button-like rectangle
      ctx.fillStyle = color;
      ctx.fillRect(denormalizedCoordinates[0], denormalizedCoordinates[1], denormalizedDimensions[0], denormalizedDimensions[1]);

      // Add text to the button
      ctx.fillStyle = 'white'; // White text color
      ctx.font = 'bold 14px Arial';
      ctx.fillText(label, coordinates[0] + 10, coordinates[1] + 20); // Adjust text position according to button size
}

invoke('button_create').then(() => {
      invoke("start_background_worker").then(() => {
            console.log("Background worker started");
      });
});

function drawShape(ctx, vertices, color) {
      ctx.beginPath();
      ctx.moveTo(vertices[0][0], vertices[0][1]);
      for(let i = 1; i < vertices.length; i++) {
            ctx.lineTo(vertices[i][0], vertices[i][1]);
      }
      ctx.closePath();
      ctx.fillStyle = color;
      ctx.fill();
}
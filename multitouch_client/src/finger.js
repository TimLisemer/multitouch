import { Status } from './finger_payload.js';

export class finger {
    constructor(id, status, coordinates, ctx, canvas, draw_size) {
        this._id = id;
        this._status = status;
        this._coordinates = coordinates;
        this._ctx = ctx;
        this._canvas = canvas;
        this._draw_size = draw_size;
    }

    get id() {
        return this._id;
    }

    get status() {
        return this._status;
    }

    get coordinates() {
        return this._coordinates;
    }

    set coordinates(coordinates) {
        this.remove_finger_from_canvas(coordinates);
        this._coordinates = coordinates;
    }

    get ctx() {
        return this._ctx;
    }

    get canvas() {
        return this._canvas;
    }

    get draw_size() {
        return this._draw_size;
    }

    draw_finger_to_canvas(status) {
        if (status === Status.Create) {
            this.add_finger_to_canvas(this.coordinates);
        } else if (status === Status.Update) {
            this.update_finger_on_canvas(this.coordinates);
        } else if (status === Status.Delete) {
            this.remove_finger_from_canvas(this.coordinates);
        }
    }


    add_finger_to_canvas(denormalizedCoordinates) {
        const coordinate_x = denormalizedCoordinates[0];
        const coordinate_y = denormalizedCoordinates[1];
        this.ctx.fillRect(coordinate_x, coordinate_y, this.draw_size, this.draw_size);
    }

    remove_finger_from_canvas(denormalizedCoordinates) {
        const coordinate_x = denormalizedCoordinates[0];
        const coordinate_y = denormalizedCoordinates[1];
        this.ctx.clearRect(coordinate_x, coordinate_y, this.draw_size, this.draw_size);
    }

    update_finger_on_canvas(denormalizedCoordinates) {
        this.add_finger_to_canvas(denormalizedCoordinates);
    }

}



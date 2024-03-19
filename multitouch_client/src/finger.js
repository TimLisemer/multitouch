import { Status } from './finger_payload.js';

export class finger {

    constructor(id, status, coordinates, previousCoordinates) {
        this._id = id;
        this._status = status;
        this._coordinates = coordinates;
        this._previousCoordinates = previousCoordinates;
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
        this._coordinates = coordinates;
    }

    get previousCoordinates() {
        return this._previousCoordinates;
    }

    set previousCoordinates(previousCoordinates) {
        this._previousCoordinates = previousCoordinates;
    }

    static draw_finger_to_canvas(ctx, canvas, status, denormalizedCoordinates, previousCoordinates) {
        if (status === Status.Create) {
            this.add_finger_to_canvas(ctx, canvas, denormalizedCoordinates);
        } else if (status === Status.Update) {
            this.update_finger_on_canvas(ctx, canvas, denormalizedCoordinates, previousCoordinates);
        } else if (status === Status.Delete) {
            this.remove_finger_from_canvas(ctx, canvas, denormalizedCoordinates);
        }
    }

    static add_finger_to_canvas(ctx, canvas, denormalizedCoordinates) {
        const coordinate_x = denormalizedCoordinates[0];
        const coordinate_y = denormalizedCoordinates[1];
        ctx.fillRect(coordinate_x, coordinate_y, 10, 10);
    }

    static remove_finger_from_canvas(ctx, canvas, denormalizedCoordinates) {
        const coordinate_x = denormalizedCoordinates[0];
        const coordinate_y = denormalizedCoordinates[1];
        ctx.clearRect(coordinate_x, coordinate_y, 10, 10);
    }

    static update_finger_on_canvas(ctx, canvas, denormalizedCoordinates, previousCoordinates) {
        this.remove_finger_from_canvas(ctx, canvas, previousCoordinates);
        this.add_finger_to_canvas(ctx, canvas, denormalizedCoordinates);
    }
}

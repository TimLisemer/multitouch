export const Status = {
    Create: "Create",
    Update: "Update",
    Delete: "Delete"
};

// Define the Payload class with getters
export class Finger {
    constructor(id, coordinates, history, color, status) {
        this._id = id;
        this._coordinates = coordinates;
        this._history = history;
        this._color = color;
        this._status = status;
    }

    get id() {
        return this._id;
    }
    get coordinates() {
        return this._coordinates;
    }
    set coordinates(coordinates) {
        this._coordinates = coordinates;
    }
    get history() {
        return this._history;
    }
    get color() {
        return this._color;
    }
    get status() {
        return this._status;
    }
    set status(status) {
        this._status = status;
    }

    static deserializePayload(payload) {
        const id = payload.id;
        const coordinates = payload.coordinates;
        const history = payload.history;
        const color = payload.color;
        const status = payload.status;

        return new Finger(id, coordinates, history, color, status);
    }
}
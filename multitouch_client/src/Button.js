export class Button {
    constructor(id, coordinates, dimension, label, color) {
        this._id = id;
        this._coordinates = coordinates;
        this._dimension = dimension;
        this._label = label;
        this._color = color;
    }

    get id() {
        return this._id;
    }
    get coordinates() {
        return this._coordinates;
    }
    get dimension() {
        return this._dimension;
    }
    get label() {
        return this._label;
    }
    get color() {
        return this._color;
    }

    static deserializePayload(payload) {
        const id = payload.id;
        const coordinates = payload.coordinates;
        const dimension = payload.dimension;
        const label = payload.label;
        const color = payload.color;

        return new Button(id, coordinates, dimension, label, color);
    }
}
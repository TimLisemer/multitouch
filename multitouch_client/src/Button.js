export class Button {
    constructor(id, coordinates, dimensions, label, color) {
        this._id = id;
        this._coordinates = coordinates;
        this._dimensions = dimensions;
        this._label = label;
        this._color = color;
    }

    get id() {
        return this._id;
    }
    get coordinates() {
        return this._coordinates;
    }
    get dimensions() {
        return this._dimensions;
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
        const dimensions = payload.dimensions;
        const label = payload.label;
        const color = payload.color;

        return new Button(id, coordinates, dimensions, label, color);
    }
}
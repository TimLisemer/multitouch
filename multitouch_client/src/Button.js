export class Button {
    constructor(id, coordinates, dimensions, label, color, mode_color, mode) {
        this._id = id;
        this._coordinates = coordinates;
        this._dimensions = dimensions;
        this._label = label;
        this._color = color;
        this._mode_color = mode_color;
        this._mode = mode;
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
    set color(color) {
        this._color = color;
    }
    get mode_color() {
        return this._mode_color;
    }
    get mode() {
        return this._mode;
    }
    set mode(mode) {
        this._mode = mode;
    }

    static deserializePayload(payload) {
        const id = payload.id;
        const coordinates = payload.coordinates;
        const dimensions = payload.dimensions;
        const label = payload.label;
        const color = payload.color;
        const mode_color = payload.mode_color;
        const mode = payload.mode;

        return new Button(id, coordinates, dimensions, label, color);
    }
}
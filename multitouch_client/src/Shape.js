export class Shape {
    constructor(id, vertices, scale, color) {
        this._id = id;
        this._vertices = vertices;
        this._scale = scale;
        this._color = color;
    }

    get id() {
        return this._id;
    }
    get vertices() {
        return this._vertices;
    }
    get scale() {
        return this._scale;
    }
    get color() {
        return this._color;
    }

    static deserializePayload(payload, vertices_from_payload) {
        const id = payload.id;
        const vertices = vertices_from_payload;
        const scale = payload.scale;
        const color = payload.color;

        return new Shape(id, vertices, scale, color);
    }

    print() {
        console.log("Shape: " + this._id + " " + this._vertices + " " + this._scale + " " + this._color);
    }
}
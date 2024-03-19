// Define the enum for Status
const Status = {
    Create: "Create",
    Update: "Update",
    Delete: "Delete"
};

// Define the Payload class with getters
export class finger_payload {
    constructor(id, status, coordinates, message) {
        this._id = id;
        this._status = status;
        this._coordinates = coordinates;
        this._message = message;
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

    get message() {
        if (this._message === undefined || this._message === null) {
            return '';
        }
        return ' ' + this._message;
    }

    // Static method for deserializing payload
    static deserializePayload(obj) {
        const id = obj.id;
        const status = Status[obj.status];
        const coordinates = obj.coordinates;
        const message = obj.message;

        return new finger_payload(id, status, coordinates, message);
    }

    // Method to get string representation of payload
    toString() {
        return `Payload { id: ${this._id}, status: ${this._status}, coordinates: [${this._coordinates}], message: ${this._message} }`;
    }

    toBottomInfo() {
        return 'ID: ' + this.id + ' - ' + this.status + ' - x=' + this.coordinates[0] + ' y=' + this.coordinates[1] + '' + this.message;
    }
}
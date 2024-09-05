import { hub } from 'dom-native';
import { webDelete, webGet, webPatch, webPost } from '../webc';

export interface Grocery {
    id: number;
    name: string;
    cost: number;
    status: 'Shelf' | 'Basket';
}

export type GroceryPatch = Partial<Omit<Grocery, 'id'>>;

class GroceryMco {

    async list(): Promise<Grocery[]> {
        const data = await webGet("groceries");
        return data as Grocery[];
    }

    async create(data: GroceryPatch): Promise<Grocery> {
        // guard (TODO - validate data)
        if (data.name == null || data.name.trim().length == 0) {
            throw new Error("Cannot create Grocery with empty name");
        }
        // to server
        const newData = await webPost('groceries', data);
        // sending event
        hub('dataHub').pub('Grocery', 'create', newData);

        return newData as Grocery;
    }

    async update(id: number, data: GroceryPatch): Promise<Grocery> {
        // TODO - validate data
        // if (data.name == null || data.name.trim().length == 0) {
        //     throw new Error("Cannot create Grocery with empty name");
        // }
        // to server
        const newData = await webPatch(`groceries/${id}`, data);
        // event
        hub('dataHub').pub('Grocery', 'update', newData);

        return newData as Grocery;
    }

    async delete(id: number): Promise<Grocery> {
        // to server
        const oldData = await webDelete(`groceries/${id}`);
        // event
        hub('dataHub').pub('Grocery', 'delete', oldData);

        return oldData as Grocery;
    }
}

export const groceryMco = new GroceryMco();
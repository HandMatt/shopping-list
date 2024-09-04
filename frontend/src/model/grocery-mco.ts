import { webGet } from "src/webc";

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
}

export const groceryMco = new GroceryMco();
import { BaseHTMLElement, customElement, getChild, getChildren, html } from "dom-native";

@customElement("grocery-mvc")
class GroceryMvc extends BaseHTMLElement {
    #groceryInputEl!: GroceryInput;
    #groceryListEl!: HTMLElement;


    init() {
        let htmlContent: DocumentFragment = html`
            <div class="box"></div>
            <h1>Shopping List</h1>
            <grocery-input></grocery-input>
            <grocery-list></grocery-list>
        `;
        [this.#groceryInputEl, this.#groceryListEl] = getChildren(htmlContent, 'grocery-input', 'grocery-list');

        this.append(htmlContent);
        this.refresh();
    }

    async refresh() {
        let groceries = [
            { id: 1, name: "mock1", cost: 50, status: 'Basket' },
            { id: 2, name: "mock2", cost: 50, status: 'Shelf' }
        ];
        let htmlContent = document.createDocumentFragment();
        for (const grocery of groceries) {
            const el = document.createElement('grocery-item');
            htmlContent.append(el);
        }

        this.#groceryListEl.innerHTML = '';
        this.#groceryListEl.append(htmlContent);
    }
}

@customElement("grocery-input")
class GroceryInput extends BaseHTMLElement { // extends HTMLElement
    #inputEl!: HTMLInputElement;

    init() {
        let htmlContent = html`
            <input type="text" placeholder="What needs to be bought?">
            <input type="number" min="0.00" max="10000.00" step="0.01" placeholder="Cost?" />
        `;
        this.#inputEl = getChild(htmlContent, 'input');

        this.append(htmlContent);
    }
}
// grocery-input tag
declare global {
    interface HTMLElementTagNameMap {
        'grocery-input': GroceryInput;
    }
}

@customElement("grocery-item")
export class GroceryItem extends BaseHTMLElement { // extends HTMLElement
    #titleEl!: HTMLElement;

    init() {
        let htmlContent = html`
            <c-check><c-ico name="ico-basket"></c-ico></c-check>
            <div class="title">STATIC TITLE</div>
            <c-ico name="del"></c-ico>
        `;
        this.#titleEl = getChild(htmlContent, 'div');

        this.append(htmlContent);
    }
}
// grocery-item type augmentation
declare global {
    interface HTMLElementTagNameMap {
        'grocery-item': GroceryItem;
    }
}
import { BaseHTMLElement, customElement, first, getChild, getChildren, html, OnEvent, onEvent, onHub } from "dom-native";
import { Grocery, groceryMco } from "src/model/grocery-mco";

@customElement("grocery-mvc")
class GroceryMvc extends BaseHTMLElement { // extends HTMLElement
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
        let groceries: Grocery[] = await groceryMco.list();
        let htmlContent = document.createDocumentFragment();
        for (const grocery of groceries) {
            const el = document.createElement('grocery-item');
            el.data = grocery; // grocery will be froszen
            htmlContent.append(el);
        }

        this.#groceryListEl.innerHTML = '';
        this.#groceryListEl.append(htmlContent);
    }

    // #region    --- UI Events
    @onEvent('pointerup', 'c-check')
    onCheckGrocery(evt: PointerEvent & OnEvent) {
        const groceryItem = evt.selectTarget.closest("grocery-item")!;
        const status = groceryItem.data.status == 'Shelf' ? 'Basket' : 'Shelf';
        // update to server
        groceryMco.update(groceryItem.data.id, { status });
    }
    // #endregion --- UI Events

    // #region    --- Data Events
    @onHub('dataHub', 'Grocery', 'update')
    onGroceryUpdate(data: Grocery) {
        // find the grocery in the UI
        const groceryItem = first(this, `grocery-item.Grocery-${data.id}`) as GroceryItem | undefined;
        // if found, update it
        if (groceryItem) {
            groceryItem.data = data; // data will be frozen      
        }
    }

    @onHub('dataHub', 'Grocery', 'create')
    onGroceryCreate(data: Grocery) {
        this.refresh();
    }
    // #endregion --- Data Events

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

    // #region    --- UI Events
    @onEvent('keyup', 'input')
    onInputKeyUp(evt: KeyboardEvent) {
        if (evt.key == "Enter") {
            // get value from UI
            const name = this.#inputEl.value;
            // send create to server
            groceryMco.create({ name });
            // don't wait, reset value input
            this.#inputEl.value = '';
        }
    }
    // #endregion --- UI Events

    // #region    --- Data Events
    @onHub('dataHub', 'Grocery', 'update')
    onGroceryUpdate(data: Grocery) {
        // find the grocery in the UI
        const groceryItem = first(this, `grocery-item.Grocery-${data.id}`) as GroceryItem | undefined;
        // if found, update it
        if (groceryItem) {
            groceryItem.data = data; // data will be frozen      
        }
    }
    // #endregion --- Data Events
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
    #data!: Grocery;

    set data(data: Grocery) {
        let oldData = this.#data;
        this.#data = Object.freeze(data);
        if (this.isConnected) {
            this.refresh(oldData);
        }
    }

    get data() { return this.#data };

    init() {
        let htmlContent = html`
            <c-check><c-ico name="ico-basket"></c-ico></c-check>
            <div class="title">STATIC TITLE</div>
            <c-ico name="del"></c-ico>
        `;
        this.#titleEl = getChild(htmlContent, 'div');

        this.append(htmlContent);
        this.refresh();
    }

    refresh(old?: Grocery) {
        if (old != null) {
            this.classList.remove(`Grocery-${old.id}`);
            this.classList.remove(old.status);
        }

        // render new data
        const grocery = this.#data;
        this.classList.add(`Grocery-${grocery.id}`);
        this.classList.add(grocery.status);
        this.#titleEl.textContent = grocery.name;
    }
}
// grocery-item type augmentation
declare global {
    interface HTMLElementTagNameMap {
        'grocery-item': GroceryItem;
    }
}
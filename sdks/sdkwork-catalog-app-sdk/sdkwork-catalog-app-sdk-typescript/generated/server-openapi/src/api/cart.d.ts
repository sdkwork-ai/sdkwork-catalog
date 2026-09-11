import type { ApiRequestOptions, HttpClient } from '../http/client';
import type { AddCartItemRequest, CartItem, CartItemPageData, UpdateCartItemRequest } from '../types';
export interface CartItemsListParams {
    page?: number;
    pageSize?: number;
}
export declare class CartItemsApi {
    private client;
    constructor(client: HttpClient);
    /** List the authenticated user's cart items. */
    list(params?: CartItemsListParams, requestOptions?: ApiRequestOptions): Promise<CartItemPageData>;
    /** Add an item to the authenticated user's cart. */
    create(body: AddCartItemRequest, requestOptions?: ApiRequestOptions): Promise<CartItem>;
    /** Update a cart item quantity. */
    update(cartItemId: string, body: UpdateCartItemRequest, requestOptions?: ApiRequestOptions): Promise<CartItem>;
    /** Delete a cart item. */
    delete(cartItemId: string, requestOptions?: ApiRequestOptions): Promise<void>;
}
export declare class CartApi {
    readonly items: CartItemsApi;
    constructor(client: HttpClient);
}
export declare function createCartApi(client: HttpClient): CartApi;
//# sourceMappingURL=cart.d.ts.map
import type { ApiRequestOptions, HttpClient } from '../http/client';
import type { Address, AddressPageData, CreateAddressRequest, UpdateAddressRequest } from '../types';
export declare class DeliveryAddressesDefaultSelectionApi {
    private client;
    constructor(client: HttpClient);
    /** Set the default delivery address. */
    update(addressId: string, requestOptions?: ApiRequestOptions): Promise<Address>;
}
export interface DeliveryAddressesListParams {
    page?: number;
    pageSize?: number;
}
export declare class DeliveryAddressesApi {
    private client;
    readonly defaultSelection: DeliveryAddressesDefaultSelectionApi;
    constructor(client: HttpClient);
    /** List the authenticated user's delivery addresses. */
    list(params?: DeliveryAddressesListParams, requestOptions?: ApiRequestOptions): Promise<AddressPageData>;
    /** Create a delivery address. */
    create(body: CreateAddressRequest, requestOptions?: ApiRequestOptions): Promise<Address>;
    /** Update a delivery address. */
    update(addressId: string, body: UpdateAddressRequest, requestOptions?: ApiRequestOptions): Promise<Address>;
    /** Delete a delivery address. */
    delete(addressId: string, requestOptions?: ApiRequestOptions): Promise<void>;
}
export declare class DeliveryApi {
    readonly addresses: DeliveryAddressesApi;
    constructor(client: HttpClient);
}
export declare function createDeliveryApi(client: HttpClient): DeliveryApi;
//# sourceMappingURL=delivery.d.ts.map
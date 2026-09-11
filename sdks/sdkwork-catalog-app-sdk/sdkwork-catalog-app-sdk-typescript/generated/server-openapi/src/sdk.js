import { createHttpClient } from './http/client';
import { createCatalogApi } from './api/catalog';
import { createCartApi } from './api/cart';
import { createDeliveryApi } from './api/delivery';
export class SdkworkAppClient {
    httpClient;
    catalog;
    cart;
    delivery;
    constructor(config) {
        this.httpClient = createHttpClient(config);
        this.catalog = createCatalogApi(this.httpClient);
        this.cart = createCartApi(this.httpClient);
        this.delivery = createDeliveryApi(this.httpClient);
    }
    setAuthToken(token) {
        this.httpClient.setAuthToken(token);
        return this;
    }
    setAccessToken(token) {
        this.httpClient.setAccessToken(token);
        return this;
    }
    setTokenManager(manager) {
        this.httpClient.setTokenManager(manager);
        return this;
    }
    get http() {
        return this.httpClient;
    }
}
export function createClient(config) {
    return new SdkworkAppClient(config);
}
export default SdkworkAppClient;
//# sourceMappingURL=sdk.js.map
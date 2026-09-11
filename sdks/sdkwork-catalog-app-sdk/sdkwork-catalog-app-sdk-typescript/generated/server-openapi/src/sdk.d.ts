import { HttpClient } from './http/client';
import type { SdkworkAppConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { CatalogApi } from './api/catalog';
import { CartApi } from './api/cart';
import { DeliveryApi } from './api/delivery';
export declare class SdkworkAppClient {
    private httpClient;
    readonly catalog: CatalogApi;
    readonly cart: CartApi;
    readonly delivery: DeliveryApi;
    constructor(config: SdkworkAppConfig);
    setAuthToken(token: string): this;
    setAccessToken(token: string): this;
    setTokenManager(manager: AuthTokenManager): this;
    get http(): HttpClient;
}
export declare function createClient(config: SdkworkAppConfig): SdkworkAppClient;
export default SdkworkAppClient;
//# sourceMappingURL=sdk.d.ts.map
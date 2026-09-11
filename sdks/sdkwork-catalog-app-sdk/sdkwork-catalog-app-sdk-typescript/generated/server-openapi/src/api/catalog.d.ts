import type { ApiRequestOptions, HttpClient } from '../http/client';
import type { CatalogAttributePageData, CatalogCategory, CatalogCategoryPageData, CatalogProduct, CatalogProductPageData, CatalogSku, CatalogSkuPageData } from '../types';
export declare class CatalogSkusApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve an active catalog SKU. */
    retrieve(skuId: string, requestOptions?: ApiRequestOptions): Promise<CatalogSku>;
}
export interface CatalogProductsSkusListParams {
    page?: number;
    pageSize?: number;
}
export declare class CatalogProductsSkusApi {
    private client;
    constructor(client: HttpClient);
    /** List active SKUs for a catalog product. */
    list(productId: string, params?: CatalogProductsSkusListParams, requestOptions?: ApiRequestOptions): Promise<CatalogSkuPageData>;
}
export interface CatalogProductsListParams {
    shopId?: string;
    categoryId?: string;
    productType?: string;
    sort?: string;
    page?: number;
    pageSize?: number;
}
export declare class CatalogProductsApi {
    private client;
    readonly skus: CatalogProductsSkusApi;
    constructor(client: HttpClient);
    /** List active catalog products. */
    list(params?: CatalogProductsListParams, requestOptions?: ApiRequestOptions): Promise<CatalogProductPageData>;
    /** Retrieve an active catalog product. */
    retrieve(productId: string, requestOptions?: ApiRequestOptions): Promise<CatalogProduct>;
}
export interface CatalogCategoriesListParams {
    parentId?: string;
    page?: number;
    pageSize?: number;
}
export declare class CatalogCategoriesApi {
    private client;
    constructor(client: HttpClient);
    /** List active catalog categories. */
    list(params?: CatalogCategoriesListParams, requestOptions?: ApiRequestOptions): Promise<CatalogCategoryPageData>;
    /** Retrieve an active catalog category. */
    retrieve(categoryId: string, requestOptions?: ApiRequestOptions): Promise<CatalogCategory>;
}
export interface CatalogAttributesListParams {
    page?: number;
    pageSize?: number;
}
export declare class CatalogAttributesApi {
    private client;
    constructor(client: HttpClient);
    /** List active catalog attributes. */
    list(params?: CatalogAttributesListParams, requestOptions?: ApiRequestOptions): Promise<CatalogAttributePageData>;
}
export declare class CatalogApi {
    readonly attributes: CatalogAttributesApi;
    readonly categories: CatalogCategoriesApi;
    readonly products: CatalogProductsApi;
    readonly skus: CatalogSkusApi;
    constructor(client: HttpClient);
}
export declare function createCatalogApi(client: HttpClient): CatalogApi;
//# sourceMappingURL=catalog.d.ts.map
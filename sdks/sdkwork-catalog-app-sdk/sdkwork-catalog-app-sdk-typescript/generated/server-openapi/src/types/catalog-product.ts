export interface CatalogProduct {
  /** Primary key. */
  id: string;
  /** Stable business key of the product, unique per tenant while the row is live. */
  productNo: string;
  /** Owning category id. */
  categoryId: string;
  /** Default-locale name. Bounded by ck_commerce_product_spu_name_length. */
  name: string;
  /** Default-locale marketing title. */
  title?: string | null;
  /** Default-locale marketing subtitle. */
  subtitle?: string | null;
  /** Default-locale long description. */
  description?: string | null;
  /** How the product is fulfilled. ck_commerce_product_spu_product_type is the authority for this set. */
  productType: 'physical' | 'digital' | 'service' | 'membership' | 'points';
  /** Lifecycle status. ck_commerce_product_spu_status is the authority for this set. */
  status: 'draft' | 'active' | 'inactive' | 'archived';
  /** Whether the product may be sold. ck_commerce_product_spu_sales_status is the authority; `active` requires a non-draft status (ck_commerce_product_spu_sales_requires_published). */
  salesStatus: 'active' | 'inactive';
  /** First publish time. Null until the product leaves draft (ck_commerce_product_spu_published_at). */
  publishedAt?: string | null;
  /** Row creation time. */
  createdAt: string;
  /** Last write time. */
  updatedAt: string;
  /** Row version, advanced by every write to this row and by any write that rewrites the row's own storage identity (a category move also advances the moved subtree). It is the value an update or delete must send back as `If-Match`, and the same number the response's `ETag` carries, so a client that has read a resource can always name the version it read. `API_SPEC` section 17 keeps the version source, the required header, and the two result codes together: a missing `If-Match` answers `42801`, a stale one answers `41201`. */
  version: string;
}

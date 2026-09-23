export interface CatalogCategory {
  /** Primary key, minted by the service host before INSERT. */
  id: string;
  /** Stable business key of the category, unique per tenant while the row is live. */
  categoryNo: string;
  /** Parent category id. Null for a root category; never equal to `id` (ck_commerce_product_category_self_parent). */
  parentId?: string | null;
  /** Materialised ancestor path. Matches ck_commerce_product_category_path_shape. */
  path: string;
  /** Zero-based depth in the tree. `depth >= 0` per ck_commerce_product_category_depth. */
  depth: string;
  /** True when the category has no live children. */
  isLeaf: boolean;
  /** Default-locale name. Bounded by ck_commerce_product_category_name_length. */
  name: string;
  /** Sibling ordering key, ascending. */
  sortOrder: string;
  /** Shared lifecycle vocabulary. The authority for this set is the column CHECK constraint named in the field description. */
  status: 'active' | 'inactive';
  /** Row creation time. */
  createdAt: string;
  /** Last write time. */
  updatedAt: string;
  /** Row version, advanced by every write to this row and by any write that rewrites the row's own storage identity (a category move also advances the moved subtree). It is the value an update or delete must send back as `If-Match`, and the same number the response's `ETag` carries, so a client that has read a resource can always name the version it read. `API_SPEC` section 17 keeps the version source, the required header, and the two result codes together: a missing `If-Match` answers `42801`, a stale one answers `41201`. */
  version: string;
}

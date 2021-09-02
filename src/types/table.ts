export interface Column {
  prop: string,
  label: string,
  sortable?: boolean,
  sortedBy?: SortedBy
}

export enum SortedBy {
  ASC = 'ASC',
  DESC = 'DESC'
}

export interface PageRequest {
  page: number;
  size: number;
  sort_by: string;
  direction?: string;
}

export interface PageResponse<T> {
  content: T[],
  total_pages: number,
  total_elements: number
}

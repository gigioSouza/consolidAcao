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

export class PageRequest {
  page: number;
  size: number;
  sortBy: string;
  direction?: string;

  constructor(page = 0, size = 10, sortyBy: string, direction?: string) {
    this.page = page;
    this.size = size;
    this.sortBy = sortyBy;
    this.direction = direction;
  }
}

export interface PageResponse<T> {
  content: T[],
  totalPages: number,
  totalElements: number
}

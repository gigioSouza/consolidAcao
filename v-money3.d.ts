import { Component, Directive, Plugin } from 'vue';

declare module 'v-money3';

declare const Money3Component: Component;
declare const Money3Directive: Directive;
declare function format(input: string, opt: any): string;
declare function unformat(input: string): number;
// backwards compatibility
declare const Money3: Component;
declare const VMoney3: Directive;
declare const Money: Component;
declare const VMoney: Directive;

export default Plugin;
export {
  Money3Component,
  Money3Directive,
  format,
  unformat,
  Money3,
  VMoney3,
  Money,
  VMoney
}

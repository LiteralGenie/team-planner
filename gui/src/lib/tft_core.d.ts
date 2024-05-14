declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	
	
	export interface ISearchTeamsOptions {
	    team_size: number
	    num_champions: number
	    slots: Array<number[]>
	    traits: Map<number, string[]>
	
	    debug?: boolean
	}
	
	
	
	
	interface Team {
	    champion_ids: number[]
	}
	
	
	
	/**
	*/
	export class TeamFinder {
	  free(): void;
	/**
	* @returns {TeamFinder}
	*/
	  static new(): TeamFinder;
	/**
	* @param {any} options
	*/
	  reset(options: any): void;
	/**
	* @returns {any}
	*/
	  next(): any;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_teamfinder_free: (a: number) => void;
  readonly teamfinder_new: () => number;
  readonly teamfinder_reset: (a: number, b: number) => void;
  readonly teamfinder_next: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;

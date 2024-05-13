declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* @param {ISearchTeamsOptions} options
	* @returns {any}
	*/
	export function search_teams(options: ISearchTeamsOptions): any;
	
	
	export interface ISearchTeamsOptions {
	    team_size: number
	    num_champions: number
	    slots: Array<number[]>
	    traits: Map<number, string[]>
	
	    debug?: boolean
	}
	
	export type SearchTeams = (options: ISearchTeamsOptions) => Team[]
	
	
	
	
	
	interface Team {
	    champion_ids: number[]
	}
	
	
	
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly search_teams: (a: number) => number;
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

import { writable } from 'svelte/store';

export enum SequenceName {
	Fibonacci = 0,
	Primes,
	Lucas,
	Ramanujan,
	SHC,
	Pronic,
	Composite
}

export const sequence = writable<SequenceName>(SequenceName.Fibonacci);

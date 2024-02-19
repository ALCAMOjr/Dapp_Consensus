import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CrearPropuesta {
  'descripcion' : string,
  'is_active' : boolean,
}
export type Eleccion = { 'Paso' : null } |
  { 'Aprovar' : null } |
  { 'Rechazar' : null };
export interface Propuesta {
  'aprovados' : number,
  'rechazados' : number,
  'votos' : Array<Principal>,
  'descripcion' : string,
  'pasados' : number,
  'propietario' : Principal,
  'esta_activo' : boolean,
}
export type Result = { 'Ok' : null } |
  { 'Err' : VotoError };
export type VotoError = { 'EntradaInvalida' : null } |
  { 'VotoYaRealizado' : null } |
  { 'EleccionInvalida' : null } |
  { 'PropuestaInactiva' : null } |
  { 'Errorctualizacion' : null } |
  { 'AccesoDenegado' : null } |
  { 'PropuestaInexistente' : null };
export interface _SERVICE {
  'create_proposal' : ActorMethod<[bigint, CrearPropuesta], [] | [Propuesta]>,
  'end_proposal' : ActorMethod<[bigint], Result>,
  'get_proposal' : ActorMethod<[bigint], [] | [Propuesta]>,
  'get_proposal_account' : ActorMethod<[], bigint>,
  'update_proposal' : ActorMethod<[bigint, CrearPropuesta], Result>,
  'vote' : ActorMethod<[bigint, Eleccion], Result>,
}

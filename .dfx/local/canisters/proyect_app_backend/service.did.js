export const idlFactory = ({ IDL }) => {
  const CrearPropuesta = IDL.Record({
    'descripcion' : IDL.Text,
    'is_active' : IDL.Bool,
  });
  const Propuesta = IDL.Record({
    'aprovados' : IDL.Nat32,
    'rechazados' : IDL.Nat32,
    'votos' : IDL.Vec(IDL.Principal),
    'descripcion' : IDL.Text,
    'pasados' : IDL.Nat32,
    'propietario' : IDL.Principal,
    'esta_activo' : IDL.Bool,
  });
  const VotoError = IDL.Variant({
    'EntradaInvalida' : IDL.Null,
    'VotoYaRealizado' : IDL.Null,
    'EleccionInvalida' : IDL.Null,
    'PropuestaInactiva' : IDL.Null,
    'Errorctualizacion' : IDL.Null,
    'AccesoDenegado' : IDL.Null,
    'PropuestaInexistente' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : VotoError });
  const Eleccion = IDL.Variant({
    'Paso' : IDL.Null,
    'Aprovar' : IDL.Null,
    'Rechazar' : IDL.Null,
  });
  return IDL.Service({
    'create_proposal' : IDL.Func(
        [IDL.Nat64, CrearPropuesta],
        [IDL.Opt(Propuesta)],
        [],
      ),
    'end_proposal' : IDL.Func([IDL.Nat64], [Result], []),
    'get_proposal' : IDL.Func([IDL.Nat64], [IDL.Opt(Propuesta)], ['query']),
    'get_proposal_account' : IDL.Func([], [IDL.Nat64], ['query']),
    'update_proposal' : IDL.Func([IDL.Nat64, CrearPropuesta], [Result], []),
    'vote' : IDL.Func([IDL.Nat64, Eleccion], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };

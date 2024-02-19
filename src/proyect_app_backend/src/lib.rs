#[warn(unused_imports)]
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Alias para la memoria virtual con implementación predeterminada.
type Memoria = VirtualMemory<DefaultMemoryImpl>;

const TAMAÑO_VALOR_MAXIMO: u32 = 5000;

#[derive(CandidType, Deserialize)]
enum Eleccion {
    Aprovar,
    Rechazar,
    Paso,
}

#[derive(CandidType, Deserialize)]
enum VotoError {
    VotoYaRealizado,
    PropuestaInactiva,
    EleccionInvalida,
    PropuestaInexistente,
    AccesoDenegado,
    Errorctualizacion,
    EntradaInvalida,
}

#[derive(CandidType, Deserialize, Clone)]
struct Propuesta {
    descripcion: String,
    aprovados: u32,
    rechazados: u32,
    pasados: u32,
    esta_activo: bool,
    votos: Vec<candid::Principal>,
    propietario: candid::Principal,
}

#[derive(CandidType, Deserialize, Clone)]
struct CrearPropuesta {
    descripcion: String,
    is_active: bool, // Cambia esto a String
}

impl Storable for Propuesta {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Propuesta {
    const MAX_SIZE: u32 = TAMAÑO_VALOR_MAXIMO;
    const IS_FIXED_SIZE: bool = false;
}

// Variables thread-local para la gestión de memoria y mapas de datos.
thread_local! {
    // Gestor de memoria.
    static ADMIN_MEMORY: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Mapa estable que asocia IDs de usuarios con estructuras Exam.
    static MAPA_PROPUESTA: RefCell<StableBTreeMap<u64, Propuesta, Memoria>> = RefCell::new(StableBTreeMap::init(
        ADMIN_MEMORY.with(|m| m.borrow().get(MemoryId::new(0))),
    ));
}

#[ic_cdk::query]
fn get_proposal(key: u64) -> Option<Propuesta> {
    MAPA_PROPUESTA.with(|p| p.borrow().get(&key))
}

#[ic_cdk::query]
fn get_proposal_account() -> u64 {
    MAPA_PROPUESTA.with(|p| p.borrow().len())
}

#[ic_cdk::update]
fn create_proposal(key: u64, propuesta: CrearPropuesta) -> Option<Propuesta> {
    let valor: Propuesta = Propuesta {
        descripcion: propuesta.descripcion,
        aprovados: 0u32,
        rechazados: 0u32,
        pasados: 0u32,
        esta_activo: propuesta.is_active, // Usa el valor booleano aquí
        votos: vec![],
        propietario: ic_cdk::caller(),
    };

    MAPA_PROPUESTA.with(|p| p.borrow_mut().insert(key, valor))
}

#[ic_cdk::update]
fn update_proposal(key: u64, propuesta: CrearPropuesta) -> Result<(), VotoError> {
    MAPA_PROPUESTA.with(|p| {
        let propuesta_vieja_opt = p.borrow().get(&key);
        let propuesta_vieja: Propuesta;

        match propuesta_vieja_opt {
            Some(value) => propuesta_vieja = value,
            None => return Err(VotoError::PropuestaInexistente),
        }
        if ic_cdk::caller() != propuesta_vieja.propietario {
            return Err(VotoError::AccesoDenegado);
        }
        let valor: Propuesta = Propuesta {
            descripcion: propuesta.descripcion,
            aprovados: propuesta_vieja.aprovados,
            rechazados: propuesta_vieja.rechazados,
            pasados: propuesta_vieja.pasados,
            esta_activo: propuesta_vieja.esta_activo, // Usa el valor booleano aquí
            votos: propuesta_vieja.votos,
            propietario: ic_cdk::caller(),
        };
        let resultado = p.borrow_mut().insert(key, valor);
        match resultado {
            Some(_) => Ok(()),
            None => Err(VotoError::Errorctualizacion),
        }
    })
}

#[ic_cdk::update]
fn end_proposal(key: u64) -> Result<(), VotoError> {
    MAPA_PROPUESTA.with(|p| {
        let propuesta_opt = p.borrow().get(&key);
        let mut propuesta: Propuesta;

        match propuesta_opt {
            Some(value) => propuesta = value,
            None => return Err(VotoError::PropuestaInexistente),
        }
        if ic_cdk::caller() != propuesta.propietario {
            return Err(VotoError::AccesoDenegado);
        }
        propuesta.esta_activo = false;

        let resultado = p.borrow_mut().insert(key, propuesta);
        match resultado {
            Some(_) => Ok(()),
            None => Err(VotoError::Errorctualizacion),
        }
    })
}

#[ic_cdk::update]
fn vote(key: u64, eleccion: Eleccion) -> Result<(), VotoError> {
    MAPA_PROPUESTA.with(|p| {
        let propuesta_opt = p.borrow().get(&key);
        let mut propuesta: Propuesta;
        match propuesta_opt {
            Some(valor) => propuesta = valor,
            None => return Err(VotoError::PropuestaInexistente),
        };
        let votante= ic_cdk::caller();

        if propuesta.votos.contains(&votante) {
            return Err(VotoError::VotoYaRealizado);
        } else if propuesta.esta_activo == false {
            return Err(VotoError::PropuestaInexistente);
        }

        match eleccion {
            Eleccion::Aprovar => propuesta.aprovados += 1,
            Eleccion::Rechazar => propuesta.rechazados += 1,
            Eleccion::Paso => propuesta.pasados += 1,
        }

        propuesta.votos.push(votante);
        let resultado = p.borrow_mut().insert(key, propuesta);
        match resultado {
            Some(_) => Ok(()),
            None => Err(VotoError::Errorctualizacion),
            
        }
    })
}








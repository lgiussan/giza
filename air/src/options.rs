use core::ops::Deref;
use winter_air::{FieldExtension, HashFunction, ProofOptions as WinterProofOptions};

/// TODO: add docs
#[derive(Clone)]
pub struct ProofOptions(WinterProofOptions);

impl ProofOptions {
    pub fn new(
        num_queries: usize,
        blowup_factor: usize,
        grinding_factor: u32,
        hash_fn: HashFunction,
        field_extension: FieldExtension,
        fri_folding_factor: usize,
        fri_max_remainder_size: usize,
    ) -> Self {
        Self(WinterProofOptions::new(
            num_queries,
            blowup_factor,
            grinding_factor,
            hash_fn,
            field_extension,
            fri_folding_factor,
            fri_max_remainder_size,
        ))
    }

    // TODO: Use parameters appropriate for 252-bit field (see ethSTARK paper,
    // sections 5.10.1 and 5.10.2)
    pub fn with_96_bit_security() -> Self {
        Self(WinterProofOptions::new(
            54, // 27
            4,  //8,
            16,
            HashFunction::Blake3_192,
            FieldExtension::None,
            8,
            256,
        ))
    }

    pub fn into_inner(self) -> WinterProofOptions {
        self.0
    }
}

impl Default for ProofOptions {
    fn default() -> Self {
        Self::with_96_bit_security()
    }
}

impl Deref for ProofOptions {
    type Target = WinterProofOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//! The wire's own finite decimal, hand-written. Ethos's intrinsic `Decimal`
//! generates `datom_codec::Decimal`, which would carry datom-codec, and its
//! text parser, into every Nexus process that holds a `Pace`; Energy's
//! daemon carries no text dependency. So the Library imports this type,
//! `:crate:decimal:[ FiniteDecimal ]`, and only its datom impls sit behind
//! the `datom` feature, where its text is datom-codec's decimal.

/// A finite decimal: no `NaN`, no infinity, and no `-0.0` apart from `0.0`.
/// Its archive is one little-endian `f64`, and reading one back re-enters
/// `try_from`, so a non-finite archived float is refused.
#[derive(rkyv::Archive, rkyv::Serialize, Clone, Copy, Debug, Default)]
pub struct FiniteDecimal(f64);
/// A float that is not finite has no FiniteDecimal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NotFinite(pub f64);
impl std::fmt::Display for NotFinite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} is not a finite decimal", self.0)
    }
}
impl std::error::Error for NotFinite {}
impl TryFrom<f64> for FiniteDecimal {
    type Error = NotFinite;
    fn try_from(value: f64) -> Result<Self, NotFinite> {
        if value.is_finite() {
            Ok(Self(if value == 0.0 { 0.0 } else { value }))
        } else {
            Err(NotFinite(value))
        }
    }
}
/// A decimal yields the finite float it holds.
pub trait Decimating {
    fn float(&self) -> f64;
}
impl Decimating for FiniteDecimal {
    fn float(&self) -> f64 {
        self.0
    }
}
impl PartialEq for FiniteDecimal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
/// Sound because the value is finite.
impl Eq for FiniteDecimal {}
impl std::hash::Hash for FiniteDecimal {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.0.to_bits().hash(hasher)
    }
}
impl<D> rkyv::Deserialize<FiniteDecimal, D> for ArchivedFiniteDecimal
where
    D: rkyv::rancor::Fallible + ?Sized,
    D::Error: rkyv::rancor::Source,
{
    fn deserialize(&self, _deserializer: &mut D) -> Result<FiniteDecimal, D::Error> {
        let float = self.0.to_native();
        FiniteDecimal::try_from(float).map_err(<D::Error as rkyv::rancor::Source>::new)
    }
}
#[cfg(feature = "datom")]
impl datom_codec::Datomizable for FiniteDecimal {
    fn datomize(&self, at: datom_codec::Path) -> datom_codec::Datom {
        datom_codec::Datomizable::datomize(
            &datom_codec::Decimal::try_from(self.0).expect("a FiniteDecimal is finite"),
            at,
        )
    }
}
#[cfg(feature = "datom")]
impl datom_codec::Composing for FiniteDecimal {
    fn compose(
        datom: &datom_codec::Datom,
        budget: &mut datom_codec::Budget,
    ) -> Result<Self, datom_codec::Error> {
        use datom_codec::Decimating;
        <datom_codec::Decimal as datom_codec::Composing>::compose(datom, budget)
            .map(|decimal| Self(decimal.float()))
    }
}

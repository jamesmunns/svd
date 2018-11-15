use elementext::ElementExt;
#[cfg(feature = "unproven")]
use std::collections::HashMap;
use xmltree::Element;

use types::Parse;

#[cfg(feature = "unproven")]
use encode::Encode;
use error::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModifiedWriteValues {
    OneToClear,
    OneToSet,
    OneToToggle,
    ZeroToClear,
    ZeroToSet,
    Clear,
    Set,
    Modify,
}

impl Parse for ModifiedWriteValues {
    type Object = ModifiedWriteValues;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<ModifiedWriteValues, SVDError> {
        use self::ModifiedWriteValues::*;
        let text = tree.get_text()?;

        Ok(match text.as_ref() {
            "oneToClear" => OneToClear,
            "oneToSet" => OneToSet,
            "oneToToggle" => OneToToggle,
            "zeroToClear" => ZeroToClear,
            "zeroToSet" => ZeroToSet,
            "clear" => Clear,
            "set" => Set,
            "modify" => Modify,
            s => {
                return Err(SVDErrorKind::InvalidModifiedWriteValues(tree.clone(), s.into()).into())
            }
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for ModifiedWriteValues {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        use self::ModifiedWriteValues::*;
        let v = match *self {
            OneToClear => "oneToClear",
            OneToSet => "oneToSet",
            OneToToggle => "oneToToggle",
            ZeroToClear => "zeroToClear",
            ZeroToSet => "zeroToSet",
            Clear => "clear",
            Set => "set",
            Modify => "modify",
        };

        Ok(Element {
            name: String::from("modifiedWriteValues"),
            attributes: HashMap::new(),
            children: vec![],
            text: Some(v.into()),
        })
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
mod tests {
    use super::*;
    use run_test;

    #[test]
    fn decode_encode() {
        // FIXME: Do we need a more extensive test?
        let tests = vec![(
            ModifiedWriteValues::OneToToggle,
            "<modifiedWriteValues>oneToToggle</modifiedWriteValues>",
        )];

        run_test::<ModifiedWriteValues>(&tests[..]);
    }
}

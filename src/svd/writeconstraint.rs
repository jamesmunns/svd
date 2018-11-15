#[cfg(feature = "unproven")]
use std::collections::HashMap;

use elementext::ElementExt;
use xmltree::Element;

#[cfg(feature = "unproven")]
use encode::Encode;
use error::*;
#[cfg(feature = "unproven")]
use new_element;
use types::Parse;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WriteConstraint {
    WriteAsRead(bool),
    UseEnumeratedValues(bool),
    Range(WriteConstraintRange),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WriteConstraintRange {
    pub min: u32,
    pub max: u32,
}

impl Parse for WriteConstraint {
    type Object = WriteConstraint;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<WriteConstraint, SVDError> {
        if tree.children.len() == 1 {
            let ref field = tree.children[0].name;
            // Write constraint can only be one of the following
            match field.as_ref() {
                "writeAsRead" => Ok(WriteConstraint::WriteAsRead(
                    tree.get_child_bool(field.as_ref())?,
                )),
                "useEnumeratedValues" => Ok(WriteConstraint::UseEnumeratedValues(
                    tree.get_child_bool(field.as_ref())?,
                )),
                "range" => Ok(WriteConstraint::Range(WriteConstraintRange::parse(
                    tree.get_child_elem(field.as_ref())?,
                )?)),
                _ => Err(SVDErrorKind::UnknownWriteConstraint(tree.clone()).into()),
            }
        } else {
            Err(SVDErrorKind::MoreThanOneWriteConstraint(tree.clone()).into())
        }
    }
}

#[cfg(feature = "unproven")]
impl Encode for WriteConstraint {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let v = match *self {
            WriteConstraint::WriteAsRead(v) => new_element("writeAsRead", Some(format!("{}", v))),
            WriteConstraint::UseEnumeratedValues(v) => {
                new_element("useEnumeratedValues", Some(format!("{}", v)))
            }
            WriteConstraint::Range(v) => v.encode()?,
        };

        Ok(Element {
            name: String::from("writeConstraint"),
            attributes: HashMap::new(),
            children: vec![v],
            text: None,
        })
    }
}

impl Parse for WriteConstraintRange {
    type Object = WriteConstraintRange;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<WriteConstraintRange, SVDError> {
        Ok(WriteConstraintRange {
            min: tree.get_child_u32("minimum")?,
            max: tree.get_child_u32("maximum")?,
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for WriteConstraintRange {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        Ok(Element {
            name: String::from("range"),
            attributes: HashMap::new(),
            children: vec![
                new_element("minimum", Some(format!("0x{:08.x}", self.min))),
                new_element("maximum", Some(format!("0x{:08.x}", self.max))),
            ],
            text: None,
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
        let tests = vec![
            (
                WriteConstraint::WriteAsRead(true),
                "<writeConstraint><writeAsRead>true</writeAsRead></writeConstraint>"
            ),
            (
                WriteConstraint::UseEnumeratedValues(true),
                "<writeConstraint><useEnumeratedValues>true</useEnumeratedValues></writeConstraint>"
            ),
            (
                WriteConstraint::Range(WriteConstraintRange{min: 1, max: 10}),
                "<writeConstraint><range><minimum>0x00000001</minimum><maximum>0x0000000a</maximum></range></writeConstraint>"
            ),
        ];

        run_test::<WriteConstraint>(&tests[..]);
    }
}

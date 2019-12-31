// Autogenerated by Thrift Compiler (0.13.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate thrift;

use thrift::OrderedFloat;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{From, TryFrom};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TCompletionHintType {
  Column = 0,
  Table = 1,
  View = 2,
  Schema = 3,
  Catalog = 4,
  Repository = 5,
  Function = 6,
  Keyword = 7,
}

impl TCompletionHintType {
  pub fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    o_prot.write_i32(*self as i32)
  }
  pub fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<TCompletionHintType> {
    let enum_value = i_prot.read_i32()?;
    TCompletionHintType::try_from(enum_value)  }
}

impl TryFrom<i32> for TCompletionHintType {
  type Error = thrift::Error;  fn try_from(i: i32) -> Result<Self, Self::Error> {
    match i {
      0 => Ok(TCompletionHintType::Column),
      1 => Ok(TCompletionHintType::Table),
      2 => Ok(TCompletionHintType::View),
      3 => Ok(TCompletionHintType::Schema),
      4 => Ok(TCompletionHintType::Catalog),
      5 => Ok(TCompletionHintType::Repository),
      6 => Ok(TCompletionHintType::Function),
      7 => Ok(TCompletionHintType::Keyword),
      _ => {
        Err(
          thrift::Error::Protocol(
            ProtocolError::new(
              ProtocolErrorKind::InvalidData,
              format!("cannot convert enum constant {} to TCompletionHintType", i)
            )
          )
        )
      },
    }
  }
}

//
// TCompletionHint
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TCompletionHint {
  pub type_: Option<TCompletionHintType>,
  pub hints: Option<Vec<String>>,
  pub replaced: Option<String>,
}

impl TCompletionHint {
  pub fn new<F1, F2, F3>(type_: F1, hints: F2, replaced: F3) -> TCompletionHint where F1: Into<Option<TCompletionHintType>>, F2: Into<Option<Vec<String>>>, F3: Into<Option<String>> {
    TCompletionHint {
      type_: type_.into(),
      hints: hints.into(),
      replaced: replaced.into(),
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<TCompletionHint> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<TCompletionHintType> = None;
    let mut f_2: Option<Vec<String>> = Some(Vec::new());
    let mut f_3: Option<String> = Some("".to_owned());
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = TCompletionHintType::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<String> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_0 = i_prot.read_string()?;
            val.push(list_elem_0);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        3 => {
          let val = i_prot.read_string()?;
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = TCompletionHint {
      type_: f_1,
      hints: f_2,
      replaced: f_3,
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("TCompletionHint");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.type_ {
      o_prot.write_field_begin(&TFieldIdentifier::new("type", TType::I32, 1))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.hints {
      o_prot.write_field_begin(&TFieldIdentifier::new("hints", TType::List, 2))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::String, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_string(e)?;
        o_prot.write_list_end()?;
      }
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.replaced {
      o_prot.write_field_begin(&TFieldIdentifier::new("replaced", TType::String, 3))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Default for TCompletionHint {
  fn default() -> Self {
    TCompletionHint{
      type_: None,
      hints: Some(Vec::new()),
      replaced: Some("".to_owned()),
    }
  }
}


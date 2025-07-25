/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use ruff_python_ast::CmpOp;
use ruff_python_ast::name::Name;

pub const AENTER: Name = Name::new_static("__aenter__");
pub const AEXIT: Name = Name::new_static("__aexit__");
pub const ALL: Name = Name::new_static("__all__");
pub const BOOL: Name = Name::new_static("__bool__");
pub const CALL: Name = Name::new_static("__call__");
pub const CONTAINS: Name = Name::new_static("__contains__");
pub const DATACLASS_FIELDS: Name = Name::new_static("__dataclass_fields__");
pub const DELATTR: Name = Name::new_static("__delattr__");
pub const DELITEM: Name = Name::new_static("__delitem__");
pub const DOC: Name = Name::new_static("__doc__");
pub const ENTER: Name = Name::new_static("__enter__");
pub const EQ: Name = Name::new_static("__eq__");
pub const EXIT: Name = Name::new_static("__exit__");
pub const GE: Name = Name::new_static("__ge__");
pub const GET: Name = Name::new_static("__get__");
pub const GETATTR: Name = Name::new_static("__getattr__");
pub const GETATTRIBUTE: Name = Name::new_static("__getattribute__");
pub const GETITEM: Name = Name::new_static("__getitem__");
pub const GT: Name = Name::new_static("__gt__");
pub const HASH: Name = Name::new_static("__hash__");
pub const INIT: Name = Name::new_static("__init__");
pub const INIT_SUBCLASS: Name = Name::new_static("__init_subclass__");
pub const INVERT: Name = Name::new_static("__invert__");
pub const ITER: Name = Name::new_static("__iter__");
pub const LE: Name = Name::new_static("__le__");
pub const LT: Name = Name::new_static("__lt__");
pub const MATCH_ARGS: Name = Name::new_static("__match_args__");
pub const NE: Name = Name::new_static("__ne__");
pub const NEG: Name = Name::new_static("__neg__");
pub const NEW: Name = Name::new_static("__new__");
pub const NEXT: Name = Name::new_static("__next__");
pub const POS: Name = Name::new_static("__pos__");
pub const POST_INIT: Name = Name::new_static("__post_init__");
pub const SET: Name = Name::new_static("__set__");
pub const SETATTR: Name = Name::new_static("__setattr__");
pub const SETITEM: Name = Name::new_static("__setitem__");
pub const SLOTS: Name = Name::new_static("__slots__");

pub const RICH_CMPS: &[Name] = &[LT, LE, EQ, NE, GT, GE];
/// Rich comparison methods supplied by the `functools.total_ordering` decorator
pub const RICH_CMPS_TOTAL_ORDERING: &[Name] = &[LT, LE, GT, GE];

/// Returns the associated dunder if `op` corresponds to a "rich comparison method":
/// https://docs.python.org/3/reference/datamodel.html#object.__lt__.
pub fn rich_comparison_dunder(op: CmpOp) -> Option<Name> {
    let name = match op {
        CmpOp::Lt => LT,
        CmpOp::LtE => LE,
        CmpOp::Eq => EQ,
        CmpOp::NotEq => NE,
        CmpOp::Gt => GT,
        CmpOp::GtE => GE,
        _ => return None,
    };
    Some(name)
}

/// Returns the fallback dunder if `op` corresponds to a "rich comparison method":
/// https://docs.python.org/3/reference/datamodel.html#object.__lt__.
pub fn rich_comparison_fallback(op: CmpOp) -> Option<Name> {
    let name = match op {
        CmpOp::Lt => GT,
        CmpOp::LtE => GE,
        CmpOp::Eq => NE,
        CmpOp::NotEq => EQ,
        CmpOp::Gt => LT,
        CmpOp::GtE => LE,
        _ => return None,
    };
    Some(name)
}

use ::tel_api::TelFile;

use crate::ast::AssignmentDest;
use crate::ast::AssignmentKw;
use crate::ast::Assignments;
use crate::ast::Ast;
use crate::ast::Block;
use crate::ast::Expr;
pub use crate::scoping::scope::LinearScope;
pub use crate::scoping::scope::Scope;
use crate::TelErr;

mod scope;

pub fn ast_to_api(ast: Ast) -> Result<TelFile, TelErr> {
    let Ast { blocks } = ast;
    let mut global_scope = <LinearScope as Scope>::new();
    let blocks = blocks.into_vec();  //TODO @mark: TEMPORARY! REMOVE THIS!
    for block in blocks.into_iter() {
        // let block: Block = block;  // enforce that `block` is not borrowed
        //TODO @mark: ^ enable this and remove clones
        match block {
            Block::Assigns(assign) => assignments_to_api(assign, &mut global_scope)?,
            Block::Expression(_expression) => todo!(),
            Block::Struct(_struct) => todo!(),
            Block::Enum(_enum) => todo!(),
        }
    }
    Ok(TelFile {})
}

fn assignments_to_api(
    assign: Assignments,
    scopes: &mut impl Scope,
) -> Result<(), TelErr> {
    let Assignments { dest: dests, op, value } = assign;
    debug_assert!(dests.len() >= 1);
    if let Some(_op) = op {
        todo!()
    }
    for dest in dests.into_iter() {
        // let dest: AssignmentDest = dest;  // enforce that `dest` is not borrowed
        //TODO @mark: ^ enable this and pass owned values to get_or_insert
        let AssignmentDest { kw, target, typ } = dest;
        let mutable = match kw {
            AssignmentKw::None => false,
            AssignmentKw::Outer => todo!(),
            AssignmentKw::Local => todo!(),
            AssignmentKw::Mut => true,
        };
        let binding = scopes.get_or_insert(
            target,
            typ.as_ref(),
            mutable
        );
        todo!();
        let expr = expression_to_api(value)?;
    }
    todo!()
}

fn expression_to_api(expr: Box<Expr>) -> Result<(), TelErr> {
    todo!()

}

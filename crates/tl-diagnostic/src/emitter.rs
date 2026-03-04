use crate::DiagnosticItem;

pub trait DiagnosticEmitter {
    fn emit(&mut self, item: DiagnosticItem);
}

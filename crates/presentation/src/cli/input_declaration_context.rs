use crate::application::dtos::responses::RunInputDeclarationResponse;

pub(super) struct InputDeclarationContext<'a> {
    declaration: &'a RunInputDeclarationResponse,
    index: usize,
    total: usize,
    interactive: bool,
}

impl<'a> InputDeclarationContext<'a> {
    pub(super) fn new(
        declaration: &'a RunInputDeclarationResponse,
        index: usize,
        total: usize,
        interactive: bool,
    ) -> Self {
        Self {
            declaration,
            index,
            total,
            interactive,
        }
    }

    pub(super) fn declaration(&self) -> &RunInputDeclarationResponse {
        self.declaration
    }

    pub(super) fn index(&self) -> usize {
        self.index
    }

    pub(super) fn total(&self) -> usize {
        self.total
    }

    pub(super) fn interactive(&self) -> bool {
        self.interactive
    }
}

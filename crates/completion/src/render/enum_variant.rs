use hir::{HasAttrs, HirDisplay, ModPath, StructKind};
use itertools::Itertools;
use test_utils::mark;

use crate::{
    item::{CompletionItem, CompletionItemKind, CompletionKind},
    render::{builder_ext::Params, RenderContext},
};

#[derive(Debug)]
pub(crate) struct EnumVariantRender<'a> {
    ctx: RenderContext<'a>,
    name: String,
    variant: hir::EnumVariant,
    path: Option<ModPath>,
    qualified_name: String,
    short_qualified_name: String,
    variant_kind: StructKind,
}

impl<'a> EnumVariantRender<'a> {
    pub(crate) fn new(
        ctx: RenderContext<'a>,
        local_name: Option<String>,
        variant: hir::EnumVariant,
        path: Option<ModPath>,
    ) -> EnumVariantRender<'a> {
        let name = local_name.unwrap_or_else(|| variant.name(ctx.db()).to_string());
        let variant_kind = variant.kind(ctx.db());

        let (qualified_name, short_qualified_name) = match &path {
            Some(path) => {
                let full = path.to_string();
                let short =
                    path.segments[path.segments.len().saturating_sub(2)..].iter().join("::");
                (full, short)
            }
            None => (name.to_string(), name.to_string()),
        };

        EnumVariantRender {
            ctx,
            name,
            variant,
            path,
            qualified_name,
            short_qualified_name,
            variant_kind,
        }
    }

    pub(crate) fn render(self) -> CompletionItem {
        let mut builder = CompletionItem::new(
            CompletionKind::Reference,
            self.ctx.source_range(),
            self.qualified_name.clone(),
        )
        .kind(CompletionItemKind::EnumVariant)
        .set_documentation(self.variant.docs(self.ctx.db()))
        .set_deprecated(self.ctx.is_deprecated(self.variant))
        .detail(self.detail());

        if self.variant_kind == StructKind::Tuple {
            mark::hit!(inserts_parens_for_tuple_enums);
            let params = Params::Anonymous(self.variant.fields(self.ctx.db()).len());
            builder =
                builder.add_call_parens(self.ctx.completion, self.short_qualified_name, params);
        } else if self.path.is_some() {
            builder = builder.lookup_by(self.short_qualified_name);
        }

        builder.build()
    }

    fn detail(&self) -> String {
        let detail_types = self
            .variant
            .fields(self.ctx.db())
            .into_iter()
            .map(|field| (field.name(self.ctx.db()), field.signature_ty(self.ctx.db())));

        match self.variant_kind {
            StructKind::Tuple | StructKind::Unit => format!(
                "({})",
                detail_types.map(|(_, t)| t.display(self.ctx.db()).to_string()).format(", ")
            ),
            StructKind::Record => format!(
                "{{ {} }}",
                detail_types
                    .map(|(n, t)| format!("{}: {}", n, t.display(self.ctx.db()).to_string()))
                    .format(", ")
            ),
        }
    }
}

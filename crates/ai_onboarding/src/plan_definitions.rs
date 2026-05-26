use gpui::{IntoElement, ParentElement};
use ui::{List, ListBulletItem, l10n, prelude::*};

/// Centralized definitions for Zed AI plans
pub struct PlanDefinitions;

impl PlanDefinitions {
    pub fn free_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(l10n::text(
                "2,000 accepted edit predictions",
            )))
            .child(ListBulletItem::new(l10n::text(
                "Unlimited prompts with your AI API keys",
            )))
            .child(ListBulletItem::new(l10n::text(
                "Unlimited use of external agents",
            )))
    }

    pub fn sign_in_upsell(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(l10n::text(
                "Unlimited edit predictions",
            )))
            .child(ListBulletItem::new(l10n::text(
                "$20 of tokens in Zed agent",
            )))
            .child(ListBulletItem::new(l10n::text("No credit card required")))
    }

    pub fn pro_trial(&self, period: bool) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(l10n::text(
                "$20 of tokens in Zed agent",
            )))
            .child(ListBulletItem::new(l10n::text(
                "Unlimited edit predictions",
            )))
            .when(period, |this| {
                this.child(ListBulletItem::new(l10n::text(
                    "Try it out for 14 days, no credit card required",
                )))
            })
    }

    pub fn pro_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(l10n::text("$5 of tokens in Zed agent")))
            .child(ListBulletItem::new(l10n::text(
                "Usage-based billing beyond $5",
            )))
            .child(ListBulletItem::new(l10n::text(
                "Unlimited edit predictions",
            )))
    }

    pub fn business_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(l10n::text(
                "Unlimited edit predictions",
            )))
            .child(ListBulletItem::new(l10n::text("Usage-based billing")))
    }

    pub fn student_plan(&self) -> impl IntoElement {
        List::new()
            .child(ListBulletItem::new(l10n::text(
                "Unlimited edit predictions",
            )))
            .child(ListBulletItem::new(l10n::text(
                "$10 of tokens in Zed agent",
            )))
            .child(ListBulletItem::new(l10n::text(
                "Optional credit packs for additional usage",
            )))
    }
}

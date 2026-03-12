use yew::prelude::*;

#[derive(PartialEq, Clone, Default)]
pub enum BannerVariant {
    #[default]
    Error,
    Success,
}

#[derive(Properties, PartialEq)]
pub struct BannerProps {
    pub message: Option<String>,
    pub on_dismiss: Callback<()>,
    #[prop_or_default]
    pub variant: BannerVariant,
}

#[component]
pub fn Banner(props: &BannerProps) -> Html {
    let Some(msg) = &props.message else {
        return html! {};
    };

    let (bg, border, shadow) = match props.variant {
        BannerVariant::Error => (
            "rgba(220, 38, 38, 0.25)",
            "rgba(220, 38, 38, 0.4)",
            "0 4px 20px rgba(220, 38, 38, 0.3)",
        ),
        BannerVariant::Success => (
            "rgba(22, 163, 74, 0.25)",
            "rgba(22, 163, 74, 0.4)",
            "0 4px 20px rgba(22, 163, 74, 0.3)",
        ),
    };

    let on_dismiss = props.on_dismiss.clone();
    html! {
        <div role="alert" style={format!("
            position: fixed;
            top: 1rem;
            left: 50%;
            transform: translateX(-50%);
            z-index: 1000;
            display: flex;
            align-items: center;
            gap: 1rem;
            padding: 0.75rem 1.25rem;
            border-radius: 0.5rem;
            border: 1px solid {border};
            background: {bg};
            backdrop-filter: blur(12px);
            -webkit-backdrop-filter: blur(12px);
            color: #fff;
            font-weight: 500;
            box-shadow: {shadow};
            max-width: 90vw;
        ")}>
            { msg.clone() }
            <button
                type="button"
                onclick={move |_| on_dismiss.emit(())}
                style="
                    background: none;
                    border: none;
                    color: inherit;
                    cursor: pointer;
                    font-size: 1rem;
                    padding: 0;
                    line-height: 1;
                    opacity: 0.8;
                "
            >
                { "✕" }
            </button>
        </div>
    }
}

use crate::components::{accordion::Accordion, user_input::internal_sparql::GraphDataContext};
use leptos::prelude::*;

#[component]
pub fn Title(#[prop(into)] title: Signal<String>) -> impl IntoView {
    view! {
                        <p class="py-4 font-thin text-center text-gray-500 text-[1.5em]">
                    {move || title.get()}
                </p>
    }
}

#[component]
pub fn DocumentBase(#[prop(into)] base: Signal<String>) -> impl IntoView {
    view! {
        <p class="flex gap-2 justify-center items-center py-2 my-2 text-sm text-gray-500">
            <a
                href=move || base.get()
                target="_blank"
                class="text-blue-600 hover:underline"
            >
                {move || base.get()}
            </a>
        </p>
    }
}

#[component]
pub fn Version(
    #[prop(into)] version_iri: Signal<Option<String>>,
    #[prop(into)] prior_version: Signal<Option<String>>,
    #[prop(into)] incompatible_with: Signal<Option<String>>,
    #[prop(into)] backward_compatible_with: Signal<Option<String>>,
) -> impl IntoView {
    let ontologyversion = RwSignal::new("0.99".to_string());
    view! {
        <p class="flex gap-2 justify-center items-center py-2 my-2 text-sm text-gray-500">
            "Version: "{move || version_iri.get().unwrap_or("None".to_string())}
            <br/>
            "Prior Version: "{move || prior_version.get().unwrap_or("None".to_string())}
            <br/>
            "Incompatible With: "{move || incompatible_with.get().unwrap_or("None".to_string())}
            <br/>
            "Backward Compatible With: "{move || backward_compatible_with.get().unwrap_or("None".to_string())}
        </p>
    }
}

#[component]
pub fn Author() -> impl IntoView {
    let ontologyauthors = RwSignal::new("Alice, Bob, Charlie".to_string());
    view! {
        <p class="flex gap-2 justify-center items-center py-2 my-2 text-sm text-gray-500">
            Author(s): {move || ontologyauthors.get()}
        </p>
    }
}

#[component]
pub fn Language() -> impl IntoView {
    let ontologylanguages = RwSignal::new(vec![
        "english".to_string(),
        "german".to_string(),
        "french".to_string(),
    ]);
    view! {
        <p class="flex gap-2 justify-center items-center py-2 my-2 text-sm text-gray-500">
            "Language(s):"
            <select class="py-1 px-2 text-sm text-gray-500 rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none w-[100px] h-[30px]">
                {move || {
                    ontologylanguages
                        .get()
                        .into_iter()
                        .map(|lang| view! { <option>{lang}</option> })
                        .collect_view()
                }}
            </select>
        </p>
    }
}

#[component]
pub fn Description() -> impl IntoView {
    let ontologydescription = RwSignal::new("The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.".to_string());
    view! {
        <Accordion title="Description">
            <p>{move || ontologydescription.get()}</p>
        </Accordion>
    }
}

#[component]
pub fn OntologyHeader() -> impl IntoView {
    let GraphDataContext { graph_metadata, .. } = expect_context::<GraphDataContext>();

    let document_base = create_read_slice(graph_metadata, |graph_metadata| {
        graph_metadata.document_base.clone()
    });
    let description = create_read_slice(graph_metadata, |graph_metadata| {
        graph_metadata.description.clone()
    });
    let version_iri = create_read_slice(graph_metadata, |graph_metadata| {
        graph_metadata.version_iri.clone()
    });
    let prior_version = create_read_slice(graph_metadata, |graph_metadata| {
        graph_metadata.prior_version.clone()
    });
    let incompatible_with = create_read_slice(graph_metadata, |graph_metadata| {
        graph_metadata.incompatible_with.clone()
    });
    let backward_compatible_with = create_read_slice(graph_metadata, |graph_metadata| {
        graph_metadata.backward_compatible_with.clone()
    });

    view! {
        <div>
                <Title title="Implement" />
                <DocumentBase base=document_base />
                <Version version_iri=version_iri prior_version=prior_version incompatible_with=incompatible_with backward_compatible_with=backward_compatible_with />
                <Author />
                <Language />
                <Description />
            </div>

    }
}

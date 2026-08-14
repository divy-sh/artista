use dioxus::prelude::*;

use daisy_rsx::*;

#[component]
pub fn ModelCard(model_name: String) -> Element {
    rsx! {
        div { class: "collapse bg-base-100 border border-base-300",
            input { r#type: "checkbox" }
            div { class: "collapse-title text-xl font-medium", "{model_name}" }
            div { class: "collapse-content",
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipisicing elit. 
                    Ab, animi asperiores atque autem commodi cumque delectus doloremque
                    eaque eligendi eos est eveniet exercitationem explicabo facilis fuga harum
                    illo impedit inventore ipsa itaque iure iusto laboriosam laborum magni 
                    maiores maxime minima molestiae natus necessitatibus nesciunt nihil nobis 
                    non numquam obcaecati officia omnis optio pariatur placeat possimus praesentium 
                    quaerat quasi quibusdam quisquam quo quos ratione recusandae rem repellendus 
                    reprehenderit rerum saepe sapiente sequi similique sint soluta sunt tempora 
                    temporibus tenetur totam ullam unde ut velit veniam veritatis vero voluptates 
                    voluptatibus voluptatum. Amet, architecto asperiores atque autem beatae commodi 
                    consequatur consequuntur corporis cumque deleniti deserunt dicta dignissimos 
                    dolorem dolores dolorum ea earum eius eligendi enim eos error est et eum eveniet 
                    exercitationem explicabo facere facilis fugiat harum hic illo impedit incidunt 
                    inventore ipsa iste itaque iure iusto laboriosam laborum magnam maiores maxime 
                    minima molestiae natus necessitatibus nesciunt nihil nobis non numquam obcaecati 
                    officia omnis optio pariatur placeat possimus praesentium quaerat quasi quibusdam 
                    quisquam quo quos ratione recusandae rem repellendus reprehenderit rerum saepe 
                    sapiente sequi similique sint soluta sunt tempora temporibus tenetur totam ullam 
                    unde ut velit veniam veritatis vero voluptates voluptatibus voluptatum."
                }
            }
        }
    }
}

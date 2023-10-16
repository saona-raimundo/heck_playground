#![forbid(unsafe_code)]

use leptos::*;
use heck::{
	ToKebabCase,
	ToLowerCamelCase,
	ToShoutyKebabCase,
	ToShoutySnakeCase,
	ToSnakeCase,
	ToTitleCase,
	ToTrainCase,
	ToUpperCamelCase,
};



fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());

    let description = view! {

    	<h2>{"Definition of a word boundary"}</h2>
    	<p>{"Word boundaries are defined as the “unicode words” defined in the unicode_segmentation library, as well as within those words in this manner:"}</p>
    	<ol>
    		<li>{"All underscore characters are considered word boundaries."}</li>
    		<li>{"If an uppercase character is followed by lowercase letters, a word boundary is considered to be just prior to that uppercase character."}</li>
    		<li>{"If multiple uppercase characters are consecutive, they are considered to be within a single word, except that the last will be part of the next word if it is followed by lowercase characters (see rule 2)."}</li>
    	</ol>
		<p>{"That is, “HelloWorld” is segmented `Hello|World` whereas “XMLHttpRequest” is segmented `XML|Http|Request`."}</p>    
		<p>{"Characters not within words (such as spaces, punctuations, and underscores) are not included in the output string except as they are a part of the case being converted to. Multiple adjacent word boundaries (such as a series of underscores) are folded into one. (“hello__world” in snake case is therefore “hello_world”, not the exact same string). Leading or trailing word boundary indicators are dropped, except insofar as CamelCase capitalizes the first word."}</p>    
    };

    view! {
		<h1>{"Case conversion"}</h1>
    	<div class="flex-container" style="display:flex; flex-wrap: wrap;">
	    	<textarea 
	    		autofocus 
	    		style="flex: 10;"
	    		placeholder="Text for case changing."
	    		on:input=move |ev| {
		            // event_target_value is a Leptos helper function
		            // it functions the same way as event.target.value
		            // in JavaScript, but smooths out some of the typecasting
		            // necessary to make this work in Rust
		            set_input.set(event_target_value(&ev));
		        }
		        // the `prop:` syntax lets you update a DOM property,
		        // rather than an attribute.
		        prop:value=input
	    	>
	    		{move || input.get()}
	    	</textarea>

	    	<div style="display:flex; flex-direction: column; flex: 1;">
		        <button
		            on:click=move |_| {
		                set_output.set(input.get().to_upper_camel_case());
		            }
		        >
		            "UpperCamelCase"
		        </button>

		        <button
		            on:click=move |_| {
		                set_output.set(input.get().to_lower_camel_case());
		            }
		        >
		            "lowerCamelCase"
		        </button>
		    
		        <button
		            on:click=move |_| {
		                set_output.set(input.get().to_snake_case());
		            }
		        >
		            "snake_case"
		        </button>
		    
		        <button
		            on:click=move |_| {
		                set_output.set(input.get().to_kebab_case());
		            }
		        >
		            "kebab-case"
		        </button>

		        <button
		            on:click=move |_| {
		                set_output.set(input.get().to_shouty_snake_case());
		            }
		        >
		            "SHOUTY_SNAKE_CASE"
		        </button>

		        <button
		            on:click=move |_| {
		                set_output.set(input.get().to_title_case());
		            }
		        >
		            "Title Case"
		        </button>   

		        <button
		            on:click=move |_| {
		                set_output.set(input.get().to_title_case().to_lowercase());
		            }
		        >
		            "lower case"
		        </button>  

		        <button
		            on:click=move |_| {
		                set_output.set(input.get().to_title_case().to_uppercase());
		            }
		        >
		            "UPPER CASE"
		        </button>  

		        <button
		            on:click=move |_| {
		                set_output.set(input.get().to_shouty_kebab_case());
		            }
		        >
		            "SHOUTY-KEBAB-CASE"
		        </button>       

		        <button
		            on:click=move |_| {
		                set_output.set(input.get().to_train_case());
		            }
		        >
		            "Train-Case"
		        </button>     
	        </div>

	        <textarea 
	        	readonly
	        	style="flex: 10;"
	    		placeholder="Output."
        	>
        		{move || output.get()}
        	</textarea>
        </div>
        {description}

		<footer id="footer" name="footnote">
			<p>{"Source code: "}<a href="https://github.com/saona-raimundo/heck_playground">{"GitHub"}</a></p>
			<p>{"Author: "}<a href="https://saona-raimundo.github.io/">{"Raimundo Saona"}</a></p>
		</footer>
    }
}
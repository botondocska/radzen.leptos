# A client-side rendered (CSR) Leptos port of the Radzen Blazor UI library
This was my way to begin learning Leptos... How about another webassembly framework in c next, jk. I enjoy working with both Blazor and Leptos, and I am hoping this UI library can become the home of many-many components, so everyone becomes more keen on making web based frontend a better place. - Rust can garantee so much instead of javascript, why waste all of that? I note that 99% of this is ai generated code. The optimisation phase will be the one where I will consider rewriting all components to be much more idiomatic to how Leptos wants me to handle properties and logic.

The original Radzen library can be found here: https://blazor.radzen.com/
And Leptos is here: https://github.com/leptos-rs/leptos

## The original workflow:
- lets copy paste the scss files and symbols, character files:
- The components are c# codes and razor files, with lots of inline css.
- Since both frameworks are using webassembly, and both rust and C# are typesafe languages, its easier to make the similar components with similar logic.

### The flow of implementing a component is as follows:
- if the component has dependencies not implemented, we shouldnt start implementing given component at all - we have to map recursively what components use what - then start with the one that only has already implemented dependencies, or no dependencies at all.
    - We have validate every component implemented, based on the c# version: no features can be missing at the end:  
        - We need to create a list of all features (and I mean everything from accessibility, css classes, properties, functionalities - if there is more I should put it here) that exist in the original component: compare them one by one wheter they exist in the rust version or not.
        - Making notes on what feauters are different from just "translating the code":
            - The original radzen library has a js library as well: I was not planning on adding that, instead use webassembly. 
            - Some built in Blazor functionalities, that have to be solved in a different matter - these are dangerous ones, so I have to approach it with caution
            - C# "perks" - usually inheritance and interfaces can be challenging - Traits only exist for structs, and Leptos uses functions - (macros?) 
            - There could be some I haven't came accross yet. These should be noted going forward.


## Only after implementing every component in the library:
- Optimize, to make ease of use, and the library itself cleaner. - this includes simplifying structures, reconsidering logics, finding other leptos ui libraries and looking at their way of using the leptos features, considering the differences between our, and their components.
  - Here we follow the same logic of iterating over each component, mapping recursively what components use what - then start with the one that only has already implemented dependencies, or no dependencies at all. - and compare the all in all structures - which approach results in more "flexibility" in the component.
    - also checking wheter we are missing any functionalities that we could implement. note that nor do I nor does AI know the proper css classes to be used from the scss files.

## Ideologies:
- I want to add proper testing, with CI/CD - should probably implement it soon, adding automatic formatting, auditing etc. (I will check how Zero to production in rust by Luca Palmieri does it, it can't be much different here.)
- Making a proper presentational website, where I present all component's usage examples with view and code. (some virtual machine needed perhaps)

- I want to try to make a copy of the project instead of scss to tailwind since its way more lightweight (in production), and much more flexible. It would simply a better stack.



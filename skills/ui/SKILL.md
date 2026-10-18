---
name: ui
description: Build exceptional frontend UI/UX with a polished, production-grade visual standard. Use when asked to design, implement, improve, restyle, or review user interfaces, frontend components, pages, dashboards, apps, design systems, interaction states, responsive layouts, or visual quality, especially when the user asks for beautiful, intuitive, slick, premium, Stripe-level, world-class, polished, or high-conversion UI/UX.
---

# UI

## Operating Standard

Treat UI work as product design and frontend engineering together. Produce interfaces that feel deliberate, clear, fast, and premium while using the libraries, components, tokens, icons, routes, data-fetching patterns, and design conventions already present in the project.

Do not settle for a feature that merely works. Make the primary workflow obvious, reduce friction, cover interaction states, and make the screen visually balanced across realistic viewport sizes.

## Workflow

1. Inspect the existing app before designing:
   - Identify the framework, routing model, styling system, component library, icon set, animation utilities, form libraries, data hooks, and existing page patterns.
   - Reuse established primitives before adding new ones.
   - Preserve the product's current visual language unless the user explicitly asks for a redesign.

2. Define the target experience:
   - Identify the user's goal, the primary action, secondary actions, and the information hierarchy.
   - Remove unnecessary explanatory text from the interface itself.
   - Prefer direct manipulation, clear controls, and progressive disclosure over dense instruction copy.

3. Design the component or screen:
   - Establish layout first: spacing, alignment, grouping, scan paths, empty space, density, and responsive behavior.
   - Choose typography intentionally: restrained sizes, clear hierarchy, no viewport-scaled font sizes, no negative letter spacing.
   - Use color with purpose: state, emphasis, hierarchy, and brand fit. Avoid one-note palettes and decorative gradients unless they serve the product.
   - Make controls feel tactile: clear hover, focus, pressed, disabled, loading, success, empty, and error states.

4. Implement with project-native tools:
   - Use the app's existing component abstractions, style tokens, utility classes, themes, and icon package.
   - Use proven libraries already installed for complex UI behavior such as menus, dialogs, command palettes, drag-and-drop, charts, calendars, animation, validation, or virtualized lists.
   - Add dependencies only when the existing stack cannot support the expected quality or accessibility.

5. Verify the result:
   - Run the relevant formatter, typecheck, lint, unit tests, or app build.
   - For browser UI, run the app and inspect desktop and mobile viewports with screenshots when feasible.
   - Check that text does not overflow, controls do not shift unexpectedly, focus states are visible, and loading/empty/error states are coherent.

## Quality Bar

Aim for a Stripe-level feel by making these details explicit in the implementation:

- **Hierarchy:** The most important action and content should be visually obvious within one second.
- **Composition:** Align edges, use consistent spacing, avoid accidental centering, and make dense screens scannable.
- **Restraint:** Prefer crisp surfaces, subtle borders, calibrated shadows, and purposeful motion over heavy decoration.
- **Affordance:** Buttons, inputs, tabs, menus, toggles, sliders, and segmented controls should look and behave like their function.
- **Feedback:** Every async action should have a clear in-progress and completed state. Every destructive action should be hard to trigger accidentally.
- **Accessibility:** Use semantic elements, keyboard navigation, focus management, accessible names, sufficient contrast, and reduced-motion-friendly animation.
- **Responsiveness:** Design for real content on narrow, medium, and wide screens. Stable UI elements should have stable dimensions.
- **Polish:** Refine icons, microcopy, spacing, empty states, skeletons, validation, transitions, and edge cases instead of leaving generic defaults.

## Component Guidance

- Use icons inside icon-only or tool buttons when a familiar symbol exists; provide tooltips for unfamiliar icon-only controls.
- Use tabs for view changes, segmented controls for modes, toggles or checkboxes for binary settings, sliders or numeric inputs for numeric settings, and menus for larger option sets.
- Use cards for repeated items, modals, and genuinely framed tools. Avoid cards inside cards and avoid turning whole page sections into floating cards.
- Keep card radii modest unless the existing design system says otherwise.
- Do not create marketing-style hero sections for apps, tools, dashboards, or games unless the user asks for a landing page.
- For dashboards and operational tools, prioritize dense but calm information design, quick comparison, predictable navigation, and repeated-use efficiency.
- For games, creative tools, portfolios, product pages, or editorial experiences, allow more expressive visuals while preserving clarity and usability.

## Visual Assets

Use meaningful visual assets when they improve comprehension or product feel. Prefer actual product/place/object/state imagery or generated bitmap assets over generic abstract decoration. Do not use decorative gradient orbs, bokeh blobs, or purely atmospheric imagery as a substitute for useful visual content.

## Review Checklist

Before finishing UI work, check:

- The first screen exposes the actual product/workflow, not a placeholder or marketing wrapper.
- The primary action is clear, reachable, and not competing with too many equivalent actions.
- The interface handles loading, empty, error, disabled, active, hover, focus, and selected states.
- Layout holds up with long labels, realistic data, and mobile widths.
- The implementation uses existing project libraries and patterns.
- The result has been viewed or otherwise verified, not only compiled.

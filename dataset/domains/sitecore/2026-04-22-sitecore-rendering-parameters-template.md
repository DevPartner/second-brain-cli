---
type: "zknotes"
date: 2026-04-22 1776866615.797
tags: [sitecore, rendering-parameters, template]
---

# Sitecore Rendering parameters template

## Source

From: [Website](https://learning.sitecore.com/partners/learn/learning-plans/119/sitecoreai-cms-for-developers/courses/1595/renderings-and-layouts/lessons/3048:1216/renderings-and-layouts)

## Summary

Rendering parameters templates define fields that control how a component renders on a page. Unlike **data templates** that store content, these templates focus on presentation aspects such as:

- Visual styling options
- Layout configurations
- Component behavior settings
- Display preferences
  
## Key Ideas

### Benefits of rendering parameters templates

- **Empowered Editors** - Allows content editors to modify appearance without developer intervention.
- **Reusability** - Facilitates reuse of the same component with different visual configurations.
- **Consistency** - Ensures consistent styling options across similar components.

### Location of base rendering parameter templates

- **Standard rendering parameter template** - */sitecore/templates/System/Layout/Rendering Parameters/Standard Rendering Parameters*
- **Base Rendering Parameters** -*/sitecore/templates/Foundation/Headless Experience Accelerator/Presentation/Rendering Parameters/BaseRenderingParameters*

## Create custom rendering parameter template

Developers can create custom parameters templates that directly inherit from the **Base Rendering Parameters** template or inherits other built-in templates as required.

Some common rendering parameters templates:

- **IStyling** for styles support
- **IComponentVariant** for rendering variant support
- **Grid Parameters** for grid support
- **IRenderingID** for HTML identifier support
- **IDynamicPlaceholder** for dynamic placeholder keys support

## Related

- [[2026-04-22-sitecore-renderings-components]]

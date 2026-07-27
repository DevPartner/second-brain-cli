---
type: "zknotes"
date: 2026-04-22 1776868022.674
tags: [sitecore, placeholders]
---

# Sitecore Placeholders

## Source

From: [Website](https://learning.sitecore.com/partners/learn/learning-plans/119/sitecoreai-cms-for-developers/courses/1595/renderings-and-layouts/lessons/3048:1216/renderings-and-layouts)

## Summary

## Placeholders

Placeholders in SitecoreAI are designated areas within [[2026-04-22-sitecore-layouts|layouts]] or [[2026-04-22-sitecore-renderings-components|renderings]] where content authors can add [[2026-04-22-sitecore-renderings-components|components]]. They serve as containers that define where content [[2026-04-22-sitecore-renderings-components|components]] can be placed on a page, providing structure while enabling flexibility.

## Key Ideas

Here are the key terminologies:

- **Placeholder** - A named location in a layout or rendering where [[2026-04-22-sitecore-renderings-components|components]] can be added.
- **Placeholder Key** - A unique identifier for each placeholder (e.g., "main", "header", "footer").
- **Placeholder Settings** - Definition items that configure which [[2026-04-22-sitecore-renderings-components|components]] can be added to specific placeholders.
- **Allowed Controls** - [[2026-04-22-sitecore-renderings-components|components]] that are permitted to be added to a specific placeholder.

**Placeholders** are fundamental to Sitecore's compositional approach to page building, allowing:

- Modular page construction
- Content author empowerment
- Controlled flexibility
- Consistent brand experience

A rendering, such as the **Container** component, can define placeholders within its structure to accommodate other [[2026-04-22-sitecore-renderings-components|components]].

## Placeholder settings

Placeholder settings are definition (configuration) items in SitecoreAI CMS that define (configure) and control which [[2026-04-22-sitecore-renderings-components|components]] (renderings) can be added to specific placeholders. They act as the rulebook for what content authors can place where.

Placeholder (settings) are crucial to:

- Maintain design consistency.
- Limit options to prevent inappropriate component placement.
- Provide content authors with relevant component choices.

> **Note:** Placeholder settings are typically found at ***sitecore/layout/Placeholder Settings*** in Content Editor.

> **Note:** You can create placeholder setting items for your site in ***sitecore/layout/Placeholder Settings/Project/{site collection}/{site} or sitecore/layout/Placeholder Settings/{Feature}/{site}***

### Main properties of Placeholder (settings) item

- **Name**: The name of the placeholder (setting) item (usually matches the placeholder key).
- **Placeholder Key**: The key that links this setting to placeholders in layouts/renderings.
- **Allowed Controls**: The list of [[2026-04-22-sitecore-renderings-components|components]] that can be added to this placeholder.
- **Description**: Optional information about the placeholder's purpose.
  
## Configure allowed controls

Allowed controls are the specific [[2026-04-22-sitecore-renderings-components|components]] or renderings that content authors can add to a particular placeholder. They define the "menu of options" presented to content authors when they choose to add component to a placeholder.

## Allowed controls in the Main placeholder

The allowed controls define what options are available to content editors when adding [[2026-04-22-sitecore-renderings-components|components]], and each component can have its own set of allowed controls.

## Related

- [[2026-04-22-sitecore-renderings-components]]
- [[2026-04-22-sitecore-layouts|layouts]]

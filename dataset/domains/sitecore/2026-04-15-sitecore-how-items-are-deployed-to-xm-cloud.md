---
type: "zknotes"
date: 2026-04-15 1776276374.067
tags: [sitecore, xmcloud]
---

# How items are deployed to XM Cloud

## Source

From: [website](https://learning.sitecore.com/partners/learn/learning-plans/119/sitecoreai-cms-for-developers/courses/1598/sitecore-content-serialization/lessons/3056:1219/sitecore-content-serialization)

## Summary

There are two distinct methods for deploying items to XM Cloud with Sitecore Content Serialization, each catering to different requirements as detailed below.

## Key Ideas

### Deploying Items to the File System using Items as Resources (IAR)

This pertains to items intended for developer control and should not be modified within the Content Management environment, except when potential impacts have been duly considered. For configuring which items are deployed this way, you would adjust the **xmcloud.build.json** file located at the root of your solution. To configure the items that will be deployed to the file system (IAR), refer to the example below:

````json
{ "deployItems: { "modules": [ "Multisite", "Localization", ... ] }, ...}
````

In the given code example, we use **deployItems**to define which module definitions from our solution will be deployed as Items as Resources (IAR) onto the XM Cloud file system. It is important to remember that you can use wildcards, to simplify the configuration of items.

Here are some examples of items that developers should configure and deploy on the file system (similar to the Sitecore Item paths listed above):

- Modules
- Templates
- Branch Templates
- Media Library Folder
- Layouts
- Renderings
- Placeholder Settings
- Site collection
- The site root item

  - The Media Item (under site item)
  - The Data item with it’s direct children for the different data source folders
  - Dictionary item with direct children
  - The Presentation Section incl. all subitems
  - The Settings sections incl. all subitems

> **Note:** Never use this method to serialize and synchronize individual media items - only use it for the parent media item folder. Using this method for individual media items can lead to performance issues. If you need to synchronize media items, utilize the Post Deploy Action. This will efficiently store your media items in Azure Blob Storage.
>
### Deploying Items to the Database using Post Deploy Actions

This refers to items that are typically managed by a content author within the Content Management system. These items, such as the **Home**item, are beneficial for content authors to have when they start creating content.

To set up items for the Content Management database, you will need to modify the **xmcloud.build.json** file located at the root of your solution. This involves configuring the Post Deploy Actions section as follows:

````json
{ ..., "postActions": { "actions": { "scsModules": { "modules": [ ... ] } } }}
````

In the provided code, you're defining a post-deploy action known as scsModules. This action uses the same modules definition as you would use with deployActions. For clarity, you should create and configure specific modules that define the items needed for your database deployments in XM Cloud.

## Related

- [[2026-04-15-sitecore-project-configuration-sitecore-json]]
- [[2026-04-15-sitecore-content-serialization-module-json]]

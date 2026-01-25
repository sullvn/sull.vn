---
name: mdn
description: Search MDN Web Docs for web platform documentation. Use when the user asks about HTML elements, CSS properties, JavaScript APIs, Web APIs, browser APIs, DOM methods, or any web platform feature.
argument-hint: <search query>
context: fork
agent: general-purpose
allowed-tools: WebFetch, WebSearch
---

Search MDN Web Docs for: $ARGUMENTS

## Instructions

1. Construct the MDN search URL by URL-encoding the query:
   `https://developer.mozilla.org/en-US/search?q=<encoded-query>&page=1`

2. Use WebFetch to retrieve the search results page

3. From the results, identify the most relevant documentation pages

4. Fetch the top 1-3 most relevant pages to get detailed information

5. Summarize the findings, including:
   - Direct answers to the query
   - Code examples if applicable
   - Browser compatibility notes if relevant
   - Links to the source documentation

## Example

For query "fetch API":
- Search URL: `https://developer.mozilla.org/en-US/search?q=fetch+API&page=1`
- Fetch top results like `/en-US/docs/Web/API/Fetch_API`

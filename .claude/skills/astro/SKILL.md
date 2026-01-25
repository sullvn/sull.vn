---
name: astro
description: Search Astro framework documentation. Use when the user asks about Astro components, Astro routing, content collections, Astro islands, Astro integrations, or any Astro framework feature.
argument-hint: <search query>
context: fork
agent: general-purpose
allowed-tools: mcp__astro-docs__search_astro_docs, WebFetch
---

Search Astro documentation for: $ARGUMENTS

## Instructions

1. **Preferred: Use the astro-docs MCP server**

   If the `mcp__astro-docs__search_astro_docs` tool is available, use it to search the documentation directly.

2. **Fallback: Fetch from docs site**

   If the MCP server is not available, use WebFetch to search the Astro docs site:
   `https://docs.astro.build/`

   Navigate to relevant pages based on the query (e.g., `/en/guides/`, `/en/reference/`, `/en/tutorial/`).

3. Summarize the findings, including:
   - Direct answers to the query
   - Code examples if applicable
   - Links to the source documentation

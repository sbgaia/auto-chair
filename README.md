# Auto-chair 🦀 🪑

🚧 *Project Status:* auto-chair is an ongoing project under active development. 

Auto-chair is a robust, asynchronous Rust library designed to solve the Reviewer Assignment Problem (RAP) for academic conferences and journals. By treating manuscript-to-reviewer matching as a data-driven information retrieval and mathematical optimization task, auto-chair eliminates the heuristic guesswork and systemic bias of manual bidding.

Auto-chair is built with a privacy-first architecture. The entire optimization and matching engine is designed to be run entirely locally. Your proprietary submission data and sensitive reviewer matrices never leave your machine, guaranteeing complete anonymity and confidentiality for both authors and reviewers.

Powered by persistent identifiers (ORCID) and the [OpenAlex API](https://docs.openalex.org/how-to-use-the-api/api-overview) this library automatically builds reviewer profiles, filters out professional conflicts, and calculates deep semantic affinity.

## ✨ Features
- **Privacy-First Local Execution:** Runs completely on your own hardware. While the library queries external APIs for public metadata, your sensitive, unpublished submission abstracts and final assignment matrices remain completely localized and anonymous.

- **Deterministic Entity Resolution:** Ingests Program Committee (PC) members via their ORCID IDs, natively mapping them to canonical OpenAlex Author IDs to completely bypass the "author name ambiguity" problem.

- **Automated Conflict of Interest (COI) Detection:**
  - **Co-authorship Networks:** Automatically traverses the authorships arrays of recent publications to build a graph of recent collaborators, establishing hard constraints against conflicted assignments.
  - **Institutional Overlap:** Resolves raw affiliation strings to canonical Research Organization Registry (ROR) IDs, preventing assignments between researchers at the same institution.

- **Mathematical Workload Optimization:** Uses linear programming constraints to ensure no PC member is overburdened and every paper receives the required number of qualified reviews.

## 📄 License
Licensed under the MIT license. Data retrieved by this library is provided by OpenAlex and is licensed under CC0.

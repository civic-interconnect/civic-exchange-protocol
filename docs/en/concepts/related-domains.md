# Related Research Domains

This standard operates at the intersection of three major, yet often separate, academic research domains: 
- Formal Entity Resolution (CS), 
- Campaign Finance/Policy Analysis (Political Science), and
- Global Data Standards (Information Systems).

## Prior Work: Entity Resolution, Data Standards, and Civic Transparency

This standard advances the state-of-the-art by bridging the gap between three distinct bodies of work: high-fidelity entity resolution, policy-driven data harmonization, and the development of open standards for public accountability.

### 1. Entity Resolution and Data Quality Methodology
Research in Entity Resolution (ER), also known as record linkage or deduplication, has been a cornerstone of computer science and database theory.

Carnegie Mellon University (CMU) has contributed foundational work in formalizing the ER problem, such as the ENRES framework, which provides a semantic model to represent and relate various ER research models.
This work highlights the crucial need for formal reasoning and explicit assumptions regarding entities and their references, which directly informs our Category Theory approach.
Other CMU-related research has addressed Generic Entity Resolution with Data Confidences, underscoring the necessity of associating numerical confidence with merged records, a feature we implement via the Splink-generated confidence_score.

The computational methodology behind our implementation relies heavily on advancements in probabilistic record linkage, particularly the Fellegi-Sunter model, which allows for high-accuracy linking of data without labeled training sets, critical for messy public data.
The use of the open-source Splink library (pioneered outside academia and rigorously validated) represents a pragmatic, scalable implementation of these probabilistic principles.

### 2. Campaign Finance and Policy Data Standardization
A parallel track of research has focused on the application of computational methods to clean and analyze fragmented political data.

Stanford University's DIME (Database on Ideology, Money in Politics, and Elections) Project exemplifies the effort to centralize and standardize complex political contributions data.
DIME has assigned unique identifiers for millions of individual and organizational donors, candidates, and political committees across federal and state elections. This established the value proposition for a persistent, standardized identifier in the campaign finance domain.

The University of Chicago MS in Computational Analysis and Public Policy (MSCAPP) and Stanford's Computational Public Policy programs have emphasized the necessary skills, including machine learning, big data, and computational analysis required to address policy issues like public procurement and campaign finance.
These programs underscore the academic recognition that robust data infrastructure is the prerequisite for rigorous policy analysis.

While these efforts successfully resolve entities within their domain (e.g., within campaign finance), they often use internal, proprietary identifiers that do not readily link to entities in the public procurement or grant domains, creating siloed transparency.

### 3. Open Data, Interoperability, and Global Standards
The third body of work defines the principles of modern data governance and standardization.

Massachusetts Institute of Technology (MIT), through its Research Data Principles, champions the need for researchers to manage data actively and avoid acquiring or generating data that restricts future use and sharing.
This principle of "open by default" and "designed for reuse" informs the design of the our open-source standard and its reliance on the non-proprietary SNFEI.

Organizations supporting the Open Data Charter emphasize that data must be comparable and interoperable to enhance governance and accountability.
This reflects the global demand for cross-sectoral standards.
The existing Legal Entity Identifier (LEI) standard provides the template for a globally harmonized system in the financial sector.

### Novelty and Contribution of this Standard
This project is novel in three key ways:

- Compositional Rigor: We model the civic exchange system as a Category, using Category Theory to formally prove that the SNFEI acts as the Universal Property (Limit) that unifies all messy civic records, helping to guarantee the standard's structural integrity and extensibility.

- Tiered, Extensible Identity: We address sub-federal identities by explicitly creating the SNFEI (Tier 3) as an open-source bridge to the global LEI (Tier 1) and federal UEI (Tier 2).

- The Provenance Tag: We enforce a Compositional Provenance Tag that structurally records the Morphism Type (GRANT, CONTRACT_FEE) and the entity hierarchy, enabling automated tracing of the entire funding chain, which goes beyond simple entity deduplication.

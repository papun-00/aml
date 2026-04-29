# Threat Model — &lt;component name&gt;

Use STRIDE for any component. Add LINDDUN for components handling PII/PHI. Add MAESTRO + ATLAS for any AI feature. Reference: `/Volumes/hex/current-work/skills/threat-modeling.md`.

## 1. Scope

- **Component:** &lt;name + path&gt;
- **Owner:** &lt;name&gt;
- **Data classification:** public | internal | confidential | restricted
- **Trust boundaries crossed:** &lt;list&gt;
- **Out of scope:** &lt;list — keeps the model focused&gt;
- **Frameworks applied:** STRIDE | LINDDUN | MAESTRO | ATLAS

## 2. Assets &amp; flows

- **Assets:** &lt;data, secrets, capability tokens, models, etc.&gt;
- **Inbound flows:** &lt;source → component&gt;
- **Outbound flows:** &lt;component → sink&gt;
- **DFD reference:** `security/threat-model/data-flow-diagrams/&lt;name&gt;.md`

## 3. Threats — STRIDE

| ID | Category | Threat | Likelihood | Impact | Risk |
|----|----------|--------|------------|--------|------|
| T-001 | Spoofing | | | | |
| T-002 | Tampering | | | | |
| T-003 | Repudiation | | | | |
| T-004 | Information disclosure | | | | |
| T-005 | Denial of service | | | | |
| T-006 | Elevation of privilege | | | | |

## 4. Threats — LINDDUN (privacy; only if PII/PHI)

| ID | Category | Threat | Likelihood | Impact | Risk |
|----|----------|--------|------------|--------|------|
| L-001 | Linkability | | | | |
| L-002 | Identifiability | | | | |
| L-003 | Non-repudiation | | | | |
| L-004 | Detectability | | | | |
| L-005 | Disclosure of information | | | | |
| L-006 | Unawareness | | | | |
| L-007 | Non-compliance | | | | |

## 5. Threats — MAESTRO + ATLAS (AI features only)

See `/Volumes/hex/current-work/skills/threat-modeling.md` §AI for the full taxonomy. Cover at minimum:

- Prompt injection (direct + indirect)
- Training-data poisoning
- Model theft / extraction
- Output handling (hallucinated commands, unsafe deserialisation)
- Tool-use mediation (capability sprawl)
- Sensitive data exposure via context window
- Supply chain (poisoned model weights, compromised inference endpoint)

| ID | Category | Threat | Likelihood | Impact | Risk |
|----|----------|--------|------------|--------|------|
| AI-001 | Prompt injection | | | | |
| AI-002 | Output handling | | | | |
| AI-003 | Tool-use abuse | | | | |
| AI-004 | Model supply chain | | | | |

## 6. Controls (≥2 independent per high-impact threat)

| Threat ID | Control | Type | Status |
|-----------|---------|------|--------|
| | | preventive / detective / corrective | proposed / implemented / verified |

## 7. Residual risk &amp; sign-off

- **Residual risk:** &lt;summary after controls&gt;
- **Risk owner:** &lt;name&gt;
- **Reviewer:** &lt;name&gt;
- **Date:** &lt;YYYY-MM-DD&gt;
- **Re-review trigger:** &lt;next arch change | every 90 days | on incident&gt;

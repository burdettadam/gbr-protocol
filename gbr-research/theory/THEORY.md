# Theoretical Foundations

The GBR Protocol encodes concepts from literary and narrative theory. This document maps protocol constructs to their theoretical sources at a **concept level**.

> **For field-level rationale** — why each *specific attribute* exists and what scholarly tradition it derives from — see the [`architecture/`](architecture/) directory:
> - [`architecture/SCENE_CARD.md`](architecture/SCENE_CARD.md)
> - [`architecture/CHARACTER_STATE.md`](architecture/CHARACTER_STATE.md)
> - [`architecture/REGISTRY.md`](architecture/REGISTRY.md)
> - [`architecture/STORY_ARCHITECTURE.md`](architecture/STORY_ARCHITECTURE.md)

---

## Narratology

### Gérard Genette (1980)

**Narrative Discourse** provides the core temporal and voice frameworks:

| Protocol Concept | Genette Term | Description |
|------------------|--------------|-------------|
| `narrative_time.order` | Order | analepsis (flashback), prolepsis (flash-forward) |
| `narrative_time.duration_mode` | Duration | scene, summary, ellipsis, pause, stretch |
| `narrative_time.frequency` | Frequency | singulative, iterative, repetitive |
| `focalization_type` | Focalization | zero, internal (fixed/variable/multiple), external |
| `diegetic_level` | Narrative Levels | extradiegetic, intradiegetic, metadiegetic |
| `transtextuality_type` | Transtextuality | inter-, para-, meta-, hyper-, architextuality |

### Dorrit Cohn (1978)

**Transparent Minds** — consciousness representation:

| Protocol Concept | Cohn Term | Description |
|------------------|-----------|-------------|
| `consciousness_mode: psychonarration` | Psychonarration | Narrator reports thoughts |
| `consciousness_mode: quoted_monologue` | Quoted Monologue | Direct thought (tagged) |
| `consciousness_mode: narrated_monologue` | Narrated Monologue | Free indirect discourse |

### Mieke Bal (1997)

**Narratology** — focalizer/focalized distinction:

| Protocol Concept | Bal Term | Description |
|------------------|----------|-------------|
| `focalizer` | Focalizer | Who sees |
| participants viewed | Focalized Object | What is seen |
| embedded focalization | Embedded Focalization | Nested perception |

---

## Structuralism

### A.J. Greimas (1966)

**Structural Semantics** — actantial model:

| Protocol Concept | Greimas Term | Description |
|------------------|--------------|-------------|
| `actant: subject` | Subject | Who desires |
| `actant: object` | Object | What is desired |
| `actant: sender` | Sender | Who initiates quest |
| `actant: receiver` | Receiver | Who benefits |
| `actant: helper` | Helper | Who aids |
| `actant: opponent` | Opponent | Who impedes |

### Vladimir Propp (1928)

**Morphology of the Folktale** — narrative functions (foundation for beat types).

### Seymour Chatman (1978)

**Story and Discourse**:

| Protocol Concept | Chatman Term | Description |
|------------------|--------------|-------------|
| Canonical summary (fabula) | Story | What happened |
| Prose (syuzhet) | Discourse | How it's told |
| `event_significance: kernel` | Kernel | Plot-changing event |
| `event_significance: satellite` | Satellite | Elaboration/texture |

---

## Character Theory

### John Truby (2007)

**The Anatomy of Story**:

| Protocol Concept | Truby Term | Description |
|------------------|------------|-------------|
| `character.want` | Want | External goal |
| `character.need` | Need | Thematic truth |
| `character.flaw` | Flaw | What prevents growth |
| `antagonist.thematic_mirror` | Thematic Opponent | Shadow of protagonist |
| `opposition_level` | Opposition Levels | physical → thematic |

### K.M. Weiland / Lisa Cron

**Creating Character Arcs** / **Story Genius**:

| Protocol Concept | Term | Description |
|------------------|------|-------------|
| `character.ghost` | Ghost / Wound | Origin of misbelief |
| `protagonist_arc.lie_believed` | The Lie | False belief at start |
| `protagonist_arc.truth_needed` | The Truth | What must be learned |
| `drive_model: wound` | Wound-driven | Behavior from trauma |

### Joseph Campbell / Christopher Vogler

**Hero with a Thousand Faces** / **Writer's Journey**:

| Protocol Concept | Term | Description |
|------------------|------|-------------|
| `archetype` enum | Archetypes | Hero, Mentor, Shadow, etc. |
| `beat_type` enum | Hero's Journey | Threshold, Ordeal, Return, etc. |

---

## Rhetoric

### Kenneth Burke (1945/1969)

**A Grammar of Motives** / **A Rhetoric of Motives**:

| Protocol Concept | Burke Term | Description |
|------------------|------------|-------------|
| `pentad_focus` | The Pentad | Act, Scene, Agent, Agency, Purpose |
| `burke_form_type` | Form Types | progressive, repetitive, conventional, minor |

---

## Psychoanalysis

### Sigmund Freud

| Protocol Concept | Freud Term | Description |
|------------------|------------|-------------|
| `freudian_mechanism: condensation` | Condensation | Multiple meanings in one image |
| `freudian_mechanism: displacement` | Displacement | Affect shifted to safer target |
| `freudian_mechanism: uncanny` | The Uncanny | Familiar made strange |
| `freudian_mechanism: repetition_compulsion` | Repetition Compulsion | Re-enacting trauma |

### Jacques Lacan

| Protocol Concept | Lacan Term | Description |
|------------------|------------|-------------|
| `lacan_register: real` | The Real | Pre-linguistic, traumatic |
| `lacan_register: symbolic` | The Symbolic | Language, law, social order |
| `lacan_register: imaginary` | The Imaginary | Image, ego, mirror-stage |

### Julia Kristeva (1982)

**Powers of Horror**:

| Protocol Concept | Kristeva Term | Description |
|------------------|---------------|-------------|
| `abject_category` | Abjection | What must be expelled to form identity |

### Laura Mulvey (1975)

**Visual Pleasure and Narrative Cinema**:

| Protocol Concept | Mulvey Term | Description |
|------------------|-------------|-------------|
| `gaze_type` | The Gaze | Scopophilia, male gaze, spectacle |

---

## Speech Act Theory

### J.L. Austin / John Searle

| Protocol Concept | Austin/Searle Term | Description |
|------------------|-------------------|-------------|
| `speech_act_category` | Illocutionary Acts | assertive, directive, commissive, expressive, declaration |
| `illocutionary_force` | Illocutionary Force | What the utterance does |

### H.P. Grice (1975)

**Logic and Conversation**:

| Protocol Concept | Grice Term | Description |
|------------------|------------|-------------|
| `subtext.maxim_violated` | Cooperative Principle | Quantity, Quality, Relation, Manner |
| `subtext.violation_type` | Implicature | Meaning beyond what's said |

---

## Trauma Studies

### Cathy Caruth (1996)

**Unclaimed Experience**:

| Protocol Concept | Caruth Term | Description |
|------------------|-------------|-------------|
| `trauma_representation_mode: belated` | Belatedness | Trauma returns later |
| `trauma_representation_mode: fragmented` | Traumatic Memory | Non-integrated |

### Judith Herman (1992)

**Trauma and Recovery**:

| Protocol Concept | Herman Term | Description |
|------------------|-------------|-------------|
| Three-stage recovery | Safety → Remembrance → Reconnection | |

### Bessel van der Kolk (2014)

**The Body Keeps the Score**:

| Protocol Concept | van der Kolk Term | Description |
|------------------|-------------------|-------------|
| `somatic_trauma_response` | Somatic Memory | Body-first response |

### Dominick LaCapra (2001)

| Protocol Concept | LaCapra Term | Description |
|------------------|--------------|-------------|
| `trauma_representation_mode: acting_out` | Acting Out | Compulsive repetition |
| `trauma_representation_mode: working_through` | Working Through | Conscious integration |

---

## Craft

### John Gardner (1983)

**The Art of Fiction**:

| Protocol Concept | Gardner Term | Description |
|------------------|--------------|-------------|
| `psychic_distance` (1-5) | Psychic Distance | Intimacy of narration |

### Wayne Booth (1961)

**The Rhetoric of Fiction**:

| Protocol Concept | Booth Term | Description |
|------------------|------------|-------------|
| `narrator_reliability_type` | Unreliable Narrator | Factual, interpretive, evaluative |
| `irony_type` | Stable/Unstable Irony | Types of ironic distance |

### Shlomith Rimmon-Kenan (1983)

**Narrative Fiction**:

Synthesis of Genette and Bal; clarifies focalization/narration distinction.

### Gerald Prince (1971/1987)

**Narratology** / Dictionary entry:

| Protocol Concept | Prince Term | Description |
|------------------|-------------|-------------|
| `narratee_type` | Narratee | Fictional listener constructed by narration |

---

## Affect Studies

### Sara Ahmed (2004/2006)

| Protocol Concept | Ahmed Term | Description |
|------------------|------------|-------------|
| `textual_affect_type: affective_stickiness` | Sticky Emotions | How emotions attach to objects |

### Lauren Berlant (2011)

**Cruel Optimism**:

| Protocol Concept | Berlant Term | Description |
|------------------|--------------|-------------|
| `textual_affect_type: cruel_optimism` | Cruel Optimism | Attachment to damaging desires |
| `textual_affect_type: impasse` | Impasse | Suspension of agency |

---

## Critical Theory

### Edward Said (1978)
`postcolonial_mode: orientalism` — Orientalism

### Gayatri Spivak (1988)
`postcolonial_mode: subaltern_silence` — Can the Subaltern Speak?

### Homi Bhabha (1994)
`postcolonial_mode: third_space`, `hybridity`, `mimicry`

### Eve Kosofsky Sedgwick (1990)
`queer_mode: closet_epistemology` — Epistemology of the Closet

### Judith Butler (1990/1993)
`performativity_mode` — Gender Trouble / Bodies That Matter

### Gerald Vizenor (1999)
`survivance_mode` — Manifest Manners

---

## Spatial Theory

### Yuri Lotman (1977)

**The Structure of the Artistic Text**:

| Protocol Concept | Lotman Term | Description |
|------------------|-------------|-------------|
| `spatial_structure` | Semantic Spaces | enclosed/open, threshold |
| `boundary_type` | Boundary | What separates zones |
| `semantic_zone` | Semantic Zones | safety/danger, known/unknown |

### Gaston Bachelard (1958)

**The Poetics of Space**:

| Protocol Concept | Bachelard Term | Description |
|------------------|----------------|-------------|
| `intimate_space_type` | Topoanalysis | attic, cellar, nest, shell, corner |

### Michel de Certeau (1984)

**The Practice of Everyday Life**:

Strategies vs. tactics, walking as narrative.

### Yi-Fu Tuan (1977)

**Space and Place**:

Space vs. place, topophilia/topophobia.

---

## References (Selected)

- Austin, J.L. *How to Do Things with Words*, 1962
- Bal, Mieke. *Narratology*, 1985/1997
- Booth, Wayne. *The Rhetoric of Fiction*, 1961
- Burke, Kenneth. *A Grammar of Motives*, 1945
- Caruth, Cathy. *Unclaimed Experience*, 1996
- Chatman, Seymour. *Story and Discourse*, 1978
- Cohn, Dorrit. *Transparent Minds*, 1978
- Gardner, John. *The Art of Fiction*, 1983
- Genette, Gérard. *Narrative Discourse*, 1972/1980
- Greimas, A.J. *Structural Semantics*, 1966
- Grice, H.P. "Logic and Conversation", 1975
- Herman, Judith. *Trauma and Recovery*, 1992
- Propp, Vladimir. *Morphology of the Folktale*, 1928
- Rimmon-Kenan, Shlomith. *Narrative Fiction*, 1983
- Truby, John. *The Anatomy of Story*, 2007
- Weiland, K.M. *Creating Character Arcs*, 2016

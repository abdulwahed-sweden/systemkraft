# Services

**Abdulwahed Mansour — Rust systems, security & admin platforms.**
Independent engineering for Swedish companies.

I build software that is *correct by construction*. My work centres on the
Rust language: memory-safe, strongly-typed systems where the compiler rules
out whole classes of failure before they ship. Where a system handles money,
authority, or physical hardware, that guarantee is not a luxury — it is the
point.

Below are the four lines of work I offer, each with the problems it solves and
the work behind it.

---

## 1. Rust Admin & Internal Platforms

**For companies that run on internal tools and have outgrown spreadsheets and
disconnected apps.**

I build production back-office platforms in Rust where the hard parts —
authentication, sessions, password recovery, and a complete audit trail — are
**designed as one system**, not assembled from separate parts and bolted on
afterward. CRUD is the easy layer on top.

This is the discipline behind my own framework,
[`rustio-admin`](https://github.com/abdulwahed-sweden/rustio-admin) (the "Django
Admin for Rust") and its sibling RustIO — security-first administrative
frameworks I author and maintain.

**Problems I solve**

- Slow, error-prone internal tools that staff don't trust.
- A sprawl of separate admin apps with no shared access control.
- No answer to "who changed this, and when?" — every authority change here is
  recorded with a correlation id.
- Heavy front-end build pipelines for tools that don't need them: this ships as
  a **single binary, no build step**.

**Technology:** Rust, `rustio-admin`, PostgreSQL, role-based access control,
audit-by-default.
**Engagement:** fixed-scope build, or an ongoing retainer.

---

## 2. Smart-Contract & Protocol Security Audits

**For teams about to put capital and reputation on the line.**

I perform independent, doctrine-driven security reviews of smart contracts and
on-chain protocols: threat modelling, line-by-line analysis, and a written
report with each finding ranked by impact and paired with a concrete remedy.

This builds on an extensive portfolio of protocol security work across major
DeFi and blockchain systems — including reviews and research touching Fluid,
ENS, Moonwell, Superfluid, LayerZero, the XRP Ledger, Folks, Morpho, and
Perennial.

**Problems I solve**

- Vulnerabilities that surface only after launch, when funds are already at
  risk.
- The need for an *independent* second set of eyes before a mainnet release.
- Findings teams can actually act on — and share with their community.

**Technology:** Solidity and protocol-level review, DeFi mechanics, threat
modelling.
**Engagement:** fixed-scope audit with a written report.

---

## 3. Systems & Robotics Engineering in Rust

**For teams building hardware that has to work reliably in the real world.**

I design reliable control software with **one trait surface shared between
simulation and hardware** — so what you validate in the simulator is the same
code that runs on the machine. The maths stays pure and the motion planner is
explicit and testable.

This is the architecture of my own `robotics-platform`: a Rust workspace with
analytic kinematics (via `nalgebra`), trajectory and motion planning, and one
backend boundary spanning an in-process simulator and Raspberry Pi hardware.

**Problems I solve**

- Two divergent codebases for simulation and hardware that drift apart and fail
  in the field.
- Control logic that can't be tested without the physical rig.
- Safety and real-time concerns treated as afterthoughts.

**Technology:** Rust, `nalgebra`, kinematics, motion planning, embedded
(rppal), simulation.
**Engagement:** hourly or project-based.

---

## 4. Secure-by-Construction Web Applications

**For companies that want security built in, not bolted on.**

I build web applications where login, password recovery, session lifecycle, and
audit logging are **first-class, reviewable concerns from day one** — governed
by written doctrine rather than improvised late in the project.

**Problems I solve**

- Security features added near the deadline, with the gaps that always leaves.
- Applications that can't tell you what happened after an incident.
- Auth, recovery, and audit wired together ad hoc from unrelated parts.

**Technology:** Rust and Django, audit-by-default, secure session and recovery
design.
**Engagement:** fixed-scope build.

---

## How I work

- **Correct by construction.** Rust's type system and strict-by-default design
  eliminate whole classes of bug before they ship.
- **Audit-by-default.** Every authority change is recorded; you can always
  reconstruct who did what.
- **Operational simplicity.** One database, one stylesheet, one binary — less
  surface to secure and to operate.
- **Doctrine before code.** Security-sensitive behaviour is written down and
  reviewed against, not invented under deadline.

## Get in touch

Abdulwahed Mansour · Sweden
Email: **abdulwahed.sweden@gmail.com** · GitHub: **github.com/abdulwahed-sweden**

I reply personally to every serious inquiry from a Swedish company.

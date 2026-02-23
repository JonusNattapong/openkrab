---
name: KrabHub
description: Use the KrabHub CLI to search, install, update, and publish agent skills from KrabHub.com. Use when you need to fetch new skills on the fly, sync installed skills to latest or a specific version, or publish new/updated skill folders with the npm-installed KrabHub CLI.
metadata:
  {
    "OpenKrab":
      {
        "requires": { "bins": ["KrabHub"] },
        "install":
          [
            {
              "id": "node",
              "kind": "node",
              "package": "KrabHub",
              "bins": ["KrabHub"],
              "label": "Install KrabHub CLI (npm)",
            },
          ],
      },
  }
---

# KrabHub CLI

Install

```bash
npm i -g KrabHub
```

Auth (publish)

```bash
KrabHub login
KrabHub whoami
```

Search

```bash
KrabHub search "postgres backups"
```

Install

```bash
KrabHub install my-skill
KrabHub install my-skill --version 1.2.3
```

Update (hash-based match + upgrade)

```bash
KrabHub update my-skill
KrabHub update my-skill --version 1.2.3
KrabHub update --all
KrabHub update my-skill --force
KrabHub update --all --no-input --force
```

List

```bash
KrabHub list
```

Publish

```bash
KrabHub publish ./my-skill --slug my-skill --name "My Skill" --version 1.2.0 --changelog "Fixes + docs"
```

Notes

- Default registry: https://KrabHub.com (override with KrabHub_REGISTRY or --registry)
- Default workdir: cwd (falls back to OpenKrab workspace); install dir: ./skills (override with --workdir / --dir / KrabHub_WORKDIR)
- Update command hashes local files, resolves matching version, and upgrades to latest unless --version is set


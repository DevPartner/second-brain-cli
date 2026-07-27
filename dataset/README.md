# Interview preparation guide

## Folder Structure

```plaintext
├── README.md
└── domains/
    ├── angular/
    ├── architecture/
    ├── aspnet/
    ├── azure/
    ├── dotnet/
    ├── javascript/
    ├── react/
    ├── sitecore/
    ├── sql/
└── mdanki/
```

## Example workflow

1. Write notes in VS Code using a consistent Q/A Markdown snippet in MDAnki format.

    ```markdown
    ## What's the Markdown?

    Markdown is a lightweight markup language with plain-text-formatting syntax.
    Its design allows it to be converted to many output formats,
    but the original tool by the same name only supports HTML.

    ## Who created Markdown?

    John Gruber created the Markdown language in 2004 in collaboration with
    Aaron Swartz on the syntax.

    ```

    If you want to have multiple lines on the card's front side - use `%` symbol for splitting front and back sides:

    ```markdown
    ## YAGNI

    Describe this acronym and why it's so important.

    %

    "You aren't gonna need it" (YAGNI) is a principle of extreme programming
    (XP) that states a programmer should not add functionality until deemed
    necessary.

    ```

2. Use a simple script or mdanki to generate Anki .apkg file.
    **Install**

    ```bash
    npm install -g mdanki
    ```

    **Usage**

    Convert a single markdown file:

    ```bash
    mdanki library.md anki.apkg
    ```

3. Import just generated `.apkg` file to Anki ("File" - "Import").

### Implementation Strategy

## 1: Quick Start (Week 1)

1. **Download [Anki](https://apps.ankiweb.net/)** and import existing .NET, etc.  decks
2. **Clone GitHub repos** for comprehensive question banks
3. **Set up daily review schedule** (20-30 questions/day)

## Phase 2: Customization (Week 2-3)

1. **Create custom Anki cards** for areas
2. **Organize questions by domain** (dotnet, aspnet, sitecore, etc.)
├── domains/
│   ├── dotnet/
│   ├── aspnet/
│   ├── sitecore/
│   ├── javascript/
│   ├── react/
│   ├── angular/
│   ├── sql/
│   └── architecture/

### Weekly Workflow

Rotate domains each day to avoid repetition and cover breadth.

- Monday: .NET Core
- Tuesday: ASP.NET / Web APIs
- Wednesday: Sitecore
- Thursday: Frontend (JS / React / Angular)
- Friday: SQL & Database design
- Weekend: Architecture & Process

- At the end of the week export updated decks to Anki (.apkg) using a small script or mdanki. Add basic Anki stats to `dashboard.md`.

### Daily Workflow (30–45 minutes)

1. Morning (5–10 minutes): do 10-20 Anki cards with coffee.
2. Practice (20–30 minutes): mock answers, record gaps, convert gaps to cards/zknotes.
3. Evening (10–15 minutes): Anki/Foam spaced repetition review.
4. Weekly: update custom flashcard decks from recent mock interviews.

### Tips

- Keep notes flashcard-friendly: question → answer, clear context, single concept per card.
- Automate weekly export with mdanki or a tiny script that converts MD cards to Anki .apkg.
- Use .foam\templates\new-question-card.md for a new cards

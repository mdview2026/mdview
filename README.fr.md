<p align="center">
  <img src="static/logo_big.png" width="96" alt="logo mdview">
</p>

<h1 align="center">mdview</h1>

<p align="center">
  <strong>Markdown, sans le superflu.</strong> Un visualiseur de 2,0 Mo, pas un éditeur de 200 Mo de plus — double-cliquez sur n'importe quel fichier .md pour le lire comme une page web, appuyez sur F2 pour éditer sur place.<br>
  Windows · macOS · Android &nbsp;|&nbsp; Construit avec Rust + WebView2, sans Electron
</p>

<p align="center">
  <a href="https://www.mdview.top/">Site web</a> ·
  <a href="https://github.com/mdview2026/mdview/releases/latest">Télécharger</a> ·
  <a href="https://www.mdview.top/changelog.html">Nouveautés</a> ·
  <a href="https://www.mdview.top/blog/">Blog</a> ·
  <a href="docs/facts.md">Docs</a>
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.es.md">Español</a> ·
  <a href="README.fr.md">Français</a>
</p>

---

La plupart du temps, quand vous ouvrez un fichier `.md`, vous voulez juste le **lire** — un README, des notes, une réponse générée par une IA. Démarrer un éditeur complet pour ça, c'est utiliser un marteau-pilon pour écraser une mouche. mdview l'ouvre instantanément comme une page web, et quand vous avez vraiment besoin d'éditer, `F2` vous offre du WYSIWYG façon Obsidian dans la même fenêtre — sans basculer vers une autre application.

- **Application minuscule.** Installateur de 2,0 Mo, ouverture en moins d'une seconde, zéro configuration
- **Zéro encombrement.** Lecture pleine page et épurée — pas de barres d'outils, pas de panneaux que vous n'avez pas demandés
- **Édition sur place.** `F2` WYSIWYG façon Obsidian, `F3` double colonne avec aperçu en direct
- **100 % local** — sans compte, sans publicité, vos fichiers ne quittent jamais votre ordinateur

<p align="center">
  <img src="web_en/img/shots125/gallery-01-hero.png" alt="mdview affichant un fichier Markdown comme une page web — panneau du plan à gauche, titres, listes, tableaux et un bloc de code coloré" width="1080">
</p>

## Téléchargement

| Plateforme | Community (gratuit à vie) | Pro (6,99 $ à vie) |
|---|---|---|
| Windows 10/11 | [mdview-setup-en-latest.exe](https://github.com/mdview2026/mdview/releases/latest/download/mdview-setup-en-latest.exe) | [mdview-setup-pro-en-latest.exe](https://github.com/mdview2026/mdview/releases/latest/download/mdview-setup-pro-en-latest.exe) |
| macOS 11+ | [mdview-macos-en-latest.dmg](https://github.com/mdview2026/mdview/releases/latest/download/mdview-macos-en-latest.dmg) | [mdview-macos-pro-en-latest.dmg](https://github.com/mdview2026/mdview/releases/latest/download/mdview-macos-pro-en-latest.dmg) |
| Android 7.0+ | [mdview.apk](https://www.mdview.top/download/mdview.apk) | — |

Les deux éditions sont le même produit et s'écrasent mutuellement à l'installation — changez quand vous voulez. Android n'existe qu'en édition Community.

> **Alerte SmartScreen ?** Windows signale les applications sans certificat de signature de code coûteux. Cliquez sur **Informations complémentaires → Exécuter quand même**. mdview fonctionne entièrement en local et n'envoie rien.

## Pourquoi mdview

Vous double-cliquez sur un `.md` pour le lire — pas pour démarrer un IDE. Voyez-le comme le **Quick Look du Markdown** :

| | mdview | Typora / VS Code | Extension de navigateur |
|---|---|---|---|
| Taille d'installation | **2,0 Mo** | 100–400 Mo | — (mais nécessite un navigateur) |
| Double-clic sur un .md | **Instantané** | 3–5 s de démarrage de l'éditeur | Glisser-déposer ou copier-coller à chaque fois |
| Interface pensée pour la lecture | **Zéro fioriture** | Barres d'outils, panneaux, onglets | L'interface du navigateur autour |
| Conçu pour | **Lire** | Écrire du code / de la doc | Aperçu occasionnel |

Ils se complètent — gardez votre éditeur pour écrire, et utilisez mdview pour les 90 % du temps où vous ne faites que lire.

## Fonctionnalités

### Aperçu

- **Double-clic pour ouvrir** — l'installateur associe automatiquement les fichiers `.md` ; également `Ctrl+O`, glisser-déposer et le menu contextuel de l'Explorateur
- **Rafraîchissement en direct** — enregistrez dans n'importe quel éditeur et la page se re-renderise instantanément (push SSE), en conservant votre position de défilement
- **Panneau de plan** — sommaire flottant avec saut au clic et synchronisation du défilement, plus les onglets **Historique** (groupé par jour) et **Favoris**
- **Mémoire de position de lecture** — rouvre là où vous vous étiez arrêté ; une seule fenêtre par fichier
- **9 thèmes de couleur + mode clair/sombre** — Glacier, Forest, Sunset, Typewriter… un clic droit pour changer
- **Contrôles typographiques** — choisissez n'importe quelle police installée ; `Ctrl+molette` pour la taille, `Alt+molette` pour la largeur de colonne (reflow réel, pas un zoom bitmap)
- **Rendu complet** — tableaux alignés, notes de bas de page avec liens de retour, images locales, blocs de code colorés (copier / retour à la ligne / numéros de ligne)
- **Export PDF**, option de saut de ligne simple, repli d'encodage GBK/GB18030, vérification automatique des mises à jour

<table>
  <tr>
    <td><img src="web_en/img/shots125/gallery-02a-outline.png" alt="panneau de plan mdview mettant en surbrillance le titre actuel pendant le défilement"></td>
    <td><img src="web_en/img/shots125/gallery-02b-history.png" alt="panneau d'historique mdview regroupant par jour les fichiers Markdown récemment ouverts"></td>
  </tr>
  <tr>
    <td align="center"><sub>Le plan suit votre défilement</sub></td>
    <td align="center"><sub>Historique et favoris, groupés par jour</sub></td>
  </tr>
</table>

**9 thèmes de couleur + mode sombre**, un clic droit pour changer :

<table>
  <tr>
    <td><img src="web_en/img/shots125/gallery-03-themes-default.png" alt="thème clair par défaut de mdview"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-dark.png" alt="thème mode sombre de mdview"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-glacier.png" alt="thème Glacier de mdview, tons bleus froids"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-typewriter.png" alt="thème Typewriter de mdview, style machine à écrire monospace"></td>
  </tr>
  <tr>
    <td align="center"><sub>Default</sub></td>
    <td align="center"><sub>Dark</sub></td>
    <td align="center"><sub>Glacier</sub></td>
    <td align="center"><sub>Typewriter</sub></td>
  </tr>
  <tr>
    <td><img src="web_en/img/shots125/gallery-03-themes-sunset.png" alt="thème Sunset de mdview, tons ambre chauds"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-forest.png" alt="thème Forest de mdview, tons verts apaisants"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-editorial.png" alt="thème Editorial de mdview, style magazine avec serif"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-violet.png" alt="thème Violet de mdview, tons violets"></td>
  </tr>
  <tr>
    <td align="center"><sub>Sunset</sub></td>
    <td align="center"><sub>Forest</sub></td>
    <td align="center"><sub>Editorial</sub></td>
    <td align="center"><sub>Violet</sub></td>
  </tr>
  <tr>
    <td><img src="web_en/img/shots125/gallery-03-themes-lychee.png" alt="thème Lychee de mdview, tons rouges doux"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-mint.png" alt="thème Mint de mdview, tons menthe frais"></td>
    <td><img src="web_en/img/shots125/gallery-03-themes-neon.png" alt="thème Neon de mdview, contraste vif éclatant"></td>
    <td></td>
  </tr>
  <tr>
    <td align="center"><sub>Lychee</sub></td>
    <td align="center"><sub>Mint</sub></td>
    <td align="center"><sub>Neon</sub></td>
    <td></td>
  </tr>
</table>

### Édition

| Touche | Mode | Édition |
|---|---|---|
| `F2` | **Édition en direct** — WYSIWYG façon Obsidian dans l'aperçu : mise en forme, liens, tableaux (opérations lignes/colonnes), encadrés | Pro |
| `F3` | **Édition double colonne** — source à gauche, aperçu en direct à droite, défilement synchronisé par section ; `Tab` développe les extraits ; coller une image/fichier l'enregistre automatiquement dans `.__assets/` et insère le Markdown | Pro |
| `F4` / `Ctrl+E` | **Édition externe** — ouvre VS Code / Sublime / Notepad++ / le Bloc-notes (détection auto) à la ligne exacte du texte sélectionné | Gratuit |
| `Esc` / `Ctrl+W` | Fermer la fenêtre | Gratuit |

<p align="center">
  <img src="web_en/img/shots125/gallery-04-editing.png" alt="édition double colonne F3 de mdview : source Markdown à gauche, aperçu rendu en direct à droite" width="1080">
</p>

### Rendu Pro

- **Diagrammes Mermaid** (organigrammes / séquence / Gantt) avec visionneuse plein écran au double-clic — zoom molette, panoramique au glisser
- **Formules mathématiques KaTeX** — LaTeX en ligne `$...$` et en bloc `$$...$$`
- Les moteurs de rendu se téléchargent une seule fois à la première utilisation, puis fonctionnent entièrement hors ligne

<table>
  <tr>
    <td><img src="web_en/img/shots125/gallery-05a-mermaid.png" alt="organigramme Mermaid rendu par mdview Pro dans un document Markdown"></td>
    <td><img src="web_en/img/shots125/gallery-05b-katex.png" alt="formules mathématiques KaTeX rendues par mdview Pro, équation BM25 en LaTeX"></td>
  </tr>
  <tr>
    <td align="center"><sub>Diagrammes Mermaid</sub></td>
    <td align="center"><sub>Maths KaTeX</sub></td>
  </tr>
</table>

## Tarif

| | Community | Pro |
|---|---|---|
| Prix | **Gratuit à vie** (sponsoring optionnel) | **6,99 $** déblocage unique à vie (Gumroad : carte / PayPal) |
| Fonctions de lecture principales | ✅ tout | ✅ tout |
| Mermaid + KaTeX | — | ✅ |
| Édition en direct (F2) + double colonne (F3) | — | ✅ |
| Essai | — | 100 ouvertures gratuites, puis une invite « Peut-être plus tard » escamotable — **jamais de verrouillage forcé** |

## Démarrage rapide

1. Téléchargez et lancez l'installateur (ou le `mdview.exe` portable)
2. Il associe automatiquement les fichiers `.md` (dissociez quand vous voulez : clic droit → Paramètres, ou `mdview --unbind`)
3. Double-cliquez sur n'importe quel fichier Markdown et lisez

## Ligne de commande

```bash
mdview                     # exécuter une fois : associe automatiquement les fichiers .md
mdview <fichier.md>        # prévisualiser un fichier précis
mdview --install           # ajouter « Ouvrir avec mdview » au menu contextuel de l'Explorateur
mdview --uninstall         # retirer le menu contextuel
mdview --settings          # fenêtre de paramètres (association, fichiers récents, éditeur)
mdview --unbind            # retirer l'association par défaut des .md
mdview --help              # aide
```

## Variables d'environnement

| Variable | Description |
|---|---|
| `PORT` | Port du serveur HTTP (par défaut : port libre aléatoire, repli 3456) |
| `MD_HTML=1` | Produire aussi un `.html` rendu à côté du fichier `.md` |
| `MD_HTML_OUTPUT=<chemin>` | Écrire le `.html` à un emplacement précis |
| `MD_EDITOR` | Commande d'éditeur pour `F4` (ex. `code --goto "{file}:{line}"`) ; détection auto si non définie |

## FAQ

<details>
<summary><strong>Comment ouvrir des fichiers .md sur Windows ?</strong></summary>

Installez mdview une fois et il associe les fichiers `.md` pour vous. Ensuite, double-cliquez sur n'importe quel fichier Markdown : il s'ouvre rendu comme une page web — sans éditeur, sans extension de navigateur, sans configuration.
</details>

<details>
<summary><strong>mdview est-il gratuit ?</strong></summary>

Oui. L'édition Community est gratuite à vie avec l'expérience de lecture complète. L'édition Pro ajoute les diagrammes Mermaid, les maths KaTeX et deux modes d'édition (F2 en direct, F3 double colonne) pour 6,99 $ une seule fois à vie — avec d'abord 100 ouvertures d'essai gratuites, puis une simple invite « Peut-être plus tard » escamotable. Jamais de verrouillage forcé.
</details>

<details>
<summary><strong>En quoi diffère-t-il de Typora ou VS Code ?</strong></summary>

Typora et VS Code sont des éditeurs — puissants, mais lourds quand vous voulez seulement lire. mdview est un visualiseur dédié : un installateur de 2,0 Mo qui ouvre en moins d'une seconde et se ferme avec `Esc`. Voyez-le comme le Quick Look du Markdown. Ils se complètent.
</details>

<details>
<summary><strong>mdview envoie-t-il mes fichiers ?</strong></summary>

Non. mdview fonctionne 100 % en local. Il démarre seulement un serveur temporaire sur `localhost` pour rendre la page — votre Markdown ne quitte jamais votre ordinateur. Sans compte, sans publicité, sans traçage.
</details>

<details>
<summary><strong>Quelle est la configuration requise ?</strong></summary>

Windows 10/11 avec le runtime WebView2 (préinstallé sur Windows 11 et la plupart des PC Windows 10) · macOS 11 Big Sur ou ultérieur · Android 7.0 ou ultérieur.
</details>

## Pile technique

- **Langage** : Rust (édition 2021)
- **GUI** : wry (WebView2) + tao
- **HTTP** : axum + tokio (localhost uniquement)
- **Markdown** : md4c embarqué
- **Surveillance de fichiers** : notify + rechargement en direct SSE
- **Styles** : Tailwind CSS

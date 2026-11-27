# ❄ Blizzard

**Un tuteur d'étude qui tourne sur ton ordinateur.** Pose tes questions, explique avec tes mots, et Blizzard suit ta compréhension de chaque sujet.

Blizzard fonctionne entièrement en local : tes conversations et tes fichiers restent sur ton ordinateur. Il n'y a ni compte à créer ni abonnement.

Version actuelle : **1.0.1**

---

## Fonctionnalités

- **Chat avec une IA locale.** Le modèle LFM3-Omni-2B tourne sur ton processeur, sans envoi de données sur internet.
- **Réponses en direct.** Le texte apparaît au fur et à mesure qu'il est écrit.
- **Raisonnement visible.** L'IA présente ses étapes sous « Voir son raisonnement », repliées par défaut.
- **Suivi de compréhension.** Blizzard mesure ta compréhension d'après ce que tu expliques correctement. Les explications du tuteur ne comptent pas. Le cercle et les barres se mettent à jour en douceur.
- **Dossiers de chats.** Range tes chats par matière : crée un dossier, donne-lui un nom, choisis sa couleur.
- **Fichiers de cours.** Ajoute des fichiers `.txt`, `.md`, `.csv`, `.json` ou du code à un chat. L'IA s'appuie dessus pour répondre.
- **Recherche web (optionnelle).** Active-la pour une question et Blizzard cherche sur internet avant de répondre.
- **Léger pour ton ordinateur.** Le moteur tourne à faible priorité, pour que tes autres applications restent fluides.

---

## Télécharger et installer

Va sur la page **[Releases](../../releases)** et télécharge le fichier de ton système :

| Système | Fichier à télécharger | Comment l'ouvrir |
|---|---|---|
| Windows | `Blizzard_1.0.1_x64-setup.exe` ou `.msi` | Double-clique, puis suis l'installeur. |
| macOS | `Blizzard_1.0.1_aarch64.dmg` (Apple Silicon) ou `_x64.dmg` (Intel) | Ouvre le fichier et glisse Blizzard dans Applications. |
| Linux | `Blizzard_1.0.1_amd64.AppImage` ou `.deb` | AppImage : rends-le exécutable, puis double-clique. `.deb` : installe-le avec ton gestionnaire de paquets. |

**Avertissement de sécurité.** Blizzard n'est pas encore signé, donc ton système peut afficher un avertissement la première fois :

- **Windows :** clique sur **Informations complémentaires**, puis **Exécuter quand même**.
- **macOS :** fais un clic droit sur Blizzard, choisis **Ouvrir**, puis **Ouvrir** à nouveau. Si macOS bloque encore, va dans Réglages Système, Confidentialité et sécurité, puis clique sur **Ouvrir quand même**.

**Configuration minimale :** 8 Go de RAM (16 Go conseillés), environ 2 Go d'espace disque libre, et une connexion internet pour le premier lancement.

**Premier lancement.** Blizzard télécharge le moteur d'IA et le modèle, environ 1 Go. Ça prend quelques minutes, une seule fois. Le message en bas de la fenêtre indique l'étape. Attends **Prêt** avant de poser ta première question.

---

## Lancer Blizzard depuis le code source

Cette méthode sert à tester ou à développer. Elle demande Python 3 et Rust. Dans tous les cas, commence par récupérer le projet :

```bash
git clone https://github.com/Swiss4ppD3v/Blizzard.git
cd Blizzard
```

Puis lance le script de démarrage. Il vérifie ce qui manque, compile Blizzard une fois, puis l'ouvre :

```bash
python3 launch.py          # compile une fois, puis ouvre Blizzard
python3 launch.py --dev    # mode développement, affiche les erreurs
python3 launch.py --build  # recompile après une modification
```

### Windows

1. **Installe Python 3** depuis python.org. Coche « Add python.exe to PATH ».
2. **Installe Rust** : télécharge `rustup-init.exe` sur rustup.rs et suis les étapes par défaut.
3. **Installe les outils de compilation C++** : dans Visual Studio Installer, coche « Développement desktop en C++ », ou installe les « Build Tools for Visual Studio ».
4. Ouvre **PowerShell** dans le dossier du projet, puis lance :
   ```powershell
   py launch.py --dev
   ```
   Si `py` n'est pas reconnu, utilise `python launch.py --dev`.

WebView2 est déjà présent sur Windows 10 et 11 récents. Si la fenêtre ne s'ouvre pas, installe-le depuis le site de Microsoft.

### macOS

1. **Installe les outils Xcode** : ouvre le Terminal et lance `xcode-select --install`.
2. **Installe Rust** :
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Ferme et rouvre le Terminal ensuite.
3. Python 3 est généralement déjà présent. Sinon, installe-le depuis python.org.
4. Dans le dossier du projet :
   ```bash
   python3 launch.py --dev
   ```

### Linux

1. **Installe les bibliothèques système.**
   - Debian ou Ubuntu :
     ```bash
     sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libssl-dev
     ```
   - Fedora :
     ```bash
     sudo dnf install webkit2gtk4.1-devel libappindicator-gtk3-devel librsvg2-devel patchelf openssl-devel
     ```
2. **Installe Rust :**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Ajoute ensuite Rust à ton terminal. Avec fish : `fish_add_path ~/.cargo/bin`. Avec bash ou zsh, ferme et rouvre le terminal.
3. Dans le dossier du projet :
   ```bash
   python3 launch.py --dev
   ```

Au premier lancement, la compilation peut durer plusieurs minutes. Les lancements suivants sont beaucoup plus rapides.

---

## Recherche web (optionnelle)

Pour activer la recherche web, il faut une clé API gratuite de Tavily :

1. Crée un compte sur **tavily.com** (sans carte bancaire). Tu obtiens 1 000 recherches par mois.
2. Copie ta clé API depuis le tableau de bord.
3. Dans Blizzard, ouvre **Réglages**, colle la clé dans **Recherche web (Tavily)**, puis clique sur **Enregistrer**.
4. Active **Recherche web** dans le chat pour les questions qui en ont besoin.

Seul le texte de ta question part sur internet, et uniquement quand tu actives la recherche. Ta clé reste sur ton ordinateur.

---

## Confidentialité

- Tes chats, tes fichiers, tes réglages et ta clé sont stockés uniquement dans le dossier de données de Blizzard sur ton ordinateur.
- Le modèle d'IA tourne sur ton ordinateur. Blizzard n'envoie aucune question à un serveur.
- La recherche web est le seul envoi vers internet, et seulement quand tu l'actives.

Dossier de données :

- Windows : `%APPDATA%\io.github.swiss4ppd3v.blizzard`
- macOS : `~/Library/Application Support/io.github.swiss4ppd3v.blizzard`
- Linux : `~/.local/share/io.github.swiss4ppd3v.blizzard`

Pour tout réinitialiser, ferme Blizzard puis supprime ce dossier.

---

## Dépannage

**Le message dit « Le modèle ne démarre pas à temps ».**
Ferme Blizzard, attends quelques secondes, puis rouvre-le. Le premier téléchargement peut être long sur une connexion lente.

**Les réponses sont lentes.**
Ferme les programmes lourds comme les lecteurs vidéo et les onglets inutiles. Blizzard utilise environ deux cœurs du processeur pendant qu'il écrit une réponse.

**La recherche web dit qu'il faut une clé.**
Ouvre Réglages et colle ta clé Tavily, comme décrit plus haut.

**La compilation échoue.**
Vérifie que Rust et les bibliothèques système de ton système sont installés. Copie les dernières lignes d'erreur et ouvre une issue sur ce dépôt.

---

## Développement

L'interface est dans `src/index.html`. Le backend Rust, qui gère le moteur d'IA, les réponses et la recherche, est dans `src-tauri/src/main.rs`. Le projet utilise [Tauri 2](https://tauri.app).

### Ajouter un modèle

Les modèles sont listés dans la constante `MODELS` au début de `src/index.html`. Chaque entrée contient un nom affiché et un dépôt Hugging Face, par exemple `LiquidAI/LFM3-Omni-2B-GGUF`.

---

## Crédits

- **[LFM3-Omni-2B](https://huggingface.co/LiquidAI/LFM3-Omni-2B-GGUF)** par Liquid AI. Le modèle est téléchargé depuis Hugging Face et reste sous sa propre licence, LFM 1.0. Blizzard ne contient pas le fichier du modèle.
- **[llama.cpp](https://github.com/ggml-org/llama.cpp)**, le moteur qui fait tourner le modèle (licence MIT).
- **[Tauri](https://tauri.app)**, le framework qui fait de Blizzard une application de bureau (licence MIT ou Apache 2.0).
- **[Tavily](https://tavily.com)**, utilisé pour la recherche web optionnelle.

---

## Licence

Blizzard est distribué sous licence [MIT](LICENSE). Tu peux l'utiliser, le copier, le modifier et le partager, y compris à des fins commerciales, à condition de conserver la notice de copyright.

# For Code Contributors:

1. Fork the repository.
2. Clone the forked repository.
3. Create a new branch.
4. Make & commit your changes.
5. Open a pull request.

# For Translators:

The translations for the app are stored in the [`locales`](lescan/locales) directory as .yml files.
To add a new language, open the related file that you want to translate and then add the translations for each key.
If you need to add another language, just write your language code as a key (e.g. `zh-CN`) and then write your translations under that key.

## Example:

```yaml
file_menu:
  file:
    en: "File"
    tr: "Dosya"
    # Add your translation here
    zh-CN: "文件"
```

## Steps:

### If you want to test your translations:
1. Install git from [here](https://git-scm.com/downloads).
2. Install rust from [here](https://www.rust-lang.org/tools/install).
3. Fork the repository.
4. Clone the forked repository with:
```bash
git clone https://github.com/[your_user_name]/lescan.git
```
5. Change directory to the project folder with:
```bash
cd lescan
```
6. Open the locale files with the editor of your choice, and add your translations.
7. Build the project and test your translations with:
```bash
cargo run
```
8. If you are satisfied with your translations, commit your changes with:
```bash
git add *
git commit -m "Add translations for [language code]"
git push
```
9. Open a pull request with your changes.

### If you don't want to test your translations:
1. Install git from [here](https://git-scm.com/downloads).
2. Fork the repository.
3. Clone the forked repository with:
```bash
git clone https://github.com/[your_user_name]/lescan.git
```
4. Open the locale files with the editor of your choice, and add your translations.
If you are satisfied with your translations, commit your changes with:
```bash
git add *
git commit -m "Add translations for [language code]"
git push
```
5. Open a pull request with your changes.
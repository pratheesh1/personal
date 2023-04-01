## Motivation

Sick and tired ot having no good way to visualize the dependancy graph of a program in USasK. Check out the [requirements](requirements.txt) for the list of dependencies.

## Usage

1.  Install dependencies from `requirements.txt`

```bash
pip install -r requirements.txt
```

2.  Run `main.py` with the couse lists file as the first argument.

```bash
python3 main.py <course_lists_file> [Program Name]
```

For example, both of the following are valid commands.

```bash
python3 main.py example.csv 'Bachelor of Science Honours [B.Sc. (BMSC) Honours]'

python3 main.py bmsc_honors.csv
```

Header defaults to `Study Plan` if not provided. 3. Open `index.html` in a browser.

## Sample csv file

`bmsc_honors.csv`
| Subject | Code | Name | Credits | Prerequisites | required | notes |
| ------- | ---- | ------------------------------------------- | ------- | -------------------------- | -------- | ------------------------------------- |
| HIL | 120 | Knowledge Mind and Existence | 3 | | Y | |
| HIL | 121 | Introduction to World Philosophies | 3 | | Y | |
| INDG | 107 | Introduction to Canadian Indigenous Studies | 3 | | Y | |
| MATH | 110 | Calculus I | 3 | | Y | |
| BIOL | 120 | The Nature of Life | 3 | | Y | |
| BIOL | 121 | The Diversity of Life | 3 | | N | |
| STAT | 246 | Introduction to Biostatistics | 3 | MATH 110,BIOL 120,BIOL 121 | Y | Permission of the Department required |

`Subject`, `Code`, `Name` are required fileds.

Default values:

- `Credits`: 3
- `Prerequisites`: []
- `required`: Y
- `notes`: ''

Note: I did not use a virtal environment for this project. But used python 3.10.6 for development. So it is the recommended version. But it should work with python 3.6+.

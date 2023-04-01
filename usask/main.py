from pyvis.network import Network
import networkx as nx
import csv
import sys
from enum import IntEnum


class Course:
    def __init__(self: str, subject: str, code: str, name: str, credits: int = 3, prereqs: list[str] = [], required: bool = True, notes: str = None, completed: bool = False):
        self.subject = subject
        self.code = code
        self.name = name
        self.credits = credits
        self.prereqs = prereqs
        self.required = required
        self.notes = notes
        self.completed = completed

    def __repr__(self) -> str:
        return f'{self.subject} {self.code}.{self.credits}'

    def __str__(self) -> str:
        return (f'{self.fmt}: {self.name}\n\n'
                f'Subject: {self.subject}\n'
                f'Code: {self.code}\n'
                f'Name: {self.name}\n'
                f'Credits: {self.credits}\n'
                f'Prereqs: {len(self.prereqs) > 0 and self.prereqs or "NA"}\n'
                f'Required: {self.required}\n'
                f'Notes: {self.notes or "NA" }\n'
                f'Completed: {self.completed}')

    def __hash__(self) -> int:
        return hash((self.subject, self.code))

    def __eq__(self, other) -> bool:
        return (self.subject, self.code) == (other.subject, other.code)

    def __ne__(self, __value: object) -> bool:
        return not self.__eq__(__value)

    @property
    def fmt(self) -> str:
        return self.__repr__()


class Plan:
    def __init__(self, courses: list[Course]):
        self.__course_array: list[Course] = courses
        self.__prereqs_array: list[list[Course]] = []

        self.__coursemap: dict[tuple[str, str], Course] = {}

        self.__build_plan()
        self.__build_pre_reqs()

    def __repr__(self) -> str:
        return str(list(map(lambda x: x.__repr__(), self.__coursemap.values())))

    def __build_plan(self) -> None:
        for course in self.__course_array:
            self.__coursemap[(course.subject, course.code)] = course

    def __build_pre_reqs(self) -> None:
        for course in self.__coursemap.values():
            course_prereqs: list[Course] = []
            for prereq in course.prereqs:
                if self.__coursemap.get(prereq) is None:
                    print("Error: Prereq '{}_{}' required by '{}_{}' not found in study plan.".format(
                          prereq[CourseCode.SUBJECT], prereq[CourseCode.CODE], course.subject, course.code), file=sys.stderr)
                    sys.exit(1)
                else:
                    course_prereqs.append(self.__coursemap[prereq])
            self.__prereqs_array.append(course_prereqs)

    @property
    def prereqs(self) -> list[list[Course]]:
        return self.__prereqs_array

    @property
    def courses(self) -> list[Course]:
        return self.__course_array

# ----------------------------------------------------------------------------------------------


class Fields(IntEnum):
    SUBJECT = 0
    CODE = 1
    NAME = 2
    CREDITS = 3
    PREREQS = 4
    REQUIRED = 5
    NOTES = 6
    COMPLETED = 7


class CourseCode(IntEnum):
    SUBJECT = 0
    CODE = 1
    CREDITS = 2


legend = ["Required with prereqs",
          "Required without prereqs", "Elective", "Completed"]


class Legend(IntEnum):
    REQUIRED_WITH_PREREQS = 0
    REQUIRED_WITHOUT_PREREQS = 1
    ELECTIVE = 2
    COMPLETED = 3


def course_from_csv(row: list[str]) -> Course:
    row_len = len(row)
    subject = row[Fields.SUBJECT]
    code = row[Fields.CODE]
    name = row[Fields.NAME]

    if (subject == '' or code == '' or name == '' or row_len < len(Fields)):
        print("Error: Invalid course entry: {}".format(row), file=sys.stderr)
        sys.exit(1)

    credits = int(row[Fields.CREDITS])

    prereqs_map: list[str] = list(
        map(lambda x: x.strip(), row[Fields.PREREQS].split(',')))
    prereqs: list[tuple[str, str]] = len(prereqs_map) > 0 and prereqs_map[0] != '' and list(map(lambda s: (
        (s.split(' ')[CourseCode.SUBJECT].strip(), s.split(' ')[CourseCode.CODE].strip())), prereqs_map)) or []

    required = row[Fields.REQUIRED] == 'Y'
    notes = row[Fields.NOTES]
    completed = row[Fields.COMPLETED] == 'Y'

    return Course(subject, code, name,
                  credits, prereqs, required, notes, completed)


def course_gen(file: str) -> Course:
    with open(file, 'r') as f:
        reader = csv.reader(f)
        for row in reader:
            # Skip the header row
            if row[0] == 'Subject':
                continue
            yield course_from_csv(row)

# ----------------------------------------------------------------------------------------------


argc = len(sys.argv)
if argc < 2:
    print('Usage: python3 {} <csv file> [Program name]'.format(
        sys.argv[0]), file=sys.stderr)
    sys.exit(1)

file = sys.argv[1]
courses: list[Course] = []
for course in course_gen(file):
    courses.append(course)

plan = Plan(courses)

# ----------------------------------------------------------------------------------------------


G = nx.Graph()
net = Network(height='750px', bgcolor="white", font_color="black",
              directed=True, select_menu=True, filter_menu=True, neighborhood_highlight=True, cdn_resources='in_line')

heading = argc >= 3 and sys.argv[2] or "Study Plan"
net.heading = heading
# net.show_buttons(filter_=['physics'])

for i, course in enumerate(plan.courses):
    group: str
    if course.required and len(course.prereqs) > 0:
        group = legend[Legend.REQUIRED_WITH_PREREQS]
    elif course.required:
        group = legend[Legend.REQUIRED_WITHOUT_PREREQS]
    else:
        group = legend[Legend.ELECTIVE]

    if course.completed:
        group = legend[Legend.COMPLETED]

    G.add_node(course.fmt, label=course.fmt,
               title=str(course), value=course.credits, group=group)

    for prereq in plan.prereqs[i]:
        id = '{} -> {}'.format(prereq.fmt, course.fmt)
        G.add_edge(prereq.fmt, course.fmt, id=id)


# Add Legend Nodes
step = 30
x_offset = 2000
y_offset = -100
legend_nodes = [
    (
        G.number_of_nodes() + node,
        {
            'group': legend[node],
            'label': legend[node],
            'size': 50,
            # 'fixed': True,  # So that we can move the legend nodes around to arrange them better
            'physics': False,
            'x': f'{x_offset}px',
            'y': f'{y_offset + node * step}px',
            'shape': 'box',
            'widthConstraint': 200,
            'font': {'size': 50},
            'id': legend[node],
        }
    )
    for node in range(len(legend))
]
G.add_nodes_from(legend_nodes)

net.from_nx(G)
net.generate_html("index.html")
net.get_network_data()

# Refer https://github.com/WestHealth/pyvis/issues/219
html_str = net.html.replace(
    '<center>\n<h1>' + heading + '</h1>\n</center>', '')
# this actually edits heading, change code to remove second occurance
h = open('index.html', 'w')
h.write(html_str)
h.close()

print("Done")

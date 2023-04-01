from pyvis.network import Network
import networkx as nx
import csv
import sys
from enum import IntEnum


class Course:
    def __init__(self: str, subject: str, code: str, name: str, credits: int = 3, prereqs: list[str] = [], coreqs: list[str] = [], required: bool = True, notes: str = None, completed: bool = False):
        self.subject = subject
        self.code = code
        self.name = name
        self.credits = credits
        self.prereqs = prereqs
        self.coreqs = coreqs
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
                f'Coreqs: {len(self.coreqs) > 0 and self.coreqs or "NA"}\n'
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
        self.__has_errors = False
        self.__errors: list[str] = []
        self.__warnings: list[str] = []

        self.__course: list[Course] = courses
        self.__prereqs: list[list[Course]] = []
        self.__coreqs: list[list[Course]] = []

        self.__coursemap: dict[tuple[str, str], Course] = {}

        self.__build_plan()
        self.__build_reqs()

        self.__validate()

    def __repr__(self) -> str:
        return str(list(map(lambda x: x.__repr__(), self.__coursemap.values())))

    def __build_plan(self) -> None:
        for course in self.__course:
            if self.__coursemap.get((course.subject, course.code)) is not None:
                print("Error: Duplicate course '{}' found in study plan.".format(
                    course.fmt), file=sys.stderr)
                sys.exit(1)
            self.__coursemap[(course.subject, course.code)] = course

    def __build_reqs(self) -> None:
        for course in self.__coursemap.values():
            course_prereqs: list[Course] = []
            course_coreqs: list[Course] = []

            for prereq in course.prereqs:
                if self.__coursemap.get(prereq) is None:
                    self.__errors.append("Prerequisite '{} {}' required by '{}' not found in study plan.".format(
                        prereq[CourseCode.SUBJECT], prereq[CourseCode.CODE], course.fmt))
                    self.__has_errors = True
                else:
                    course_prereqs.append(self.__coursemap[prereq])

            for corereq in course.coreqs:
                if self.__coursemap.get(corereq) is None:
                    self.__errors.append("Corequisite '{} {}' required by '{}' not found in study plan.".format(
                        corereq[CourseCode.SUBJECT], corereq[CourseCode.CODE], course.fmt))
                    self.__has_errors = True
                else:
                    course_coreqs.append(self.__coursemap[corereq])

            self.__prereqs.append(course_prereqs)
            self.__coreqs.append(course_coreqs)

    def __validate(self) -> None:
        for course in self.__course:
            if course.completed:
                for prereq in course.prereqs:
                    if not self.__coursemap[prereq].completed:
                        self.__errors.append("Course '{}' marked as completed, but prerequisite '{} {}' is not.".format(
                            course.fmt, prereq[CourseCode.SUBJECT], prereq[CourseCode.CODE]))
                        self.__has_errors = True
                for corereq in course.coreqs:
                    if not self.__coursemap[corereq].completed:
                        self.__warnings.append("Course '{}' marked as completed, but corequisite '{} {}' is not.".format(
                            course.fmt, corereq[CourseCode.SUBJECT], corereq[CourseCode.CODE]))

        if self.__has_errors:
            self.__print_errors()
            sys.exit(1)

    def __print_errors(self) -> None:
        len(self.__errors) > 0 and print(
            "Error(s) found:", file=sys.stderr)
        for error in self.__errors:
            print('\t', error, file=sys.stderr)

        len(self.__warnings) > 0 and print(
            "\nWarning(s) found:", file=sys.stderr)
        for warning in self.__warnings:
            print('\t', warning, file=sys.stderr)

    @property
    def prereqs(self) -> list[list[Course]]:
        return self.__prereqs

    @property
    def coreqs(self) -> list[list[Course]]:
        return self.__coreqs

    @property
    def courses(self) -> list[Course]:
        return self.__course

# ----------------------------------------------------------------------------------------------


class Fields(IntEnum):
    SUBJECT = 0
    CODE = 1
    NAME = 2
    CREDITS = 3
    PREREQS = 4
    COREQS = 5
    REQUIRED = 6
    NOTES = 7
    COMPLETED = 8


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

    if (subject.strip() == '' or code.strip() == '' or name.strip() == '' or row_len < len(Fields)):
        print("Error: Invalid course entry: {}".format(row), file=sys.stderr)
        sys.exit(1)

    credits = row[Fields.CREDITS].strip() != '' and int(
        row[Fields.CREDITS]) or 3

    prereqs_map: list[str] = list(
        map(lambda x: x.strip(), row[Fields.PREREQS].split(',')))
    prereqs: list[tuple[str, str]] = len(prereqs_map) > 0 and prereqs_map[0].strip() != '' and list(map(lambda s: (
        (s.split(' ')[CourseCode.SUBJECT].strip(), s.split(' ')[CourseCode.CODE].strip())), prereqs_map)) or []

    coreqs_map: list[str] = list(
        map(lambda x: x.strip(), row[Fields.COREQS].split(',')))
    coreqs: list[tuple[str, str]] = len(coreqs_map) > 0 and coreqs_map[0].strip() != '' and list(map(lambda s: (
        (s.split(' ')[CourseCode.SUBJECT].strip(), s.split(' ')[CourseCode.CODE].strip())), coreqs_map)) or []

    required = not (row[Fields.REQUIRED] ==
                    'N' and row[Fields.REQUIRED].strip() != '')
    notes = row[Fields.NOTES]
    completed = row[Fields.COMPLETED] == 'Y'

    return Course(subject, code, name,
                  credits, prereqs, coreqs, required, notes, completed)


def course_gen(file: str) -> Course:
    with open(file, 'r') as f:
        reader = csv.reader(f)
        for row in reader:
            # Skip the header row and  any empty rows
            if row[0] == 'Subject' or row[0] == '' or row[0].startswith('//'):
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
    if course.completed:
        group = legend[Legend.COMPLETED]
    elif course.required:
        if len(course.prereqs) > 0:
            group = legend[Legend.REQUIRED_WITH_PREREQS]
        else:
            group = legend[Legend.REQUIRED_WITHOUT_PREREQS]
    else:
        group = legend[Legend.ELECTIVE]

    G.add_node(course.fmt, label=course.fmt,
               title=str(course), value=course.credits, group=group)

    for prereq in plan.prereqs[i]:
        # id = '{} -> {}'.format(prereq.fmt, course.fmt)
        G.add_edge(prereq.fmt, course.fmt)


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

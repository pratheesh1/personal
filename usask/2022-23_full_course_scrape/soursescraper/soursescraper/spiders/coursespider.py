from colorama import Fore, Style
import scrapy


class CoursespiderSpider(scrapy.Spider):
    name = "coursespider"
    allowed_domains = ["catalogue.usask.ca"]
    start_urls = ["https://catalogue.usask.ca"]
    course_url = "https://catalogue.usask.ca/?subj_code={}"

    def parse(self, response):
        options = response.xpath("//select[@id='subj-code']/option/@value").getall()

        for option in options:
            if option == "":
                continue

            yield scrapy.Request(
                self.course_url.format(option), callback=self.parse_course_search
            )

    def parse_course_search(self, response):
        course_name = response.xpath("//div/h4/a/text()").getall()
        course_url = response.xpath("//div/h4/a/@href").getall()

        for name, url in zip(course_name, course_url):
            yield scrapy.Request(
                url=url, callback=self.parse_course, cb_kwargs={"name": name.strip()}
            )

    def parse_course(self, response, name: str):
        course_detail = {"Subject": response.request.cb_kwargs["name"]}

        details = response.css("#section-0-subsection-0 p").get()
        details = (
            details.replace("<strong>", "")
            .replace("</strong>", "")
            .replace("<p>", "")
            .replace("</p>", "")
            .strip()
            .replace(
                ": ", ":\n"
            )  # beacuse one cannot be bothered to uniformly format their html
            .split("<br>")
        )
        for detail in details[1:]:
            currDetail = detail.split("\n")
            currDetail = [x.strip().replace(":", "") for x in currDetail if x]
            course_detail[currDetail[0]] = currDetail[1]

        course_detail["Description"] = (
            response.css("#Description-subsection-0 p::text").get().strip()
        )

        course_requirements_title = [
            x.replace(":", "").strip()
            for x in response.xpath(
                "//div[@id='Description-subsection-0']/p[2]/b/text()"
            ).getall()
        ]
        course_requirements = [
            x.strip()
            for x in response.xpath(
                "//div[@id='Description-subsection-0']/p[2]/text()"
            ).getall()
            if x.strip()
        ]
        for title, requirement in zip(course_requirements_title, course_requirements):
            course_detail[title] = requirement

        self.print_info("Crawled", name)
        yield course_detail

    def print_info(self, name, details):
        print(f"{Fore.CYAN}{name}: {Style.RESET_ALL}{details}")

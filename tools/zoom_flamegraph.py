import fileinput
import io
from html import escape
from html.entities import name2codepoint
from html.parser import HTMLParser


class MyHTMLParser(HTMLParser):
    script = False
    in_a_g = False
    record_next_y = False
    g_buffer = None
    y_limit = None
    g_current_y = None
    g_dict = dict()

    def _print(self, s):
        if self.g_buffer is not None:
            print(s, file=self.g_buffer)
        else:
            print(s)

    def handle_starttag(self, tag, attrs):
        formatted_attrs = ""
        for k, v in attrs:
            formatted_attrs += f' {k}="{v}"'
            if k == "y":
                y = float(v)
                if self.record_next_y:
                    self.y_limit = y
                    self.record_next_y = False
                if self.g_current_y is None or self.g_current_y > y:
                    self.g_current_y = y

        if tag == "script":
            self.script = True
        elif tag == "g":
            self.in_a_g = True
            self.g_buffer = io.StringIO()
            self.g_current_y = None

        self._print(f"<{tag}{formatted_attrs}>")

    def handle_endtag(self, tag):
        self._print(f"</{tag}>")

        if tag == "script":
            self.script = False
        elif tag == "g":
            if self.y_limit and self.g_current_y <= self.y_limit:
                print(self.g_buffer.getvalue())

            self.in_a_g = False
            self.g_buffer.close()
            self.g_buffer = None

    def handle_data(self, data):
        if not self.script:
            data = escape(data)
        if "::bench " in data:
            self.record_next_y = True
        self._print(data)

    def handle_decl(self, data):
        self._print(f"<!{data}>")

    def handle_pi(self, data):
        self._print(f"<?{data}>")

    def handle_comment(self, data):
        self._print(f"<!--{data}-->")

    def handle_entityref(self, name):
        c = chr(name2codepoint[name])
        raise NotImplementedError(f"Named ent: {c}")

    def handle_charref(self, name):
        if name.startswith("x"):
            c = chr(int(name[1:], 16))
        else:
            c = chr(int(name))
        raise NotImplementedError(f"Num ent: {c}")


parser = MyHTMLParser()
for line in fileinput.input():
    parser.feed(line)

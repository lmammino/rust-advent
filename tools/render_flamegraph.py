import fileinput
import io
from html import escape
from html.entities import name2codepoint
from html.parser import HTMLParser

from PIL import Image, ImageDraw, ImageFont


def rect_parse(attrs):
    ret = dict(attrs)
    for k in "x y width height fg:x fg:w".split():
        if k in ret:
            ret[k] = float(ret[k].strip("%"))
    if ret["fill"] == "url(#background)":
        del ret["fill"]
    else:
        ret["fill"] = tuple(map(int, ret["fill"].strip("rgb()").split(",")))
    return frozenset(ret.items())


class MyHTMLParser(HTMLParser):
    record_next_rect = False
    rects = dict()
    record_next_text = True
    last_title = "Dummy text"

    def handle_starttag(self, tag, attrs):
        if tag == "rect":
            coords = rect_parse(attrs)
            if self.record_next_rect:
                self.base_rect = dict(coords)
                self.record_next_rect = False
            self.last_rect_key = coords
            self.rects[coords] = self.last_title[: self.last_title.rfind(" samples, ")] + ")"
            # print(coords)
        elif tag == "title":
            self.record_next_text = True
            self.last_title = ""

    def handle_endtag(self, tag):
        if tag == "title":
            self.record_next_text = False

    def handle_data(self, data):
        if "::bench " in data:
            self.record_next_rect = True
        if self.record_next_text:
            self.last_title += data

    def handle_decl(self, data):
        # self._print(f"<!{data}>")
        pass

    def handle_pi(self, data):
        # self._print(f"<?{data}>")
        pass

    def handle_comment(self, data):
        # self._print(f"<!--{data}-->")
        pass

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


x_offset = int(parser.base_rect["fg:x"])
y_offset = 0
w = int(parser.base_rect["fg:w"])
size_y = int(parser.base_rect["y"])

# print(parser.base_rect)
def draw_image(w, size_y, x_offset, y_offset, multiplier=1):
    mult = 1
    img = Image.new(
        "RGB",
        (int((w - x_offset) * multiplier), int(size_y - y_offset)),
        color="white",
    )
    draw = ImageDraw.Draw(img)
    # font = ImageFont.truetype("sans-serif.ttf", 16)
    for k, v in parser.rects.items():
        kd = dict(k)
        try:
            x = int(kd["fg:x"])
            w = int(kd["fg:w"])
            y = int(kd["y"])
            h = int(kd["height"])
            x, y = (x - x_offset, y - y_offset)
            if x < 0 or y < 0:
                continue
            draw.rectangle(
                (
                    (x * multiplier, y),
                    (x * multiplier + w * multiplier, y + h),
                ),
                fill=kd.get("fill", "white"),
                outline="black",
            )
            tx, _ = draw.textsize(v)
            m = 1 if tx < w else tx / w
            mult = max((m, mult))
            draw.text((x * multiplier + 5, y + 2), v, (0, 0, 0))  # , font=font)
        except KeyError:
            pass
    return img, mult


img, m = draw_image(w, size_y, x_offset, y_offset)
img.save("bench.png")

img, m = draw_image(w, size_y, x_offset, y_offset, min(10, m))
img.save("bench_wide.png")

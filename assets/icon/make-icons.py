#!/usr/bin/env python3
"""生成易言的 Y 图标：菜单栏模板（menu.svg / menu.pdf）、应用图标（logo.png）、README 页头（yiyan-mark.svg）。

改形状就改下面这几个常数再跑 `python3 assets/icon/make-icons.py`（在仓库根目录）。
PDF 是手写的最小文件：机器上没有 rsvg-convert，而菜单栏图标必须是矢量 PDF（Info.plist 的图标键指向它）。
"""

from PIL import Image, ImageDraw

# 画布与 Y 的几何，单位 pt，原点在左上角；22×16 是菜单栏图标的既定尺寸
W, H = 22.0, 16.0
TOP, BOT = 2.2, 13.8
CX = W / 2
FORK = 8.6
HALF = 3.1
STEM = 1.75

# 应用图标：深竹青底 + 米白的 Y
SIZE, SUPERSAMPLE = 1024, 4
BACKGROUND = (30, 95, 75, 255)
FOREGROUND = (245, 242, 232, 255)


def oriented(points):
    """统一绕向：PDF 的非零填充下，两条臂绕向相反会把重叠处抵消成空洞。"""
    area = sum(
        x0 * y1 - x1 * y0
        for (x0, y0), (x1, y1) in zip(points, points[1:] + points[:1])
    )
    return points if area >= 0 else points[::-1]


def shapes():
    """Y = 两条臂（平行四边形）+ 一根竖干（矩形），重叠部分靠非零填充合并。"""
    left = [(CX - 2 * HALF, TOP), (CX - 2 * HALF + 2 * STEM, TOP), (CX + STEM, FORK), (CX - STEM, FORK)]
    right = [(CX + 2 * HALF, TOP), (CX + 2 * HALF - 2 * STEM, TOP), (CX - STEM, FORK), (CX + STEM, FORK)]
    stem = [(CX - STEM, FORK), (CX + STEM, FORK), (CX + STEM, BOT), (CX - STEM, BOT)]
    return [oriented(p) for p in (left, right, stem)]


def svg_paths():
    return [
        '  <path d="M' + " L".join(f"{x:.2f},{y:.2f}" for x, y in pts) + ' Z"/>'
        for pts in shapes()
    ]


def write_menu_svg(path):
    body = "\n".join(svg_paths())
    open(path, "w").write(
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W:g} {H:g}" '
        f'width="{W:g}pt" height="{H:g}pt">\n<g fill="#000000">\n{body}\n</g>\n</svg>\n'
    )


def write_menu_pdf(path):
    ops = ["0 g"]
    for pts in shapes():
        x0, y0 = pts[0]
        # PDF 的 y 轴朝上
        ops.append(f"{x0:.2f} {H - y0:.2f} m")
        ops += [f"{x:.2f} {H - y:.2f} l" for x, y in pts[1:]]
        ops.append("h")
    ops.append("f")
    content = "\n".join(ops).encode()
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {W:g} {H:g}] /Contents 4 0 R /Resources << >> >>".encode(),
        b"<< /Length " + str(len(content)).encode() + b" >>\nstream\n" + content + b"\nendstream",
    ]
    out, offsets = bytearray(b"%PDF-1.4\n"), []
    for index, body in enumerate(objects, 1):
        offsets.append(len(out))
        out += f"{index} 0 obj\n".encode() + body + b"\nendobj\n"
    table = len(out)
    out += f"xref\n0 {len(objects) + 1}\n0000000000 65535 f \n".encode()
    for offset in offsets:
        out += f"{offset:010d} 00000 n \n".encode()
    out += f"trailer\n<< /Size {len(objects) + 1} /Root 1 0 R >>\nstartxref\n{table}\n%%EOF\n".encode()
    open(path, "wb").write(bytes(out))


def write_logo_png(path):
    side = SIZE * SUPERSAMPLE
    image = Image.new("RGBA", (side, side), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    draw.rounded_rectangle([0, 0, side - 1, side - 1], radius=int(0.22 * side), fill=BACKGROUND)
    scale = side / W
    offset = (side - H * scale) / 2
    for pts in shapes():
        draw.polygon([(x * scale, y * scale + offset) for x, y in pts], fill=FOREGROUND)
    image.resize((SIZE, SIZE), Image.LANCZOS).save(path)


def write_mark_svg(path):
    body = "\n".join(svg_paths())
    open(path, "w").write(
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W:g} {H:g}" width="110" height="80">\n'
        f'  <rect width="{W:g}" height="{H:g}" rx="3.4" fill="#1E5F4B"/>\n'
        f'  <g fill="#F5F2E8">\n{body}\n  </g>\n</svg>\n'
    )


if __name__ == "__main__":
    write_menu_svg("assets/icon/menu.svg")
    write_menu_pdf("assets/icon/menu.pdf")
    write_logo_png("assets/icon/logo.png")
    write_mark_svg("assets/icon/yiyan-mark.svg")
    print("menu.svg / menu.pdf / logo.png / yiyan-mark.svg 已生成")

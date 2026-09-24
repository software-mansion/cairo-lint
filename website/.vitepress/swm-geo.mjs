import fs from "node:fs";
import path from "node:path";

const ORGANIZATION_ID = "https://swmansion.com/#organization";
const SITE = "https://docs.swmansion.com";

const escapeRegExp = (value) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

const decode = (value) =>
    value
        .replace(/&amp;/g, "&")
        .replace(/&lt;/g, "<")
        .replace(/&gt;/g, ">")
        .replace(/&quot;/g, '"')
        .replace(/&#(?:39|x27);/g, "'")
        .trim();

const projectFromBase = (base) => base.replace(/\//g, "");

// Same @id as swmansion.com, so engines read one company across both domains.
export function swmStructuredData({ siteConfig }) {
    const { base, description, title } = siteConfig.site;
    const project = projectFromBase(base);

    return [
        "script",
        { type: "application/ld+json" },
        JSON.stringify({
            "@context": "https://schema.org",
            "@graph": [
                {
                    "@type": "Organization",
                    "@id": ORGANIZATION_ID,
                    name: "Software Mansion",
                    url: "https://swmansion.com",
                    sameAs: [
                        "https://github.com/software-mansion",
                        "https://www.linkedin.com/company/software-mansion/",
                        "https://twitter.com/swmansion",
                        "https://www.youtube.com/c/SoftwareMansion",
                    ],
                },
                {
                    "@type": "SoftwareSourceCode",
                    name: title,
                    ...(description ? { description } : {}),
                    ...(project
                        ? {
                              codeRepository: `https://github.com/software-mansion/${project}`,
                          }
                        : {}),
                    author: { "@id": ORGANIZATION_ID },
                    maintainer: { "@id": ORGANIZATION_ID },
                },
            ],
        }),
    ];
}

function collectHtml(dir, root = dir, found = []) {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
        const full = path.join(dir, entry.name);
        if (entry.isDirectory()) collectHtml(full, root, found);
        else if (entry.name.endsWith(".html") && entry.name !== "404.html")
            found.push(path.relative(root, full));
    }
    return found;
}

export function buildLlmsTxt({ base, description, files, readFile, title }) {
    const entries = [];

    for (const file of files) {
        const html = readFile(file);
        const raw = /<title[^>]*>([\s\S]*?)<\/title>/i.exec(html)?.[1] ?? "";
        const name = decode(raw).replace(new RegExp(`\\s*\\|\\s*${escapeRegExp(title)}$`), "");
        if (!name) continue;

        const detail = decode(
            /<meta[^>]+name="description"[^>]+content="([^"]*)"/i.exec(html)?.[1] ?? "",
        );
        const route = file.replace(/index\.html$/, "").replace(/\.html$/, "");
        entries.push(`- [${name}](${SITE}${base}${route})${detail ? `: ${detail}` : ""}`);
    }

    const lines = [`# ${title}`];
    if (description) lines.push("", `> ${description}`);
    if (entries.length) lines.push("", "## Documentation", "", ...entries.sort());
    lines.push(
        "",
        "## About",
        "",
        `- [Software Mansion](https://swmansion.com): maintainer of ${title}`,
        "",
    );

    return lines.join("\n");
}

export async function writeLlmsTxt(siteConfig) {
    const { outDir, site } = siteConfig;
    const files = collectHtml(outDir);

    await fs.promises.writeFile(
        path.join(outDir, "llms.txt"),
        buildLlmsTxt({
            base: site.base,
            description: site.description,
            files,
            readFile: (file) => fs.readFileSync(path.join(outDir, file), "utf8"),
            title: site.title,
        }),
        "utf8",
    );
}

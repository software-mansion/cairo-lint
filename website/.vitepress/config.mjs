import { defineConfig } from "vitepress";
import * as syntaxes from "./syntaxes.mjs";
import { getLintsSidebar } from "./lints_sidebar.mjs";

const base = "/cairo-lint/";
const absoluteBase = `https://docs.swmansion.com${base}`;
const lang = "en-US";

const getSidebar = () => ({
    "/docs": [
        {
            text: "Overview",
            items: [p("Introduction", "/docs")],
        },
        {
            text: "Lints",
            items: getLintsSidebar(),
        },
    ],
});

const telegramIcon = `
  <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
    <title>Telegram</title>
    <path d="M12 0C5.37058 0 0 5.37135 0 12C0 18.6286 5.37135 24 12 24C18.6294 24 24 18.6286 24 12C24 5.37135 18.6286 0 12 0ZM17.8939 8.22116L15.9244 17.5022C15.7788 18.1603 15.3871 18.3197 14.8405 18.0101L11.8405 15.799L10.3935 17.1925C10.2341 17.352 10.0986 17.4875 9.7889 17.4875L10.0018 14.4341L15.5613 9.4111C15.8036 9.19819 15.5079 9.07742 15.1881 9.29032L8.31716 13.6157L5.35587 12.6914C4.71252 12.4885 4.69781 12.048 5.49135 11.7383L17.0609 7.27665C17.5982 7.0831 18.0674 7.40748 17.8932 8.22039L17.8939 8.22116Z"/>
  </svg>
`;

export default defineConfig({
    title: "Cairo lint",
    description: "Cairo lint is a static code analyzer tool for Cairo language.",
    lang,
    base,

    head: [
        ["meta", { httpEquiv: "Content-Language", content: lang }],
        ["link", { rel: "icon", href: `${base}favicon.svg`, type: "image/x-icon" }],
        ["link", { rel: "apple-touch-icon", href: `${base}apple-touch-icon.png` }],
        ["meta", { name: "apple-mobile-web-app-title", content: "Cairo lint" }],
        ["meta", { name: "twitter:card", content: "summary_large_image" }],
        ["meta", { name: "twitter:site", content: "@swmansionxyz" }],
        ["meta", { name: "twitter:creator", content: "@_Tobysmy_" }],
        [
            "meta",
            {
                property: "og:title",
                content: "Cairo lint",
            },
        ],
        [
            "meta",
            {
                property: "og:description",
                content: "Cairo lint is a static code analyzer tool for Cairo language.",
            },
        ],
        [
            "meta",
            {
                property: "og:image:alt",
                content: "Cairo lint is a static code analyzer tool for Cairo language.",
            },
        ],
        ["meta", { property: "og:image:type", content: "image/png" }],
        ["meta", { property: "og:image:width", content: "1280" }],
        ["meta", { property: "og:image:height", content: "640" }],
    ],

    lastUpdated: true,

    themeConfig: {
        logo: {
            light: "/favicon.svg",
            dark: "/favicon.svg",
            alt: "Cairo lint",
        },
        siteTitle: "Cairo lint",

        nav: [
            { text: "Download", link: "https://docs.swmansion.com/scarb/download.html" },
            { text: "Documentation", link: "/docs" },
        ],

        sidebar: getSidebar(),

        socialLinks: [
            { icon: "github", link: "https://github.com/software-mansion/cairo-lint" },
            { icon: "twitter", link: "https://twitter.com/swmansionxyz" },
            {
                icon: {
                    svg: telegramIcon,
                },
                ariaLabel: "Telegram",
                link: "https://t.me/cairolint",
            },
        ],

        editLink: {
            pattern: "https://github.com/software-mansion/cairo-lint/tree/main/website/:path",
            text: "Edit this page on GitHub",
        },

        search: {
            provider: "local",
        },
    },

    sitemap: {
        hostname: absoluteBase,
    },

    markdown: {
        languages: [syntaxes.cairo],
    },
});

function p(text, link) {
    return { text, link };
}

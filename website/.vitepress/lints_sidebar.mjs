import LintMetadata from "../lints_metadata.json";

export const getLintsSidebar = () =>
    LintMetadata.map((lint) => ({
        text: lint.name,
        link: `/docs/lints/${lint.name}`,
    }));

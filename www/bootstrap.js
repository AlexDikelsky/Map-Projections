init();

async function init() {
    if (typeof process == "object") {
        // We run in the npm/webpack environment.
        const [{Chart}, {main, setup}] = await Promise.all([
            import("../pkg/projections.js"),
            import("./index.js"),
        ]);
        main();
    } else {
        const [{Chart, default: init}, {main, setup}] = await Promise.all([
            import("../pkg/projections.js"),
            import("./index.js"),
        ]);
        await init();
        main();
    }
}

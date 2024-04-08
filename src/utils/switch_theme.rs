use dioxus::events::eval;

pub fn switch_theme() {
    eval(
        r#"
        const map = {
            dark: {
                fg: '#fff',
                'fg-5': '#eee',
                'fg-10': '#ddd',
                'fg-20': '#bbb',
                'bg': '#000',
                'bg-5': '#111',
                'bg-7-5': '#191919',
                'bg-10': '#222',
                'bg-20': '#444',
                fgcolor: '#aaf0ff',
            },
            light: {
                fg: '#000',
                'fg-5': '#111',
                'fg-10': '#222',
                'fg-20': '#444',
                bg: '#fff',
                'bg-5': '#eee',
                'bg-7-5': '#eaeaea',
                'bg-10': '#ddd',
                'bg-20': '#bbb',
                fgcolor: '#268',
            }
        };
        const theme = localStorage.getItem("mode") ?? "dark";

        if (theme === "dark") {
            localStorage.setItem("mode", "light");
        } else {
            localStorage.setItem("mode", "dark");
        }

        for (const [k, v] of Object.entries(map[localStorage.getItem("mode")])) {
            console.log(k,v);
            document
                .documentElement
                .style
                .setProperty("--" + k, v)
        }
        "#
    );
}

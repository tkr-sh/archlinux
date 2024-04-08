console.log("hi :3");
Bun.serve({
  port: 8081,
  async fetch(req: Request) {
    console.log("hi :3");
    const url = new URL(req.url);
    const param = url.pathname;

    let html = await fetch(`https://wiki.archlinux.org/title${param}`);
      // console.log(html.tex);
    return new Response(await html.text());
  },
});

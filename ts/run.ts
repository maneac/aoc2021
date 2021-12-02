const readDay: () => number = () => {
  if (Deno.args.length > 0) {
    return +Deno.args[1];
  }

  // default to latest day
  let day = 0;
  for (const entry of Deno.readDirSync("./ts")) {
    if (entry.isDirectory) {
      day++;
    }
  }
  return day;
};

const mod = await import("./day_" + readDay() + "/main.ts");
mod.main();

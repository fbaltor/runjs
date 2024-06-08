const {op_read_file, op_write_file, op_remove_file} = Deno.core.ops;

((globalThis) => {
  const core = Deno.core;

  function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
  }

  globalThis.console = {
    log: (...args) => {
      core.print(`[out]: ${argsToMessage(...args)}\n`, false);
    },
    error: (...args) => {
      core.print(`[err]: ${argsToMessage(...args)}\n`, true);
    },
  };

  globalThis.runjs = {
    readFile: (path) => {
      return op_read_file(path);
    },
    writeFile: (path, contents) => {
      return op_write_file(path, contents);
    },
    removeFile: (path) => {
      return op_remove_file(path);
    },
  };
})(globalThis);

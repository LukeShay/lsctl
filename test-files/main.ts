const run = async () => {
  await fetch("https://api.github.com/users/octocat");

  console.log("Hello, World?");
};

void run();

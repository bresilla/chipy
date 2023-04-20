{ pkgs, ... }:

{
  env.GREET = "devenv";
  packages = [ pkgs.git ];
  scripts.hello.exec = "echo hello from $GREET";
  enterShell = ''
    hello
    git --version
  '';
  languages.rust.enable = true;
}

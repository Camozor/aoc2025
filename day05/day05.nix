{ }: rec {
  pkgs = import <nixpkgs> { };
  lib = pkgs.lib;

  main = let
    lists = parseFile ./input.txt;
    v1Answer = countFreshIngredientsV1 lists;
  in { inherit v1Answer; };

  parseFile = path:
    let
      content = builtins.readFile path;
      splittedFile = lib.splitString "\n\n" content;
      freshList = lib.splitString "\n" (builtins.elemAt splittedFile 0);
      availableListRaw = lib.splitString "\n" (builtins.elemAt splittedFile 1);
      availableList = builtins.filter (x: x != "") availableListRaw;

    in { inherit freshList availableList; };

  countFreshIngredientsV1 = { freshList, availableList }:
    builtins.length
    (builtins.filter (id: isIngredientFresh id freshList) availableList);

  isIngredientFresh = idStr: freshList:
    builtins.any (freshRange: isIngredientFreshInRange idStr freshRange)
    freshList;

  isIngredientFreshInRange = idStr: freshRange:
    let
      splitted = lib.splitString "-" freshRange;
      first = lib.toInt (builtins.elemAt splitted 0);
      last = lib.toInt (builtins.elemAt splitted 1);
      id = lib.toInt idStr;
    in id >= first && id <= last;
}

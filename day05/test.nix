{ }:
let
  pkgs = import <nixpkgs> { };
  day05 = import ./day05.nix { };
in pkgs.lib.runTests {
  test_countFreshIngredientsV1 = {
    expr = day05.countFreshIngredientsV1 {
      freshList = [ "3-5" "10-14" "16-20" "12-18" ];
      availableList = [ "1" "5" "8" "11" "17" "32" ];
    };
    expected = 3;
  };

  test_isIngredientFresh = {
    expr = day05.isIngredientFresh "1" [ "1-3" ];
    expected = true;
  };

  test_isIngredientFreshInRange = {
    expr = day05.isIngredientFreshInRange "1" "1-3";
    expected = true;
  };

}


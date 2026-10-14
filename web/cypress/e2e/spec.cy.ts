// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

interface E2EIngredient {
  type: string;
  percentage: string;
  parts: string;
  weight: string;
}

interface E2EState {
  num_drinks: string;
  ingreds: E2EIngredient[];
}

const readE2EState = (win: Cypress.AUTWindow): E2EState => {
  const raw = win.localStorage.getItem("alc-calc/e2e");
  expect(raw, "e2e mirror in localStorage").to.be.a("string");
  return JSON.parse(raw as string) as E2EState;
};

describe("wasm app", () => {
  it("loads and renders a canvas", () => {
    cy.visit("/");
    cy.title().should("contain", "alc-calc");
    cy.get("canvas", { timeout: 30000 }).should("exist");
  });

  it("calculates weights for multiple ingredients via keyboard", () => {
    cy.visit("/");
    cy.get("canvas", { timeout: 30000 }).should("exist");

    // num_drinks is autofocused on launch, so this picks up where
    // test_table_calc_multiple_ingreds (src/ui/view/table/data_table.rs)
    // starts after its initial tab: add an ingredient, enter 2 drinks,
    // set Whiskey/40%/1.5 parts, select Wine via the dropdown, and enter
    // 16.5%/1 part
    cy.get("body").type("{ctrl}i");
    cy.get("body").type("2");
    cy.press(Cypress.Keyboard.Keys.TAB);
    cy.press(Cypress.Keyboard.Keys.TAB);
    cy.get("body").type("40");
    cy.press(Cypress.Keyboard.Keys.TAB);
    cy.get("body").type("1.5");
    cy.press(Cypress.Keyboard.Keys.TAB);
    cy.press(Cypress.Keyboard.Keys.ENTER);
    for (let i = 0; i < 4; i++) {
      cy.get("body").type("k");
    }
    cy.press(Cypress.Keyboard.Keys.ENTER);
    cy.press(Cypress.Keyboard.Keys.TAB);
    cy.get("body").type("16.5");
    cy.press(Cypress.Keyboard.Keys.TAB);
    cy.get("body").type("1");

    cy.window({ log: false }).should((win) => {
      const state = readE2EState(win);
      expect(state.num_drinks).to.equal("2");
      expect(state.ingreds.map((ingred) => ingred.type)).to.deep.equal([
        "Whiskey",
        "Wine",
      ]);
      expect(state.ingreds.map((ingred) => ingred.weight)).to.deep.equal([
        "66.3",
        "46.9",
      ]);
    });
  });
});

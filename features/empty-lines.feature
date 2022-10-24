Feature: remove double empty lines

  Scenario: Makefile has double empty lines
    Given a Makefile:
      """
      one: foo
        echo one


      two: bar
        echo bar
      """
    When running "makefmt"
    Then the Makefile should contain:
      """
      one: foo
        echo one

      two: bar
        echo bar
      """

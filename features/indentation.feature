Feature: correct double-indented recipes

  Rule: reduce indentation if all lines in a recipe contain multiple indentations

    Scenario: some entries contain double tabs
      Given a Makefile:
        """
        alpha: foo
            echo alpha one
            echo alpha two

        beta: bar
            echo beta one
            echo beta two
        """
      When running "makefmt"
      Then the Makefile should contain:
        """
        alpha: foo
          echo alpha one
          echo alpha two

        beta: bar
          echo beta one
          echo beta two
        """

  Rule: don't correct nested lines

    Scenario: a Makefile contains nested entries
      Given a Makefile:
        """
        alpha: foo
          if [ -f Makefile ]; then
            echo has Makefile
          fi
        """
      When running "makefmt"
      Then the Makefile should contain:
        """
        alpha: foo
          if [ -f Makefile ]; then
            echo has Makefile
          fi
        """

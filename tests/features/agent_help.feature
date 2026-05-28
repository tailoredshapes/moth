Feature: Recursive Agent Help

  Scenario: Run agent help option with full checks
    Given a moth workspace is initialized
    When the user runs moth with "--agent-help"
    Then the command succeeds
    And the output should contain exactly:
      """
      Subcommands:
        # Hook management
        moth hook help
        moth hook install [--global]
        moth hook uninstall [--global]

        # Initialize .moth/ directory
        moth init

        # Create a new issue
        moth new [--severity] [--no-edit] [--start] [--stdin]
      """
    And the output should contain "  Options:"
    And the output should contain "    [--help]"
    And the output should contain "    [--version]"
    And each comment line must start with "#" and be indented two spaces more than its command line
    And each deeper level must add two spaces of indentation
    And there must be a blank line between every command block
    And the word "Subcommands:" appears only once

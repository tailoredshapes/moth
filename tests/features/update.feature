Feature: Update issue description

  Scenario: Update current issue description from stdin
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    And the issue is started
    When the user updates the current issue with "New description content"
    Then the command succeeds
    And the last issue content contains "New description content"

  Scenario: Update issue by ID from stdin
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user updates the last created issue with "Updated via ID"
    Then the command succeeds
    And the last issue content contains "Updated via ID"

  Scenario: Update with no current issue fails
    Given a moth workspace is initialized
    When the user updates the current issue with "Some content"
    Then the command fails with "No current issue"

  Scenario: Update nonexistent issue fails
    Given a moth workspace is initialized
    When the user updates issue "nonexistent" with "Some content"
    Then the command fails

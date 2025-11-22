import { test, expect } from '@playwright/test';
import { TodoPage } from './pages/TodoPage';

test.describe('TODO Leptos app', () => {
  test('loads the home page and has the correct title', async ({ page, baseURL }) => {
    const todo = new TodoPage(page);
    await todo.goto(baseURL);

    await expect(page).toHaveTitle(/TODO-LAPTOS/i);

    // Optional sanity: ensure body is not empty after hydration
    const bodyHTML = await page.locator('body').innerHTML();
    expect(bodyHTML.length).toBeGreaterThan(0);
  });

  test('adds a todo and it appears in the list', async ({ page, baseURL }) => {
    const todo = new TodoPage(page);
    await todo.goto(baseURL);

    // Open the Add Todo modal (button with a plus icon next to "Create a Todo:")
    await todo.openAddModal();

    // Prepare unique values to avoid conflicts between runs
    const title = `E2E Todo ${Date.now()}`;
    const description = 'Created via Playwright test';

    // Fill the form and submit with the ADD button
    await todo.addTodo(title, description);

    // Expect the modal to close
    await todo.expectModalClosed();

    // The empty state should no longer be present
    await todo.expectEmptyStateGone();

    // The list header should be visible now
    await todo.expectListHeaderVisible();

    // Expect the new todo title to be visible in the list (unique match)
    await todo.expectTodoWithTitleVisibleOnce(title);
  });
});
import { expect, Locator, Page } from '@playwright/test';

export class TodoPage {
  readonly page: Page;

  // Controls / buttons
  readonly addTodoButton: Locator;
  readonly addButtonInModal: Locator;

  // Inputs
  readonly titleInput: Locator;
  readonly descriptionInput: Locator;

  // Text / sections
  readonly emptyState: Locator;
  readonly listHeader: Locator;

  constructor(page: Page) {
    this.page = page;

    this.addTodoButton = page.locator('button:has(i.fa-plus)');
    this.addButtonInModal = page.getByRole('button', { name: 'ADD' });

    this.titleInput = page.locator('#title');
    this.descriptionInput = page.locator('#description');

    this.emptyState = page.getByText('Currently no Todos defined');
    this.listHeader = page.getByText('Start working');
  }

  async goto(baseURL?: string) {
    await this.page.goto(baseURL || 'http://localhost:8080');
  }

  async openAddModal() {
    await Promise.all([
      this.addTodoButton.waitFor(),
      this.addTodoButton.click(),
    ]);
  }

  async addTodo(title: string, description: string) {
    await this.titleInput.fill(title);
    await this.descriptionInput.fill(description);
    await this.addButtonInModal.click();
  }

  async expectModalClosed() {
    await expect(this.addButtonInModal).toHaveCount(0);
  }

  async expectEmptyStateGone() {
    await expect(this.emptyState).toHaveCount(0);
  }

  async expectListHeaderVisible() {
    await expect(this.listHeader).toBeVisible();
  }

  async expectTodoWithTitleVisibleOnce(title: string) {
    await expect(this.page.getByText(title, { exact: true })).toHaveCount(1);
  }
}
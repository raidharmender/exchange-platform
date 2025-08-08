import { describe, it, expect } from 'vitest';

describe('App', () => {
  it('can be imported', () => {
    expect(() => import('./App')).not.toThrow();
  });
}); 
// Unit tests for the route normaliser (route-mapper.js)
// These tests cover all five URL pattern types and deduplication logic.

import { normalisePath, addRoute, groupRoutes } from '../crawler/route-mapper.js';

describe('normalisePath', () => {
  test('passes through static paths unchanged', () => {
    expect(normalisePath('https://example.com/api/users').pattern).toBe('/api/users');
  });

  test('normalises numeric IDs', () => {
    expect(normalisePath('https://example.com/user/42').pattern).toBe('/user/:id');
    expect(normalisePath('https://example.com/product/100/details').pattern).toBe('/product/:id/details');
  });

  test('normalises UUIDs', () => {
    const uuid = '550e8400-e29b-41d4-a716-446655440000';
    expect(normalisePath(`https://example.com/item/${uuid}`).pattern).toBe('/item/:uuid');
  });

  test('normalises alphanumeric IDs (8-20 chars)', () => {
    expect(normalisePath('https://example.com/session/abc12345de').pattern).toBe('/session/:id');
  });

  test('normalises kebab-case slugs (no digits)', () => {
    expect(normalisePath('https://example.com/post/my-great-article').pattern).toBe('/post/:slug');
  });

  test('keeps API version segments literal', () => {
    expect(normalisePath('https://example.com/api/v1/resource').pattern).toBe('/api/v1/resource');
  });

  test('handles paths without a host', () => {
    const result = normalisePath('/user/99');
    expect(result.pattern).toBe('/user/:id');
  });
});

describe('addRoute', () => {
  test('adds a new route and marks it as new', () => {
    const routes = [];
    const raw = { method: 'GET', url: 'https://example.com/api/users', headers: {}, body: null, capturedAt: new Date().toISOString() };
    const { isNew, schema } = addRoute(raw, routes);
    expect(isNew).toBe(true);
    expect(routes.length).toBe(1);
    expect(schema.method).toBe('GET');
    expect(schema.interceptedCount).toBe(1);
  });

  test('deduplicates identical method+pattern and increments count', () => {
    const routes = [];
    const raw1 = { method: 'GET', url: 'https://example.com/user/1', headers: {}, body: null, capturedAt: '' };
    const raw2 = { method: 'GET', url: 'https://example.com/user/2', headers: {}, body: null, capturedAt: '' };
    addRoute(raw1, routes);
    const { isNew } = addRoute(raw2, routes);
    expect(isNew).toBe(false);
    expect(routes.length).toBe(1);
    expect(routes[0].interceptedCount).toBe(2);
  });

  test('keeps different methods as separate routes', () => {
    const routes = [];
    addRoute({ method: 'GET',  url: 'https://example.com/user/1', headers: {}, body: null, capturedAt: '' }, routes);
    addRoute({ method: 'POST', url: 'https://example.com/user/1', headers: {}, body: null, capturedAt: '' }, routes);
    expect(routes.length).toBe(2);
  });
});

describe('groupRoutes', () => {
  test('groups routes by base path segment', () => {
    const routes = [
      { id: '1', pattern: '/api/users', method: 'GET', interceptedCount: 1 },
      { id: '2', pattern: '/api/products', method: 'GET', interceptedCount: 1 },
      { id: '3', pattern: '/auth/login', method: 'POST', interceptedCount: 1 },
    ];
    const groups = groupRoutes(routes);
    expect(Object.keys(groups)).toEqual(expect.arrayContaining(['api', 'auth']));
    expect(groups['api'].length).toBe(2);
    expect(groups['auth'].length).toBe(1);
  });
});

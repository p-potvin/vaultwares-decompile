// Unit tests for the API request interceptor (api-interceptor.js)

import { interceptRequest } from '../crawler/api-interceptor.js';

function mockRequest({ method = 'GET', url = 'https://api.example.com/users', headers = {}, postData = null } = {}) {
  return {
    method: () => method,
    url: () => url,
    headers: () => headers,
    postData: () => postData,
  };
}

describe('interceptRequest', () => {
  test('captures basic GET request fields', () => {
    const req = mockRequest({ method: 'GET', url: 'https://api.example.com/users' });
    const result = interceptRequest(req);
    expect(result).not.toBeNull();
    expect(result.method).toBe('GET');
    expect(result.url).toBe('https://api.example.com/users');
    expect(result.body).toBeNull();
  });

  test('captures POST request with JSON body', () => {
    const body = JSON.stringify({ email: 'test@example.com', password: 'secret' });
    const req = mockRequest({ method: 'POST', url: 'https://api.example.com/auth/login', postData: body });
    const result = interceptRequest(req);
    expect(result.method).toBe('POST');
    expect(result.body).toEqual({ email: 'test@example.com', password: 'secret' });
  });

  test('redacts Authorization header value', () => {
    const req = mockRequest({
      headers: { Authorization: 'Bearer super-secret-token', 'Content-Type': 'application/json' },
    });
    const result = interceptRequest(req);
    expect(result.headers['Authorization']).toBe('[REDACTED]');
    expect(result.headers['Content-Type']).toBe('application/json');
  });

  test('redacts Cookie header value', () => {
    const req = mockRequest({ headers: { cookie: 'session=abc123' } });
    const result = interceptRequest(req);
    expect(result.headers['cookie']).toBe('[REDACTED]');
  });

  test('redacts x-api-key header value', () => {
    const req = mockRequest({ headers: { 'x-api-key': 'my-key-123' } });
    const result = interceptRequest(req);
    expect(result.headers['x-api-key']).toBe('[REDACTED]');
  });

  test('keeps raw string body when not valid JSON', () => {
    const req = mockRequest({ postData: 'raw=data&format=urlencoded' });
    const result = interceptRequest(req);
    expect(result.body).toBe('raw=data&format=urlencoded');
  });

  test('includes capturedAt timestamp', () => {
    const req = mockRequest();
    const result = interceptRequest(req);
    expect(result.capturedAt).toBeTruthy();
    expect(new Date(result.capturedAt).toString()).not.toBe('Invalid Date');
  });
});

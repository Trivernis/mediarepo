import { HasPropertyPipe } from './has-property.pipe';

describe('HasPropertyPipe', () => {
  it('create an instance', () => {
    const pipe = new HasPropertyPipe();
    expect(pipe).toBeTruthy();
  });
});

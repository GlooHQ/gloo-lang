import { NamedArgsSingleEnumList } from '../baml_client'
import { b } from './test-setup'

describe('Basic Input/Output Tests', () => {
  describe('Input Types', () => {
    it('single bool', async () => {
      const res = await b.TestFnNamedArgsSingleBool(true)
      expect(res).toEqual('true')
    })

    it('single string list', async () => {
      const res = await b.TestFnNamedArgsSingleStringList(['a', 'b', 'c'])
      expect(res).toContain('a')
      expect(res).toContain('b')
      expect(res).toContain('c')
    })

    it('single class', async () => {
      const res = await b.TestFnNamedArgsSingleClass({
        key: 'key',
        key_two: true,
        key_three: 52,
      })
      expect(res).toContain('52')
    })

    it('multiple classes', async () => {
      const res = await b.TestMulticlassNamedArgs(
        {
          key: 'key',
          key_two: true,
          key_three: 52,
        },
        {
          key: 'key',
          key_two: true,
          key_three: 64,
        },
      )
      expect(res).toContain('52')
      expect(res).toContain('64')
    })

    it('single enum list', async () => {
      const res = await b.TestFnNamedArgsSingleEnumList([NamedArgsSingleEnumList.TWO])
      expect(res).toContain('TWO')
    })

    it('single float', async () => {
      const res = await b.TestFnNamedArgsSingleFloat(3.12)
      expect(res).toContain('3.12')
    })

    it('single int', async () => {
      const res = await b.TestFnNamedArgsSingleInt(3566)
      expect(res).toContain('3566')
    })
  })

  describe('Output Types', () => {
    it('should work for all outputs', async () => {
      const input = 'test input'

      const bool = await b.FnOutputBool(input)
      expect(bool).toEqual(true)

      const int = await b.FnOutputInt(input)
      expect(int).toEqual(5)

      const list = await b.FnOutputClassList(input)
      expect(list.length).toBeGreaterThan(0)
      expect(list[0].prop1.length).toBeGreaterThan(0)

      const classWEnum = await b.FnOutputClassWithEnum(input)
      expect(['ONE', 'TWO']).toContain(classWEnum.prop2)

      const classs = await b.FnOutputClass(input)
      expect(classs.prop1).not.toBeNull()
      expect(classs.prop2).toEqual(540)
    })
  })
})

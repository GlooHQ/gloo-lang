import TypeBuilder from '../baml_client/type_builder'
import { b } from './test-setup'

describe('Dynamic Type Tests', () => {
  describe('Basic Dynamic Types', () => {
    it('should work with dynamic types single', async () => {
      let tb = new TypeBuilder()
      tb.Person.addProperty('last_name', tb.string().optional())
      tb.Person.addProperty('height', tb.float().optional()).description('Height in meters')
      tb.Hobby.addValue('CHESS')
      tb.Hobby.listValues().map(([name, v]) => v.alias(name.toLowerCase()))
      tb.Person.addProperty('hobbies', tb.Hobby.type().list().optional()).description(
        'Some suggested hobbies they might be good at',
      )

      const res = await b.ExtractPeople(
        "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop.",
        { tb },
      )
      expect(res.length).toBeGreaterThan(0)
    })

    it('should work with dynamic types enum', async () => {
      let tb = new TypeBuilder()
      const fieldEnum = tb.addEnum('Animal')
      const animals = ['giraffe', 'elephant', 'lion']
      for (const animal of animals) {
        fieldEnum.addValue(animal.toUpperCase())
      }
      tb.Person.addProperty('animalLiked', fieldEnum.type())
      const res = await b.ExtractPeople(
        "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop. I like giraffes.",
        { tb },
      )
      expect(res.length).toBeGreaterThan(0)
      expect(res[0]['animalLiked']).toEqual('GIRAFFE')
    })
  })

  describe('Complex Dynamic Types', () => {
    it('should work with dynamic output map', async () => {
      let tb = new TypeBuilder()
      tb.DynamicOutput.addProperty('hair_color', tb.string())
      tb.DynamicOutput.addProperty('attributes', tb.map(tb.string(), tb.string())).description(
        "Things like 'eye_color' or 'facial_hair'",
      )

      const res = await b.MyFunc(
        "My name is Harrison. My hair is black and I'm 6 feet tall. I have blue eyes and a beard.",
        { tb },
      )

      expect(res.hair_color).toEqual('black')
      expect(res.attributes['eye_color']).toEqual('blue')
      expect(res.attributes['facial_hair']).toEqual('beard')
    })

    it('should work with dynamic output union', async () => {
      let tb = new TypeBuilder()

      const class1 = tb.addClass('Class1')
      class1.addProperty('meters', tb.float())

      const class2 = tb.addClass('Class2')
      class2.addProperty('feet', tb.float())
      class2.addProperty('inches', tb.float().optional())

      tb.DynamicOutput.addProperty('height', tb.union([class1.type(), class2.type()]))

      let res = await b.MyFunc("My name is Harrison. My hair is black and I'm 6 feet tall.", { tb })

      expect(res.height['feet']).toEqual(6)

      res = await b.MyFunc("My name is Harrison. My hair is black and I'm 1.8 meters tall.", { tb })

      expect(res.height['meters']).toEqual(1.8)
    })
  })
})

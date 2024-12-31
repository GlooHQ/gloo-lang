class FormatterTest0 {
  lorem string  // trailing comments should be preserved
  ipsum string
}

class FormatterTest1 {
  lorem string
  ipsum string
  // dolor string
}

class FormatterTest2 {
  // "lorem" is a latin word
  lorem string
  // "ipsum" is a latin word
  ipsum string
}

class FormatterTest3 {
  lorem string
  ipsum string
  // Lorem ipsum dolor sit amet
  // Consectetur adipiscing elit
  // Sed do eiusmod tempor incididunt
  // Ut labore et dolore magna aliqua
  // Ut enim ad minim veniam
}
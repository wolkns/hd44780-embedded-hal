# Changelog hd44780-embedded-hal

## Version History

### **0.1.5**

+ bugfix: pcf8574 read byte, separated read and write transactions
  there was a bug specifically with the use of embassy-stm32 when compiling for release.

### **0.1.4**  

+ made crate more *generic* (create_char)
+ added ping functionality

### **< 0.1.3**

+ First versions

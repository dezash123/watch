struct LCD_Display {
    spi: spi,
}

struct LCD_Touch {
    i2c: i2c,
}

struct LCD {
    backlight: LED,
    display: LCD_Display,
    touch: LCD_Touch,
}



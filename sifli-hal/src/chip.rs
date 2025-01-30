embassy_hal_internal::interrupt_mod!(
    LPTIM1,
    LPTIM2,
    PMUC,
    RTC,
    DMAC1_CH1,
    DMAC1_CH2, 
    DMAC1_CH3,
    DMAC1_CH4,
    DMAC1_CH5,
    DMAC1_CH6,
    DMAC1_CH7,
    DMAC1_CH8,
    MAILBOX2_CH1,
    USART1,
    SPI1,
    I2C1,
    EPIC,
    LCDC1,
    I2S1,
    GPADC,
    EFUSEC,
    AES,
    PTC1,
    TRNG,
    GPTIM1,
    GPTIM2,
    BTIM1,
    BTIM2,
    USART2,
    SPI2,
    I2C2,
    EXTDMA,
    I2C4,
    SDMMC1,
    MAILBOX2_CH2,
    PDM1,
    GPIO1,
    MPI1,
    MPI2,
    EZIP1,
    AUDPRC,
    TSEN,
    USBC,
    I2C3,
    ATIM1,
    USART3,
    AUD_HP,
    SECU1
);

embassy_hal_internal::peripherals! {
    PIN_0, PIN_1, PIN_2, PIN_3, PIN_4, PIN_5, PIN_6, PIN_7, 
    PIN_8, PIN_9, PIN_10, PIN_11, PIN_12, PIN_13, PIN_14, PIN_15,
    PIN_16, PIN_17, PIN_18, PIN_19, PIN_20, PIN_21, PIN_22, PIN_23,
    PIN_24, PIN_25, PIN_26, PIN_27, PIN_28, PIN_29, PIN_30, PIN_31,
    PIN_32, PIN_33, PIN_34, PIN_35, PIN_36, PIN_37, PIN_38, PIN_39,
    PIN_40, PIN_41, PIN_42, PIN_43, PIN_44,

    DMAC1,
    DMAC_CH0, DMAC_CH1, DMAC_CH2, DMAC_CH3, 
    DMAC_CH4, DMAC_CH5, DMAC_CH6, DMAC_CH7,

    HPSYS_RCC, HPSYS_CFG, HPSYS_GPIO, HPSYS_AON, HPSYS_PINMUX,

    USART1, USART2, USART3,
    SPI1, SPI2,
    I2C1, I2C2, I2C3, I2C4,
    SDMMC1,
    MPI1, MPI2,
    
    ATIM1,
    GPTIM1, GPTIM2,
    BTIM1, BTIM2,
    LPTIM1, LPTIM2,

    I2S1,
    PDM1,
    AUDPRC,
    EPIC,
    LCDC1, 
    EZIP1, 
    AES, 
    CRC1, 
    TRNG,
    GPADC,
    AUDCODEC,
    WDT1, 
    IWDT,
    PTC1,
    TSEN,
    EXTDMA, 
    EFUSEC, 
    PMUC
}
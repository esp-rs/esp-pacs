///Register `HCON` reader
pub type R = crate::R<HCON_SPEC>;
///Field `CARD_TYPE` reader - Hardware support SDIO and MMC.
pub type CARD_TYPE_R = crate::BitReader;
///Field `CARD_NUM` reader - Support card number is 2.
pub type CARD_NUM_R = crate::FieldReader;
///Field `BUS_TYPE` reader - Register config is APB bus.
pub type BUS_TYPE_R = crate::BitReader;
///Field `DATA_WIDTH` reader - Regisger data widht is 32.
pub type DATA_WIDTH_R = crate::FieldReader;
///Field `ADDR_WIDTH` reader - Register address width is 32.
pub type ADDR_WIDTH_R = crate::FieldReader;
///Field `DMA_WIDTH` reader - DMA data witdth is 32.
pub type DMA_WIDTH_R = crate::FieldReader;
///Field `RAM_INDISE` reader - Inside RAM in SDMMC module.
pub type RAM_INDISE_R = crate::BitReader;
///Field `HOLD` reader - Have a hold regiser in data path .
pub type HOLD_R = crate::BitReader;
///Field `NUM_CLK_DIV` reader - Have 4 clk divider in design .
pub type NUM_CLK_DIV_R = crate::FieldReader;
impl R {
    ///Bit 0 - Hardware support SDIO and MMC.
    #[inline(always)]
    pub fn card_type(&self) -> CARD_TYPE_R {
        CARD_TYPE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:5 - Support card number is 2.
    #[inline(always)]
    pub fn card_num(&self) -> CARD_NUM_R {
        CARD_NUM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    ///Bit 6 - Register config is APB bus.
    #[inline(always)]
    pub fn bus_type(&self) -> BUS_TYPE_R {
        BUS_TYPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:9 - Regisger data widht is 32.
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:15 - Register address width is 32.
    #[inline(always)]
    pub fn addr_width(&self) -> ADDR_WIDTH_R {
        ADDR_WIDTH_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    ///Bits 18:20 - DMA data witdth is 32.
    #[inline(always)]
    pub fn dma_width(&self) -> DMA_WIDTH_R {
        DMA_WIDTH_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 21 - Inside RAM in SDMMC module.
    #[inline(always)]
    pub fn ram_indise(&self) -> RAM_INDISE_R {
        RAM_INDISE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Have a hold regiser in data path .
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 24:25 - Have 4 clk divider in design .
    #[inline(always)]
    pub fn num_clk_div(&self) -> NUM_CLK_DIV_R {
        NUM_CLK_DIV_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCON")
            .field("card_type", &self.card_type())
            .field("card_num", &self.card_num())
            .field("bus_type", &self.bus_type())
            .field("data_width", &self.data_width())
            .field("addr_width", &self.addr_width())
            .field("dma_width", &self.dma_width())
            .field("ram_indise", &self.ram_indise())
            .field("hold", &self.hold())
            .field("num_clk_div", &self.num_clk_div())
            .finish()
    }
}
/**Hardware feature register

You can [`read`](crate::generic::Reg::read) this register and get [`hcon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HCON_SPEC;
impl crate::RegisterSpec for HCON_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hcon::R`](R) reader structure
impl crate::Readable for HCON_SPEC {}
///`reset()` method sets HCON to value 0x0344_4cc3
impl crate::Resettable for HCON_SPEC {
    const RESET_VALUE: u32 = 0x0344_4cc3;
}

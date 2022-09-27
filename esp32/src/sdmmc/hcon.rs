#[doc = "Register `HCON` reader"]
pub struct R(crate::R<HCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARD_TYPE` reader - Hardware support SDIO and MMC."]
pub type CARD_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `CARD_NUM` reader - Support card number is 2."]
pub type CARD_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUS_TYPE` reader - Register config is APB bus."]
pub type BUS_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `DATA_WIDTH` reader - Regisger data widht is 32."]
pub type DATA_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR_WIDTH` reader - Register address width is 32."]
pub type ADDR_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_WIDTH` reader - DMA data witdth is 32."]
pub type DMA_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAM_INDISE` reader - Inside RAM in SDMMC module."]
pub type RAM_INDISE_R = crate::BitReader<bool>;
#[doc = "Field `HOLD` reader - Have a hold regiser in data path ."]
pub type HOLD_R = crate::BitReader<bool>;
#[doc = "Field `NUM_CLK_DIV` reader - Have 4 clk divider in design ."]
pub type NUM_CLK_DIV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Hardware support SDIO and MMC."]
    #[inline(always)]
    pub fn card_type(&self) -> CARD_TYPE_R {
        CARD_TYPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Support card number is 2."]
    #[inline(always)]
    pub fn card_num(&self) -> CARD_NUM_R {
        CARD_NUM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Register config is APB bus."]
    #[inline(always)]
    pub fn bus_type(&self) -> BUS_TYPE_R {
        BUS_TYPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - Regisger data widht is 32."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:15 - Register address width is 32."]
    #[inline(always)]
    pub fn addr_width(&self) -> ADDR_WIDTH_R {
        ADDR_WIDTH_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 18:20 - DMA data witdth is 32."]
    #[inline(always)]
    pub fn dma_width(&self) -> DMA_WIDTH_R {
        DMA_WIDTH_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Inside RAM in SDMMC module."]
    #[inline(always)]
    pub fn ram_indise(&self) -> RAM_INDISE_R {
        RAM_INDISE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Have a hold regiser in data path ."]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Have 4 clk divider in design ."]
    #[inline(always)]
    pub fn num_clk_div(&self) -> NUM_CLK_DIV_R {
        NUM_CLK_DIV_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[doc = "Hardware feature register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcon](index.html) module"]
pub struct HCON_SPEC;
impl crate::RegisterSpec for HCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcon::R](R) reader structure"]
impl crate::Readable for HCON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCON to value 0x0344_4cc3"]
impl crate::Resettable for HCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0344_4cc3
    }
}

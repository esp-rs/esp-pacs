#[doc = "Register `SDIO_PAD_BIST_ST` reader"]
pub type R = crate::R<SDIO_PAD_BIST_ST_SPEC>;
#[doc = "Field `SDIO_PAD_BIST_RCNT` reader - Indicate number of clock cycles received for pad bist debug"]
pub type SDIO_PAD_BIST_RCNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Indicate number of clock cycles received for pad bist debug"]
    #[inline(always)]
    pub fn sdio_pad_bist_rcnt(&self) -> SDIO_PAD_BIST_RCNT_R {
        SDIO_PAD_BIST_RCNT_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_PAD_BIST_ST")
            .field("sdio_pad_bist_rcnt", &self.sdio_pad_bist_rcnt())
            .finish()
    }
}
#[doc = "sdio pad bist status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_pad_bist_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_PAD_BIST_ST_SPEC;
impl crate::RegisterSpec for SDIO_PAD_BIST_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_pad_bist_st::R`](R) reader structure"]
impl crate::Readable for SDIO_PAD_BIST_ST_SPEC {}
#[doc = "`reset()` method sets SDIO_PAD_BIST_ST to value 0"]
impl crate::Resettable for SDIO_PAD_BIST_ST_SPEC {}

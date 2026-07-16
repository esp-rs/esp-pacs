#[doc = "Register `SDIO_PAD_BIST_INT_ST` reader"]
pub type R = crate::R<SDIO_PAD_BIST_INT_ST_SPEC>;
#[doc = "Field `SDIO_PAD_BIST_OK_INT_ST` reader - the masked interrupt status of sdio pad bist ok interrupt"]
pub type SDIO_PAD_BIST_OK_INT_ST_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_BIST_FAIL_INT_ST` reader - the masked interrupt status of sdio pad bist fail interrupt"]
pub type SDIO_PAD_BIST_FAIL_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the masked interrupt status of sdio pad bist ok interrupt"]
    #[inline(always)]
    pub fn sdio_pad_bist_ok_int_st(&self) -> SDIO_PAD_BIST_OK_INT_ST_R {
        SDIO_PAD_BIST_OK_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the masked interrupt status of sdio pad bist fail interrupt"]
    #[inline(always)]
    pub fn sdio_pad_bist_fail_int_st(&self) -> SDIO_PAD_BIST_FAIL_INT_ST_R {
        SDIO_PAD_BIST_FAIL_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_PAD_BIST_INT_ST")
            .field("sdio_pad_bist_ok_int_st", &self.sdio_pad_bist_ok_int_st())
            .field(
                "sdio_pad_bist_fail_int_st",
                &self.sdio_pad_bist_fail_int_st(),
            )
            .finish()
    }
}
#[doc = "masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_pad_bist_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_PAD_BIST_INT_ST_SPEC;
impl crate::RegisterSpec for SDIO_PAD_BIST_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_pad_bist_int_st::R`](R) reader structure"]
impl crate::Readable for SDIO_PAD_BIST_INT_ST_SPEC {}
#[doc = "`reset()` method sets SDIO_PAD_BIST_INT_ST to value 0"]
impl crate::Resettable for SDIO_PAD_BIST_INT_ST_SPEC {}

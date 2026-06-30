#[doc = "Register `GMAC0_PAD_BIST_ST` reader"]
pub type R = crate::R<GMAC0_PAD_BIST_ST_SPEC>;
#[doc = "Field `GMAC0_PAD_BIST_RCNT` reader - Indicate number of clock cycles received for pad bist debug"]
pub type GMAC0_PAD_BIST_RCNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Indicate number of clock cycles received for pad bist debug"]
    #[inline(always)]
    pub fn gmac0_pad_bist_rcnt(&self) -> GMAC0_PAD_BIST_RCNT_R {
        GMAC0_PAD_BIST_RCNT_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GMAC0_PAD_BIST_ST")
            .field("gmac0_pad_bist_rcnt", &self.gmac0_pad_bist_rcnt())
            .finish()
    }
}
#[doc = "gmac0 pad bist status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac0_pad_bist_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC0_PAD_BIST_ST_SPEC;
impl crate::RegisterSpec for GMAC0_PAD_BIST_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac0_pad_bist_st::R`](R) reader structure"]
impl crate::Readable for GMAC0_PAD_BIST_ST_SPEC {}
#[doc = "`reset()` method sets GMAC0_PAD_BIST_ST to value 0"]
impl crate::Resettable for GMAC0_PAD_BIST_ST_SPEC {}

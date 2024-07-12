#[doc = "Register `WRESP_CNT` reader"]
pub type R = crate::R<WRESP_CNT_SPEC>;
#[doc = "Field `WRESP_CNT` reader - axi wr responce cnt reg."]
pub type WRESP_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - axi wr responce cnt reg."]
    #[inline(always)]
    pub fn wresp_cnt(&self) -> WRESP_CNT_R {
        WRESP_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRESP_CNT")
            .field("wresp_cnt", &self.wresp_cnt())
            .finish()
    }
}
#[doc = "AXI wr responce cnt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`wresp_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRESP_CNT_SPEC;
impl crate::RegisterSpec for WRESP_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wresp_cnt::R`](R) reader structure"]
impl crate::Readable for WRESP_CNT_SPEC {}
#[doc = "`reset()` method sets WRESP_CNT to value 0"]
impl crate::Resettable for WRESP_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}

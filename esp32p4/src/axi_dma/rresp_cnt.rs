#[doc = "Register `RRESP_CNT` reader"]
pub type R = crate::R<RRESP_CNT_SPEC>;
#[doc = "Field `RRESP_CNT` reader - axi rd responce cnt reg."]
pub type RRESP_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - axi rd responce cnt reg."]
    #[inline(always)]
    pub fn rresp_cnt(&self) -> RRESP_CNT_R {
        RRESP_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RRESP_CNT")
            .field("rresp_cnt", &self.rresp_cnt())
            .finish()
    }
}
#[doc = "AXI wr responce cnt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rresp_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RRESP_CNT_SPEC;
impl crate::RegisterSpec for RRESP_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rresp_cnt::R`](R) reader structure"]
impl crate::Readable for RRESP_CNT_SPEC {}
#[doc = "`reset()` method sets RRESP_CNT to value 0"]
impl crate::Resettable for RRESP_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}

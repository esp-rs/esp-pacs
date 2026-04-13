#[doc = "Register `IN_DONE_DES_ADDR_CH%s` reader"]
pub type R = crate::R<IN_DONE_DES_ADDR_CH_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_DONE_DES_ADDR_CH")
            .field("val", &self.val())
            .finish()
    }
}
#[doc = "IN link EOF address for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_done_des_addr_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_DONE_DES_ADDR_CH_SPEC;
impl crate::RegisterSpec for IN_DONE_DES_ADDR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_done_des_addr_ch::R`](R) reader structure"]
impl crate::Readable for IN_DONE_DES_ADDR_CH_SPEC {}
#[doc = "`reset()` method sets IN_DONE_DES_ADDR_CH%s to value 0"]
impl crate::Resettable for IN_DONE_DES_ADDR_CH_SPEC {}

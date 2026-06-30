#[doc = "Register `RD_REPEAT_DATA7` reader"]
pub type R = crate::R<RD_REPEAT_DATA7_SPEC>;
#[doc = "Field `REPEAT7_RSVD` reader - Reserved."]
pub type REPEAT7_RSVD_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved."]
    #[inline(always)]
    pub fn repeat7_rsvd(&self) -> REPEAT7_RSVD_R {
        REPEAT7_RSVD_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA7")
            .field("repeat7_rsvd", &self.repeat7_rsvd())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA7_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data7::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA7_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA7 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA7_SPEC {}

#[doc = "Register `RD_REPEAT_DATA_ERR7` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR7_SPEC>;
#[doc = "Field `REPEAT7_RSVD_ERR` reader - Represents the programming error of EFUSE_REPEAT7_RSVD"]
pub type REPEAT7_RSVD_ERR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Represents the programming error of EFUSE_REPEAT7_RSVD"]
    #[inline(always)]
    pub fn repeat7_rsvd_err(&self) -> REPEAT7_RSVD_ERR_R {
        REPEAT7_RSVD_ERR_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR7")
            .field("repeat7_rsvd_err", &self.repeat7_rsvd_err())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR7_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err7::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR7_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR7 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR7_SPEC {}

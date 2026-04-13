#[doc = "Register `CROP_ERR_ST` reader"]
pub type R = crate::R<CROP_ERR_ST_SPEC>;
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
        f.debug_struct("CROP_ERR_ST")
            .field("val", &self.val())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crop_err_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CROP_ERR_ST_SPEC;
impl crate::RegisterSpec for CROP_ERR_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crop_err_st::R`](R) reader structure"]
impl crate::Readable for CROP_ERR_ST_SPEC {}
#[doc = "`reset()` method sets CROP_ERR_ST to value 0"]
impl crate::Resettable for CROP_ERR_ST_SPEC {}

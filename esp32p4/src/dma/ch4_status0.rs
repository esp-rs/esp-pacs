#[doc = "Register `CH4_STATUS0` reader"]
pub type R = crate::R<CH4_STATUS0_SPEC>;
#[doc = "Field `CH4_CMPLTD_BLK_TFR_SIZE` reader - NA"]
pub type CH4_CMPLTD_BLK_TFR_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - NA"]
    #[inline(always)]
    pub fn ch4_cmpltd_blk_tfr_size(&self) -> CH4_CMPLTD_BLK_TFR_SIZE_R {
        CH4_CMPLTD_BLK_TFR_SIZE_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH4_STATUS0")
            .field(
                "ch4_cmpltd_blk_tfr_size",
                &format_args!("{}", self.ch4_cmpltd_blk_tfr_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH4_STATUS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_status0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_STATUS0_SPEC;
impl crate::RegisterSpec for CH4_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_status0::R`](R) reader structure"]
impl crate::Readable for CH4_STATUS0_SPEC {}
#[doc = "`reset()` method sets CH4_STATUS0 to value 0"]
impl crate::Resettable for CH4_STATUS0_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `INLINK_DSCR_BF1` reader"]
pub type R = crate::R<INLINK_DSCR_BF1_SPEC>;
#[doc = "Field `INLINK_DSCR_BF1` reader - "]
pub type INLINK_DSCR_BF1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inlink_dscr_bf1(&self) -> INLINK_DSCR_BF1_R {
        INLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INLINK_DSCR_BF1")
            .field("inlink_dscr_bf1", &self.inlink_dscr_bf1())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr_bf1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INLINK_DSCR_BF1_SPEC;
impl crate::RegisterSpec for INLINK_DSCR_BF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inlink_dscr_bf1::R`](R) reader structure"]
impl crate::Readable for INLINK_DSCR_BF1_SPEC {}
#[doc = "`reset()` method sets INLINK_DSCR_BF1 to value 0"]
impl crate::Resettable for INLINK_DSCR_BF1_SPEC {
    const RESET_VALUE: u32 = 0;
}

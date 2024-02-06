#[doc = "Register `OUTLINK_DSCR_BF1` reader"]
pub type R = crate::R<OUTLINK_DSCR_BF1_SPEC>;
#[doc = "Field `OUTLINK_DSCR_BF1` reader - The address of next outlink data buffer."]
pub type OUTLINK_DSCR_BF1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of next outlink data buffer."]
    #[inline(always)]
    pub fn outlink_dscr_bf1(&self) -> OUTLINK_DSCR_BF1_R {
        OUTLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTLINK_DSCR_BF1")
            .field(
                "outlink_dscr_bf1",
                &format_args!("{}", self.outlink_dscr_bf1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUTLINK_DSCR_BF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Address of next outlink data buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr_bf1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTLINK_DSCR_BF1_SPEC;
impl crate::RegisterSpec for OUTLINK_DSCR_BF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outlink_dscr_bf1::R`](R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_BF1_SPEC {}
#[doc = "`reset()` method sets OUTLINK_DSCR_BF1 to value 0"]
impl crate::Resettable for OUTLINK_DSCR_BF1_SPEC {
    const RESET_VALUE: u32 = 0;
}

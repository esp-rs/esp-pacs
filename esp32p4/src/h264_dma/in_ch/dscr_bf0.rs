#[doc = "Register `DSCR_BF0` reader"]
pub type R = crate::R<DSCR_BF0_SPEC>;
#[doc = "Field `INLINK_DSCR_BF0` reader - The address of the last inlink descriptor's next address x-1."]
pub type INLINK_DSCR_BF0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the last inlink descriptor's next address x-1."]
    #[inline(always)]
    pub fn inlink_dscr_bf0(&self) -> INLINK_DSCR_BF0_R {
        INLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSCR_BF0")
            .field("inlink_dscr_bf0", &self.inlink_dscr_bf0())
            .finish()
    }
}
#[doc = "RX CHx last dscr addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr_bf0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSCR_BF0_SPEC;
impl crate::RegisterSpec for DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr_bf0::R`](R) reader structure"]
impl crate::Readable for DSCR_BF0_SPEC {}
#[doc = "`reset()` method sets DSCR_BF0 to value 0"]
impl crate::Resettable for DSCR_BF0_SPEC {}

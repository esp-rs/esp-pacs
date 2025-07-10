#[doc = "Register `IN_DSCR_BF0` reader"]
pub type R = crate::R<IN_DSCR_BF0_SPEC>;
#[doc = "Field `INLINK_DSCR_BF0_CH0` reader - Represents the address of the current receive descriptor x that has already been fetched."]
pub type INLINK_DSCR_BF0_CH0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the current receive descriptor x that has already been fetched."]
    #[inline(always)]
    pub fn inlink_dscr_bf0_ch0(&self) -> INLINK_DSCR_BF0_CH0_R {
        INLINK_DSCR_BF0_CH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_DSCR_BF0")
            .field("inlink_dscr_bf0_ch0", &self.inlink_dscr_bf0_ch0())
            .finish()
    }
}
#[doc = "The last receive descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_DSCR_BF0_SPEC;
impl crate::RegisterSpec for IN_DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_dscr_bf0::R`](R) reader structure"]
impl crate::Readable for IN_DSCR_BF0_SPEC {}
#[doc = "`reset()` method sets IN_DSCR_BF0 to value 0"]
impl crate::Resettable for IN_DSCR_BF0_SPEC {}

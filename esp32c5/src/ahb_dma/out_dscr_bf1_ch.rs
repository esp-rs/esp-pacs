#[doc = "Register `OUT_DSCR_BF1_CH%s` reader"]
pub type R = crate::R<OUT_DSCR_BF1_CH_SPEC>;
#[doc = "Field `OUTLINK_DSCR_BF1` reader - Represents the address of the previous transmit descriptor y-1 that has already been fetched."]
pub type OUTLINK_DSCR_BF1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the previous transmit descriptor y-1 that has already been fetched."]
    #[inline(always)]
    pub fn outlink_dscr_bf1(&self) -> OUTLINK_DSCR_BF1_R {
        OUTLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_DSCR_BF1_CH")
            .field("outlink_dscr_bf1", &self.outlink_dscr_bf1())
            .finish()
    }
}
#[doc = "The second-to-last transmit descriptor address of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf1_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_DSCR_BF1_CH_SPEC;
impl crate::RegisterSpec for OUT_DSCR_BF1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_dscr_bf1_ch::R`](R) reader structure"]
impl crate::Readable for OUT_DSCR_BF1_CH_SPEC {}
#[doc = "`reset()` method sets OUT_DSCR_BF1_CH%s to value 0"]
impl crate::Resettable for OUT_DSCR_BF1_CH_SPEC {}

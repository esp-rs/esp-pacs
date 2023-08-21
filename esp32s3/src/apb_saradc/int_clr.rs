#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `THRES1_LOW_INT_CLR` writer - interrupt of thres1 low"]
pub type THRES1_LOW_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THRES0_LOW_INT_CLR` writer - interrupt of thres0 low"]
pub type THRES0_LOW_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THRES1_HIGH_INT_CLR` writer - interrupt of thres1 high"]
pub type THRES1_HIGH_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THRES0_HIGH_INT_CLR` writer - interrupt of thres0 high"]
pub type THRES0_HIGH_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_SARADC2_DONE_INT_CLR` writer - interrupt of sar2 done"]
pub type APB_SARADC2_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_SARADC1_DONE_INT_CLR` writer - interrupt of sar1 done"]
pub type APB_SARADC1_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 26 - interrupt of thres1 low"]
    #[inline(always)]
    #[must_use]
    pub fn thres1_low_int_clr(&mut self) -> THRES1_LOW_INT_CLR_W<INT_CLR_SPEC, 26> {
        THRES1_LOW_INT_CLR_W::new(self)
    }
    #[doc = "Bit 27 - interrupt of thres0 low"]
    #[inline(always)]
    #[must_use]
    pub fn thres0_low_int_clr(&mut self) -> THRES0_LOW_INT_CLR_W<INT_CLR_SPEC, 27> {
        THRES0_LOW_INT_CLR_W::new(self)
    }
    #[doc = "Bit 28 - interrupt of thres1 high"]
    #[inline(always)]
    #[must_use]
    pub fn thres1_high_int_clr(&mut self) -> THRES1_HIGH_INT_CLR_W<INT_CLR_SPEC, 28> {
        THRES1_HIGH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29 - interrupt of thres0 high"]
    #[inline(always)]
    #[must_use]
    pub fn thres0_high_int_clr(&mut self) -> THRES0_HIGH_INT_CLR_W<INT_CLR_SPEC, 29> {
        THRES0_HIGH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 30 - interrupt of sar2 done"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc2_done_int_clr(&mut self) -> APB_SARADC2_DONE_INT_CLR_W<INT_CLR_SPEC, 30> {
        APB_SARADC2_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 31 - interrupt of sar1 done"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc1_done_int_clr(&mut self) -> APB_SARADC1_DONE_INT_CLR_W<INT_CLR_SPEC, 31> {
        APB_SARADC1_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "clear interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

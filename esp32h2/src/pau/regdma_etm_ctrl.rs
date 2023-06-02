#[doc = "Register `REGDMA_ETM_CTRL` writer"]
pub struct W(crate::W<REGDMA_ETM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGDMA_ETM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<REGDMA_ETM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGDMA_ETM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETM_START_0` writer - etm_start_0 reg"]
pub type ETM_START_0_W<'a, const O: u8> = crate::BitWriter<'a, REGDMA_ETM_CTRL_SPEC, O>;
#[doc = "Field `ETM_START_1` writer - etm_start_1 reg"]
pub type ETM_START_1_W<'a, const O: u8> = crate::BitWriter<'a, REGDMA_ETM_CTRL_SPEC, O>;
#[doc = "Field `ETM_START_2` writer - etm_start_2 reg"]
pub type ETM_START_2_W<'a, const O: u8> = crate::BitWriter<'a, REGDMA_ETM_CTRL_SPEC, O>;
#[doc = "Field `ETM_START_3` writer - etm_start_3 reg"]
pub type ETM_START_3_W<'a, const O: u8> = crate::BitWriter<'a, REGDMA_ETM_CTRL_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_ETM_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - etm_start_0 reg"]
    #[inline(always)]
    #[must_use]
    pub fn etm_start_0(&mut self) -> ETM_START_0_W<0> {
        ETM_START_0_W::new(self)
    }
    #[doc = "Bit 1 - etm_start_1 reg"]
    #[inline(always)]
    #[must_use]
    pub fn etm_start_1(&mut self) -> ETM_START_1_W<1> {
        ETM_START_1_W::new(self)
    }
    #[doc = "Bit 2 - etm_start_2 reg"]
    #[inline(always)]
    #[must_use]
    pub fn etm_start_2(&mut self) -> ETM_START_2_W<2> {
        ETM_START_2_W::new(self)
    }
    #[doc = "Bit 3 - etm_start_3 reg"]
    #[inline(always)]
    #[must_use]
    pub fn etm_start_3(&mut self) -> ETM_START_3_W<3> {
        ETM_START_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM start ctrl reg\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regdma_etm_ctrl](index.html) module"]
pub struct REGDMA_ETM_CTRL_SPEC;
impl crate::RegisterSpec for REGDMA_ETM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [regdma_etm_ctrl::W](W) writer structure"]
impl crate::Writable for REGDMA_ETM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGDMA_ETM_CTRL to value 0"]
impl crate::Resettable for REGDMA_ETM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

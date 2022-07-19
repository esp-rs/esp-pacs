#[doc = "Register `EDMA_CTRL` reader"]
pub struct R(crate::R<EDMA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDMA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDMA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDMA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDMA_CTRL` writer"]
pub struct W(crate::W<EDMA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDMA_CTRL_SPEC>;
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
impl From<crate::W<EDMA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDMA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDMA_CLK_ON` reader - Set 1 to enable EDMA clock."]
pub type EDMA_CLK_ON_R = crate::BitReader<bool>;
#[doc = "Field `EDMA_CLK_ON` writer - Set 1 to enable EDMA clock."]
pub type EDMA_CLK_ON_W<'a> = crate::BitWriter<'a, u32, EDMA_CTRL_SPEC, bool, 0>;
#[doc = "Field `EDMA_RESET` reader - Set 1 to let EDMA reset"]
pub type EDMA_RESET_R = crate::BitReader<bool>;
#[doc = "Field `EDMA_RESET` writer - Set 1 to let EDMA reset"]
pub type EDMA_RESET_W<'a> = crate::BitWriter<'a, u32, EDMA_CTRL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable EDMA clock."]
    #[inline(always)]
    pub fn edma_clk_on(&self) -> EDMA_CLK_ON_R {
        EDMA_CLK_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to let EDMA reset"]
    #[inline(always)]
    pub fn edma_reset(&self) -> EDMA_RESET_R {
        EDMA_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable EDMA clock."]
    #[inline(always)]
    pub fn edma_clk_on(&mut self) -> EDMA_CLK_ON_W {
        EDMA_CLK_ON_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 to let EDMA reset"]
    #[inline(always)]
    pub fn edma_reset(&mut self) -> EDMA_RESET_W {
        EDMA_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "edma clcok and reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edma_ctrl](index.html) module"]
pub struct EDMA_CTRL_SPEC;
impl crate::RegisterSpec for EDMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edma_ctrl::R](R) reader structure"]
impl crate::Readable for EDMA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edma_ctrl::W](W) writer structure"]
impl crate::Writable for EDMA_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EDMA_CTRL to value 0x01"]
impl crate::Resettable for EDMA_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}

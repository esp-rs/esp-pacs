#[doc = "Register `CLOCK_GATE` reader"]
pub struct R(crate::R<CLOCK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_GATE` writer"]
pub struct W(crate::W<CLOCK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_GATE_SPEC>;
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
impl From<crate::W<CLOCK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - clock gate enable."]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - clock gate enable."]
pub type CLK_EN_W<'a> = crate::BitWriter<'a, u32, CLOCK_GATE_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - clock gate enable."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clock gate enable."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_gate](index.html) module"]
pub struct CLOCK_GATE_SPEC;
impl crate::RegisterSpec for CLOCK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_gate::R](R) reader structure"]
impl crate::Readable for CLOCK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_gate::W](W) writer structure"]
impl crate::Writable for CLOCK_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK_GATE to value 0x01"]
impl crate::Resettable for CLOCK_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}

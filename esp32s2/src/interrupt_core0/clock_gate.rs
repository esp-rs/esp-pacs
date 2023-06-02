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
#[doc = "Field `CLK_EN` reader - This bit is used to enable or disable the clock of interrupt matrix. 1: enable the clock. 0: disable the clock."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - This bit is used to enable or disable the clock of interrupt matrix. 1: enable the clock. 0: disable the clock."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLOCK_GATE_SPEC, O>;
#[doc = "Field `PRO_NMI_MASK_HW` reader - This bit is used to disable all NMI interrupt signals to CPU."]
pub type PRO_NMI_MASK_HW_R = crate::BitReader;
#[doc = "Field `PRO_NMI_MASK_HW` writer - This bit is used to disable all NMI interrupt signals to CPU."]
pub type PRO_NMI_MASK_HW_W<'a, const O: u8> = crate::BitWriter<'a, CLOCK_GATE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to enable or disable the clock of interrupt matrix. 1: enable the clock. 0: disable the clock."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is used to disable all NMI interrupt signals to CPU."]
    #[inline(always)]
    pub fn pro_nmi_mask_hw(&self) -> PRO_NMI_MASK_HW_R {
        PRO_NMI_MASK_HW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK_GATE")
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field(
                "pro_nmi_mask_hw",
                &format_args!("{}", self.pro_nmi_mask_hw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLOCK_GATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable or disable the clock of interrupt matrix. 1: enable the clock. 0: disable the clock."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - This bit is used to disable all NMI interrupt signals to CPU."]
    #[inline(always)]
    #[must_use]
    pub fn pro_nmi_mask_hw(&mut self) -> PRO_NMI_MASK_HW_W<1> {
        PRO_NMI_MASK_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI interrupt signals mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_gate](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK_GATE to value 0x01"]
impl crate::Resettable for CLOCK_GATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

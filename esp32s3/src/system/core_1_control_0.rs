#[doc = "Register `CORE_1_CONTROL_0` reader"]
pub struct R(crate::R<CORE_1_CONTROL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_CONTROL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_CONTROL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_CONTROL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_CONTROL_0` writer"]
pub struct W(crate::W<CORE_1_CONTROL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_CONTROL_0_SPEC>;
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
impl From<crate::W<CORE_1_CONTROL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_CONTROL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTROL_CORE_1_RUNSTALL` reader - Set 1 to stall core1"]
pub type CONTROL_CORE_1_RUNSTALL_R = crate::BitReader;
#[doc = "Field `CONTROL_CORE_1_RUNSTALL` writer - Set 1 to stall core1"]
pub type CONTROL_CORE_1_RUNSTALL_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_CONTROL_0_SPEC, O>;
#[doc = "Field `CONTROL_CORE_1_CLKGATE_EN` reader - Set 1 to open core1 clock"]
pub type CONTROL_CORE_1_CLKGATE_EN_R = crate::BitReader;
#[doc = "Field `CONTROL_CORE_1_CLKGATE_EN` writer - Set 1 to open core1 clock"]
pub type CONTROL_CORE_1_CLKGATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_CONTROL_0_SPEC, O>;
#[doc = "Field `CONTROL_CORE_1_RESETING` reader - Set 1 to let core1 reset"]
pub type CONTROL_CORE_1_RESETING_R = crate::BitReader;
#[doc = "Field `CONTROL_CORE_1_RESETING` writer - Set 1 to let core1 reset"]
pub type CONTROL_CORE_1_RESETING_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_CONTROL_0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to stall core1"]
    #[inline(always)]
    pub fn control_core_1_runstall(&self) -> CONTROL_CORE_1_RUNSTALL_R {
        CONTROL_CORE_1_RUNSTALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to open core1 clock"]
    #[inline(always)]
    pub fn control_core_1_clkgate_en(&self) -> CONTROL_CORE_1_CLKGATE_EN_R {
        CONTROL_CORE_1_CLKGATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to let core1 reset"]
    #[inline(always)]
    pub fn control_core_1_reseting(&self) -> CONTROL_CORE_1_RESETING_R {
        CONTROL_CORE_1_RESETING_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_CONTROL_0")
            .field(
                "control_core_1_runstall",
                &format_args!("{}", self.control_core_1_runstall().bit()),
            )
            .field(
                "control_core_1_clkgate_en",
                &format_args!("{}", self.control_core_1_clkgate_en().bit()),
            )
            .field(
                "control_core_1_reseting",
                &format_args!("{}", self.control_core_1_reseting().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_CONTROL_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to stall core1"]
    #[inline(always)]
    #[must_use]
    pub fn control_core_1_runstall(&mut self) -> CONTROL_CORE_1_RUNSTALL_W<0> {
        CONTROL_CORE_1_RUNSTALL_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 to open core1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn control_core_1_clkgate_en(&mut self) -> CONTROL_CORE_1_CLKGATE_EN_W<1> {
        CONTROL_CORE_1_CLKGATE_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to let core1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn control_core_1_reseting(&mut self) -> CONTROL_CORE_1_RESETING_W<2> {
        CONTROL_CORE_1_RESETING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 control regiter 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_control_0](index.html) module"]
pub struct CORE_1_CONTROL_0_SPEC;
impl crate::RegisterSpec for CORE_1_CONTROL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_control_0::R](R) reader structure"]
impl crate::Readable for CORE_1_CONTROL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_control_0::W](W) writer structure"]
impl crate::Writable for CORE_1_CONTROL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_CONTROL_0 to value 0x04"]
impl crate::Resettable for CORE_1_CONTROL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}

#[doc = "Register `SRAM_PD_CTRL_0` reader"]
pub struct R(crate::R<SRAM_PD_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_PD_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_PD_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_PD_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_PD_CTRL_0` writer"]
pub struct W(crate::W<SRAM_PD_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_PD_CTRL_0_SPEC>;
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
impl From<crate::W<SRAM_PD_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_PD_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_PD_0` reader - "]
pub type SRAM_PD_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRAM_PD_0` writer - "]
pub type SRAM_PD_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, SRAM_PD_CTRL_0_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sram_pd_0(&self) -> SRAM_PD_0_R {
        SRAM_PD_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_PD_CTRL_0")
            .field("sram_pd_0", &format_args!("{}", self.sram_pd_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_PD_CTRL_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sram_pd_0(&mut self) -> SRAM_PD_0_W<0> {
        SRAM_PD_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_pd_ctrl_0](index.html) module"]
pub struct SRAM_PD_CTRL_0_SPEC;
impl crate::RegisterSpec for SRAM_PD_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_pd_ctrl_0::R](R) reader structure"]
impl crate::Readable for SRAM_PD_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_pd_ctrl_0::W](W) writer structure"]
impl crate::Writable for SRAM_PD_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_PD_CTRL_0 to value 0"]
impl crate::Resettable for SRAM_PD_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

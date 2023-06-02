#[doc = "Register `SRAM_FO_CTRL_1` reader"]
pub struct R(crate::R<SRAM_FO_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_FO_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_FO_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_FO_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_FO_CTRL_1` writer"]
pub struct W(crate::W<SRAM_FO_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_FO_CTRL_1_SPEC>;
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
impl From<crate::W<SRAM_FO_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_FO_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_FO_1` reader - "]
pub type SRAM_FO_1_R = crate::BitReader;
#[doc = "Field `SRAM_FO_1` writer - "]
pub type SRAM_FO_1_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_FO_CTRL_1_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sram_fo_1(&self) -> SRAM_FO_1_R {
        SRAM_FO_1_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_FO_CTRL_1")
            .field("sram_fo_1", &format_args!("{}", self.sram_fo_1().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_FO_CTRL_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sram_fo_1(&mut self) -> SRAM_FO_1_W<0> {
        SRAM_FO_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_fo_ctrl_1](index.html) module"]
pub struct SRAM_FO_CTRL_1_SPEC;
impl crate::RegisterSpec for SRAM_FO_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_fo_ctrl_1::R](R) reader structure"]
impl crate::Readable for SRAM_FO_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_fo_ctrl_1::W](W) writer structure"]
impl crate::Writable for SRAM_FO_CTRL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_FO_CTRL_1 to value 0x01"]
impl crate::Resettable for SRAM_FO_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

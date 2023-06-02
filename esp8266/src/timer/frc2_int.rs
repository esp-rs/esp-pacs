#[doc = "Register `FRC2_INT` reader"]
pub struct R(crate::R<FRC2_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRC2_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRC2_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRC2_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRC2_INT` writer"]
pub struct W(crate::W<FRC2_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRC2_INT_SPEC>;
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
impl From<crate::W<FRC2_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRC2_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frc2_int_clr_mask` reader - write to clear the status of the interrupt, if theinterrupt type is \"level\""]
pub type FRC2_INT_CLR_MASK_R = crate::BitReader;
#[doc = "Field `frc2_int_clr_mask` writer - write to clear the status of the interrupt, if theinterrupt type is \"level\""]
pub type FRC2_INT_CLR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, FRC2_INT_SPEC, O>;
impl R {
    #[doc = "Bit 0 - write to clear the status of the interrupt, if theinterrupt type is \"level\""]
    #[inline(always)]
    pub fn frc2_int_clr_mask(&self) -> FRC2_INT_CLR_MASK_R {
        FRC2_INT_CLR_MASK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRC2_INT")
            .field(
                "frc2_int_clr_mask",
                &format_args!("{}", self.frc2_int_clr_mask().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRC2_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - write to clear the status of the interrupt, if theinterrupt type is \"level\""]
    #[inline(always)]
    #[must_use]
    pub fn frc2_int_clr_mask(&mut self) -> FRC2_INT_CLR_MASK_W<0> {
        FRC2_INT_CLR_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRC2_INT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frc2_int](index.html) module"]
pub struct FRC2_INT_SPEC;
impl crate::RegisterSpec for FRC2_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frc2_int::R](R) reader structure"]
impl crate::Readable for FRC2_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frc2_int::W](W) writer structure"]
impl crate::Writable for FRC2_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRC2_INT to value 0"]
impl crate::Resettable for FRC2_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

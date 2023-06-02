#[doc = "Register `FRC2_LOAD` reader"]
pub struct R(crate::R<FRC2_LOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRC2_LOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRC2_LOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRC2_LOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRC2_LOAD` writer"]
pub struct W(crate::W<FRC2_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRC2_LOAD_SPEC>;
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
impl From<crate::W<FRC2_LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRC2_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frc2_load_value` reader - the load value into the counter"]
pub type FRC2_LOAD_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `frc2_load_value` writer - the load value into the counter"]
pub type FRC2_LOAD_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, FRC2_LOAD_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - the load value into the counter"]
    #[inline(always)]
    pub fn frc2_load_value(&self) -> FRC2_LOAD_VALUE_R {
        FRC2_LOAD_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRC2_LOAD")
            .field(
                "frc2_load_value",
                &format_args!("{}", self.frc2_load_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRC2_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - the load value into the counter"]
    #[inline(always)]
    #[must_use]
    pub fn frc2_load_value(&mut self) -> FRC2_LOAD_VALUE_W<0> {
        FRC2_LOAD_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the load value into the counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frc2_load](index.html) module"]
pub struct FRC2_LOAD_SPEC;
impl crate::RegisterSpec for FRC2_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frc2_load::R](R) reader structure"]
impl crate::Readable for FRC2_LOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frc2_load::W](W) writer structure"]
impl crate::Writable for FRC2_LOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRC2_LOAD to value 0"]
impl crate::Resettable for FRC2_LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

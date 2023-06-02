#[doc = "Register `DT2_FED_CFG` reader"]
pub struct R(crate::R<DT2_FED_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT2_FED_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT2_FED_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT2_FED_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT2_FED_CFG` writer"]
pub struct W(crate::W<DT2_FED_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT2_FED_CFG_SPEC>;
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
impl From<crate::W<DT2_FED_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT2_FED_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT2_FED` reader - "]
pub type DT2_FED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DT2_FED` writer - "]
pub type DT2_FED_W<'a, const O: u8> = crate::FieldWriter<'a, DT2_FED_CFG_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dt2_fed(&self) -> DT2_FED_R {
        DT2_FED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT2_FED_CFG")
            .field("dt2_fed", &format_args!("{}", self.dt2_fed().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DT2_FED_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn dt2_fed(&mut self) -> DT2_FED_W<0> {
        DT2_FED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt2_fed_cfg](index.html) module"]
pub struct DT2_FED_CFG_SPEC;
impl crate::RegisterSpec for DT2_FED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dt2_fed_cfg::R](R) reader structure"]
impl crate::Readable for DT2_FED_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt2_fed_cfg::W](W) writer structure"]
impl crate::Writable for DT2_FED_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT2_FED_CFG to value 0"]
impl crate::Resettable for DT2_FED_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

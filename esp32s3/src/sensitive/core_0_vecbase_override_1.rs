#[doc = "Register `CORE_0_VECBASE_OVERRIDE_1` reader"]
pub struct R(crate::R<CORE_0_VECBASE_OVERRIDE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_VECBASE_OVERRIDE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_VECBASE_OVERRIDE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_VECBASE_OVERRIDE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_VECBASE_OVERRIDE_1` writer"]
pub struct W(crate::W<CORE_0_VECBASE_OVERRIDE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_VECBASE_OVERRIDE_1_SPEC>;
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
impl From<crate::W<CORE_0_VECBASE_OVERRIDE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_VECBASE_OVERRIDE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE` reader - world0 vecbase_override register, when core0 in world0 use this register to override vecbase register."]
pub type CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE` writer - world0 vecbase_override register, when core0 in world0 use this register to override vecbase register."]
pub type CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_VECBASE_OVERRIDE_1_SPEC, u32, u32, 22, O>;
#[doc = "Field `CORE_0_VECBASE_OVERRIDE_SEL` reader - Set 0x3 to sel vecbase_override to override vecbase register."]
pub type CORE_0_VECBASE_OVERRIDE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_VECBASE_OVERRIDE_SEL` writer - Set 0x3 to sel vecbase_override to override vecbase register."]
pub type CORE_0_VECBASE_OVERRIDE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_VECBASE_OVERRIDE_1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:21 - world0 vecbase_override register, when core0 in world0 use this register to override vecbase register."]
    #[inline(always)]
    pub fn core_0_vecbase_override_world0_value(&self) -> CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_R {
        CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 22:23 - Set 0x3 to sel vecbase_override to override vecbase register."]
    #[inline(always)]
    pub fn core_0_vecbase_override_sel(&self) -> CORE_0_VECBASE_OVERRIDE_SEL_R {
        CORE_0_VECBASE_OVERRIDE_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:21 - world0 vecbase_override register, when core0 in world0 use this register to override vecbase register."]
    #[inline(always)]
    pub fn core_0_vecbase_override_world0_value(
        &mut self,
    ) -> CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_W<0> {
        CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_W::new(self)
    }
    #[doc = "Bits 22:23 - Set 0x3 to sel vecbase_override to override vecbase register."]
    #[inline(always)]
    pub fn core_0_vecbase_override_sel(&mut self) -> CORE_0_VECBASE_OVERRIDE_SEL_W<22> {
        CORE_0_VECBASE_OVERRIDE_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 vecbase override configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_vecbase_override_1](index.html) module"]
pub struct CORE_0_VECBASE_OVERRIDE_1_SPEC;
impl crate::RegisterSpec for CORE_0_VECBASE_OVERRIDE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_vecbase_override_1::R](R) reader structure"]
impl crate::Readable for CORE_0_VECBASE_OVERRIDE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_vecbase_override_1::W](W) writer structure"]
impl crate::Writable for CORE_0_VECBASE_OVERRIDE_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_VECBASE_OVERRIDE_1 to value 0"]
impl crate::Resettable for CORE_0_VECBASE_OVERRIDE_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

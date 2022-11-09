#[doc = "Register `CORE_0_VECBASE_OVERRIDE_0` reader"]
pub struct R(crate::R<CORE_0_VECBASE_OVERRIDE_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_VECBASE_OVERRIDE_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_VECBASE_OVERRIDE_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_VECBASE_OVERRIDE_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_VECBASE_OVERRIDE_0` writer"]
pub struct W(crate::W<CORE_0_VECBASE_OVERRIDE_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_VECBASE_OVERRIDE_0_SPEC>;
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
impl From<crate::W<CORE_0_VECBASE_OVERRIDE_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_VECBASE_OVERRIDE_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_VECBASE_WORLD_MASK` reader - Set 1 to mask world, then only world0_value will work."]
pub type CORE_0_VECBASE_WORLD_MASK_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_VECBASE_WORLD_MASK` writer - Set 1 to mask world, then only world0_value will work."]
pub type CORE_0_VECBASE_WORLD_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CORE_0_VECBASE_OVERRIDE_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to mask world, then only world0_value will work."]
    #[inline(always)]
    pub fn core_0_vecbase_world_mask(&self) -> CORE_0_VECBASE_WORLD_MASK_R {
        CORE_0_VECBASE_WORLD_MASK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to mask world, then only world0_value will work."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_vecbase_world_mask(&mut self) -> CORE_0_VECBASE_WORLD_MASK_W<0> {
        CORE_0_VECBASE_WORLD_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 vecbase override configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_vecbase_override_0](index.html) module"]
pub struct CORE_0_VECBASE_OVERRIDE_0_SPEC;
impl crate::RegisterSpec for CORE_0_VECBASE_OVERRIDE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_vecbase_override_0::R](R) reader structure"]
impl crate::Readable for CORE_0_VECBASE_OVERRIDE_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_vecbase_override_0::W](W) writer structure"]
impl crate::Writable for CORE_0_VECBASE_OVERRIDE_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_VECBASE_OVERRIDE_0 to value 0x01"]
impl crate::Resettable for CORE_0_VECBASE_OVERRIDE_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

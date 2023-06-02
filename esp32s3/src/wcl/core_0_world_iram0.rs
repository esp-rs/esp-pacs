#[doc = "Register `Core_0_World_IRam0` reader"]
pub struct R(crate::R<CORE_0_WORLD_IRAM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_WORLD_IRAM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_WORLD_IRAM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_WORLD_IRAM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_0_World_IRam0` writer"]
pub struct W(crate::W<CORE_0_WORLD_IRAM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_WORLD_IRAM0_SPEC>;
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
impl From<crate::W<CORE_0_WORLD_IRAM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_WORLD_IRAM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_WORLD_IRAM0` reader - this field is used to read current world of Iram0 bus"]
pub type CORE_0_WORLD_IRAM0_R = crate::FieldReader;
#[doc = "Field `CORE_0_WORLD_IRAM0` writer - this field is used to read current world of Iram0 bus"]
pub type CORE_0_WORLD_IRAM0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_WORLD_IRAM0_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - this field is used to read current world of Iram0 bus"]
    #[inline(always)]
    pub fn core_0_world_iram0(&self) -> CORE_0_WORLD_IRAM0_R {
        CORE_0_WORLD_IRAM0_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_World_IRam0")
            .field(
                "core_0_world_iram0",
                &format_args!("{}", self.core_0_world_iram0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_WORLD_IRAM0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - this field is used to read current world of Iram0 bus"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_world_iram0(&mut self) -> CORE_0_WORLD_IRAM0_W<0> {
        CORE_0_WORLD_IRAM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_0 Iram0 world register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_world_iram0](index.html) module"]
pub struct CORE_0_WORLD_IRAM0_SPEC;
impl crate::RegisterSpec for CORE_0_WORLD_IRAM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_world_iram0::R](R) reader structure"]
impl crate::Readable for CORE_0_WORLD_IRAM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_world_iram0::W](W) writer structure"]
impl crate::Writable for CORE_0_WORLD_IRAM0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_0_World_IRam0 to value 0"]
impl crate::Resettable for CORE_0_WORLD_IRAM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `Core_0_World_Cancel` writer"]
pub struct W(crate::W<CORE_0_WORLD_CANCEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_WORLD_CANCEL_SPEC>;
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
impl From<crate::W<CORE_0_WORLD_CANCEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_WORLD_CANCEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_WORLD_CANCEL` writer - This field is used to cancel switch world configuration,if the trigger address and update configuration complete,use this register to cancel world switch, jujst need write any value,the hardware only checks the write operation of this register and does not case about its value"]
pub type CORE_0_WORLD_CANCEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_WORLD_CANCEL_SPEC, 32, O, u32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_WORLD_CANCEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - This field is used to cancel switch world configuration,if the trigger address and update configuration complete,use this register to cancel world switch, jujst need write any value,the hardware only checks the write operation of this register and does not case about its value"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_world_cancel(&mut self) -> CORE_0_WORLD_CANCEL_W<0> {
        CORE_0_WORLD_CANCEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_0 configuration cancel register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_world_cancel](index.html) module"]
pub struct CORE_0_WORLD_CANCEL_SPEC;
impl crate::RegisterSpec for CORE_0_WORLD_CANCEL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [core_0_world_cancel::W](W) writer structure"]
impl crate::Writable for CORE_0_WORLD_CANCEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_0_World_Cancel to value 0"]
impl crate::Resettable for CORE_0_WORLD_CANCEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

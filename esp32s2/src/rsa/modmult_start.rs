#[doc = "Register `MODMULT_START` writer"]
pub struct W(crate::W<MODMULT_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODMULT_START_SPEC>;
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
impl From<crate::W<MODMULT_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODMULT_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODMULT_START` writer - Set this bit to 1 to start the modular multiplication."]
pub type MODMULT_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODMULT_START_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to start the modular multiplication."]
    #[inline(always)]
    pub fn modmult_start(&mut self) -> MODMULT_START_W<0> {
        MODMULT_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modular multiplication starting bit\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modmult_start](index.html) module"]
pub struct MODMULT_START_SPEC;
impl crate::RegisterSpec for MODMULT_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [modmult_start::W](W) writer structure"]
impl crate::Writable for MODMULT_START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODMULT_START to value 0"]
impl crate::Resettable for MODMULT_START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

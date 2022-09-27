#[doc = "Register `CORE_1_RCD_PDEBUGENABLE` reader"]
pub struct R(crate::R<CORE_1_RCD_PDEBUGENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_RCD_PDEBUGENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_RCD_PDEBUGENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_RCD_PDEBUGENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_RCD_PDEBUGENABLE` writer"]
pub struct W(crate::W<CORE_1_RCD_PDEBUGENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_RCD_PDEBUGENABLE_SPEC>;
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
impl From<crate::W<CORE_1_RCD_PDEBUGENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_RCD_PDEBUGENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_RCD_PDEBUGENABLE` reader - Core1 Pdebugenable,set 1 to open Core1 Pdebug interface, then can get Core1 PC"]
pub type CORE_1_RCD_PDEBUGENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CORE_1_RCD_PDEBUGENABLE` writer - Core1 Pdebugenable,set 1 to open Core1 Pdebug interface, then can get Core1 PC"]
pub type CORE_1_RCD_PDEBUGENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CORE_1_RCD_PDEBUGENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Core1 Pdebugenable,set 1 to open Core1 Pdebug interface, then can get Core1 PC"]
    #[inline(always)]
    pub fn core_1_rcd_pdebugenable(&self) -> CORE_1_RCD_PDEBUGENABLE_R {
        CORE_1_RCD_PDEBUGENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core1 Pdebugenable,set 1 to open Core1 Pdebug interface, then can get Core1 PC"]
    #[inline(always)]
    pub fn core_1_rcd_pdebugenable(&mut self) -> CORE_1_RCD_PDEBUGENABLE_W<0> {
        CORE_1_RCD_PDEBUGENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 pdebug configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_rcd_pdebugenable](index.html) module"]
pub struct CORE_1_RCD_PDEBUGENABLE_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_PDEBUGENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_rcd_pdebugenable::R](R) reader structure"]
impl crate::Readable for CORE_1_RCD_PDEBUGENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_rcd_pdebugenable::W](W) writer structure"]
impl crate::Writable for CORE_1_RCD_PDEBUGENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_RCD_PDEBUGENABLE to value 0"]
impl crate::Resettable for CORE_1_RCD_PDEBUGENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

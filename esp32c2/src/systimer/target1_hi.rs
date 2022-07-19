#[doc = "Register `TARGET1_HI` reader"]
pub struct R(crate::R<TARGET1_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET1_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET1_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET1_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET1_HI` writer"]
pub struct W(crate::W<TARGET1_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET1_HI_SPEC>;
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
impl From<crate::W<TARGET1_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET1_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_TARGET1_HI` reader - timer taget1 high 20 bits"]
pub type TIMER_TARGET1_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMER_TARGET1_HI` writer - timer taget1 high 20 bits"]
pub type TIMER_TARGET1_HI_W<'a> = crate::FieldWriter<'a, u32, TARGET1_HI_SPEC, u32, u32, 20, 0>;
impl R {
    #[doc = "Bits 0:19 - timer taget1 high 20 bits"]
    #[inline(always)]
    pub fn timer_target1_hi(&self) -> TIMER_TARGET1_HI_R {
        TIMER_TARGET1_HI_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - timer taget1 high 20 bits"]
    #[inline(always)]
    pub fn timer_target1_hi(&mut self) -> TIMER_TARGET1_HI_W {
        TIMER_TARGET1_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system timer comp1 value high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target1_hi](index.html) module"]
pub struct TARGET1_HI_SPEC;
impl crate::RegisterSpec for TARGET1_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target1_hi::R](R) reader structure"]
impl crate::Readable for TARGET1_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target1_hi::W](W) writer structure"]
impl crate::Writable for TARGET1_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TARGET1_HI to value 0"]
impl crate::Resettable for TARGET1_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

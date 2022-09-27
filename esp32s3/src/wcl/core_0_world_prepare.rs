#[doc = "Register `Core_0_World_PREPARE` reader"]
pub struct R(crate::R<CORE_0_WORLD_PREPARE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_WORLD_PREPARE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_WORLD_PREPARE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_WORLD_PREPARE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_0_World_PREPARE` writer"]
pub struct W(crate::W<CORE_0_WORLD_PREPARE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_WORLD_PREPARE_SPEC>;
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
impl From<crate::W<CORE_0_WORLD_PREPARE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_WORLD_PREPARE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_WORLD_PREPARE` reader - This field to used to set world to enter, 2'b01 means WORLD0, 2'b10 means WORLD1"]
pub type CORE_0_WORLD_PREPARE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_WORLD_PREPARE` writer - This field to used to set world to enter, 2'b01 means WORLD0, 2'b10 means WORLD1"]
pub type CORE_0_WORLD_PREPARE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_WORLD_PREPARE_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - This field to used to set world to enter, 2'b01 means WORLD0, 2'b10 means WORLD1"]
    #[inline(always)]
    pub fn core_0_world_prepare(&self) -> CORE_0_WORLD_PREPARE_R {
        CORE_0_WORLD_PREPARE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field to used to set world to enter, 2'b01 means WORLD0, 2'b10 means WORLD1"]
    #[inline(always)]
    pub fn core_0_world_prepare(&mut self) -> CORE_0_WORLD_PREPARE_W<0> {
        CORE_0_WORLD_PREPARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_0 prepare world configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_world_prepare](index.html) module"]
pub struct CORE_0_WORLD_PREPARE_SPEC;
impl crate::RegisterSpec for CORE_0_WORLD_PREPARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_world_prepare::R](R) reader structure"]
impl crate::Readable for CORE_0_WORLD_PREPARE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_world_prepare::W](W) writer structure"]
impl crate::Writable for CORE_0_WORLD_PREPARE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Core_0_World_PREPARE to value 0"]
impl crate::Resettable for CORE_0_WORLD_PREPARE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

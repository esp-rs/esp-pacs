#[doc = "Register `DB0_RED_CFG` reader"]
pub struct R(crate::R<DB0_RED_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DB0_RED_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DB0_RED_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DB0_RED_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DB0_RED_CFG` writer"]
pub struct W(crate::W<DB0_RED_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DB0_RED_CFG_SPEC>;
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
impl From<crate::W<DB0_RED_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DB0_RED_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB0_RED` reader - Shadow register for RED"]
pub type DB0_RED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DB0_RED` writer - Shadow register for RED"]
pub type DB0_RED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DB0_RED_CFG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Shadow register for RED"]
    #[inline(always)]
    pub fn db0_red(&self) -> DB0_RED_R {
        DB0_RED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow register for RED"]
    #[inline(always)]
    pub fn db0_red(&mut self) -> DB0_RED_W<0> {
        DB0_RED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow register for rising edge delay (RED).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [db0_red_cfg](index.html) module"]
pub struct DB0_RED_CFG_SPEC;
impl crate::RegisterSpec for DB0_RED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [db0_red_cfg::R](R) reader structure"]
impl crate::Readable for DB0_RED_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [db0_red_cfg::W](W) writer structure"]
impl crate::Writable for DB0_RED_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DB0_RED_CFG to value 0"]
impl crate::Resettable for DB0_RED_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

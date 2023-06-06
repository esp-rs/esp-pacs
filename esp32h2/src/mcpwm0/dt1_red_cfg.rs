#[doc = "Register `DT1_RED_CFG` reader"]
pub struct R(crate::R<DT1_RED_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT1_RED_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT1_RED_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT1_RED_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT1_RED_CFG` writer"]
pub struct W(crate::W<DT1_RED_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT1_RED_CFG_SPEC>;
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
impl From<crate::W<DT1_RED_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT1_RED_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB1_RED` reader - Shadow register for RED"]
pub type DB1_RED_R = crate::FieldReader<u16>;
#[doc = "Field `DB1_RED` writer - Shadow register for RED"]
pub type DB1_RED_W<'a, const O: u8> = crate::FieldWriter<'a, DT1_RED_CFG_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow register for RED"]
    #[inline(always)]
    pub fn db1_red(&self) -> DB1_RED_R {
        DB1_RED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT1_RED_CFG")
            .field("db1_red", &format_args!("{}", self.db1_red().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DT1_RED_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow register for RED"]
    #[inline(always)]
    #[must_use]
    pub fn db1_red(&mut self) -> DB1_RED_W<0> {
        DB1_RED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow register for rising edge delay (RED).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt1_red_cfg](index.html) module"]
pub struct DT1_RED_CFG_SPEC;
impl crate::RegisterSpec for DT1_RED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dt1_red_cfg::R](R) reader structure"]
impl crate::Readable for DT1_RED_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt1_red_cfg::W](W) writer structure"]
impl crate::Writable for DT1_RED_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT1_RED_CFG to value 0"]
impl crate::Resettable for DT1_RED_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `SCL_RSTART_SETUP` reader"]
pub struct R(crate::R<SCL_RSTART_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_RSTART_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_RSTART_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_RSTART_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_RSTART_SETUP` writer"]
pub struct W(crate::W<SCL_RSTART_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_RSTART_SETUP_SPEC>;
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
impl From<crate::W<SCL_RSTART_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_RSTART_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - This register is used to configure the clock num between the posedge of SCL and the negedge of SDA for restart mark."]
pub type TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIME` writer - This register is used to configure the clock num between the posedge of SCL and the negedge of SDA for restart mark."]
pub type TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCL_RSTART_SETUP_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the clock num between the posedge of SCL and the negedge of SDA for restart mark."]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the clock num between the posedge of SCL and the negedge of SDA for restart mark."]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W<0> {
        TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_rstart_setup](index.html) module"]
pub struct SCL_RSTART_SETUP_SPEC;
impl crate::RegisterSpec for SCL_RSTART_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_rstart_setup::R](R) reader structure"]
impl crate::Readable for SCL_RSTART_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_rstart_setup::W](W) writer structure"]
impl crate::Writable for SCL_RSTART_SETUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_RSTART_SETUP to value 0x08"]
impl crate::Resettable for SCL_RSTART_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}

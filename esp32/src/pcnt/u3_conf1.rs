#[doc = "Register `U3_CONF1` reader"]
pub struct R(crate::R<U3_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U3_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U3_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U3_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U3_CONF1` writer"]
pub struct W(crate::W<U3_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U3_CONF1_SPEC>;
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
impl From<crate::W<U3_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U3_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_THRES0_U3` reader - This register is used to configure thres0 value for unit3."]
pub type CNT_THRES0_U3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT_THRES0_U3` writer - This register is used to configure thres0 value for unit3."]
pub type CNT_THRES0_U3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U3_CONF1_SPEC, u16, u16, 16, O>;
#[doc = "Field `CNT_THRES1_U3` reader - This register is used to configure thres1 value for unit3."]
pub type CNT_THRES1_U3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT_THRES1_U3` writer - This register is used to configure thres1 value for unit3."]
pub type CNT_THRES1_U3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U3_CONF1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit3."]
    #[inline(always)]
    pub fn cnt_thres0_u3(&self) -> CNT_THRES0_U3_R {
        CNT_THRES0_U3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit3."]
    #[inline(always)]
    pub fn cnt_thres1_u3(&self) -> CNT_THRES1_U3_R {
        CNT_THRES1_U3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit3."]
    #[inline(always)]
    pub fn cnt_thres0_u3(&mut self) -> CNT_THRES0_U3_W<0> {
        CNT_THRES0_U3_W::new(self)
    }
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit3."]
    #[inline(always)]
    pub fn cnt_thres1_u3(&mut self) -> CNT_THRES1_U3_W<16> {
        CNT_THRES1_U3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u3_conf1](index.html) module"]
pub struct U3_CONF1_SPEC;
impl crate::RegisterSpec for U3_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u3_conf1::R](R) reader structure"]
impl crate::Readable for U3_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u3_conf1::W](W) writer structure"]
impl crate::Writable for U3_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U3_CONF1 to value 0"]
impl crate::Resettable for U3_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

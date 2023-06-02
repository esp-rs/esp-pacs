#[doc = "Register `U%s_CONF1` reader"]
pub struct R(crate::R<U_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U%s_CONF1` writer"]
pub struct W(crate::W<U_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U_CONF1_SPEC>;
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
impl From<crate::W<U_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_THRES0` reader - This register is used to configure thres0 value for unit0."]
pub type CNT_THRES0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT_THRES0` writer - This register is used to configure thres0 value for unit0."]
pub type CNT_THRES0_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF1_SPEC, 16, O, u16, u16>;
#[doc = "Field `CNT_THRES1` reader - This register is used to configure thres1 value for unit0."]
pub type CNT_THRES1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT_THRES1` writer - This register is used to configure thres1 value for unit0."]
pub type CNT_THRES1_W<'a, const O: u8> = crate::FieldWriter<'a, U_CONF1_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit0."]
    #[inline(always)]
    pub fn cnt_thres0(&self) -> CNT_THRES0_R {
        CNT_THRES0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit0."]
    #[inline(always)]
    pub fn cnt_thres1(&self) -> CNT_THRES1_R {
        CNT_THRES1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_CONF1")
            .field("cnt_thres0", &format_args!("{}", self.cnt_thres0().bits()))
            .field("cnt_thres1", &format_args!("{}", self.cnt_thres1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<U_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit0."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thres0(&mut self) -> CNT_THRES0_W<0> {
        CNT_THRES0_W::new(self)
    }
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit0."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thres1(&mut self) -> CNT_THRES1_W<16> {
        CNT_THRES1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u_conf1](index.html) module"]
pub struct U_CONF1_SPEC;
impl crate::RegisterSpec for U_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u_conf1::R](R) reader structure"]
impl crate::Readable for U_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u_conf1::W](W) writer structure"]
impl crate::Writable for U_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets U%s_CONF1 to value 0"]
impl crate::Resettable for U_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

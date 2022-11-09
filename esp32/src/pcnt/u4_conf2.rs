#[doc = "Register `U4_CONF2` reader"]
pub struct R(crate::R<U4_CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U4_CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U4_CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U4_CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U4_CONF2` writer"]
pub struct W(crate::W<U4_CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U4_CONF2_SPEC>;
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
impl From<crate::W<U4_CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U4_CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_H_LIM_U4` reader - This register is used to configure thr_h_lim value for unit4."]
pub type CNT_H_LIM_U4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT_H_LIM_U4` writer - This register is used to configure thr_h_lim value for unit4."]
pub type CNT_H_LIM_U4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U4_CONF2_SPEC, u16, u16, 16, O>;
#[doc = "Field `CNT_L_LIM_U4` reader - This register is used to confiugre thr_l_lim value for unit4."]
pub type CNT_L_LIM_U4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT_L_LIM_U4` writer - This register is used to confiugre thr_l_lim value for unit4."]
pub type CNT_L_LIM_U4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U4_CONF2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure thr_h_lim value for unit4."]
    #[inline(always)]
    pub fn cnt_h_lim_u4(&self) -> CNT_H_LIM_U4_R {
        CNT_H_LIM_U4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to confiugre thr_l_lim value for unit4."]
    #[inline(always)]
    pub fn cnt_l_lim_u4(&self) -> CNT_L_LIM_U4_R {
        CNT_L_LIM_U4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure thr_h_lim value for unit4."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_h_lim_u4(&mut self) -> CNT_H_LIM_U4_W<0> {
        CNT_H_LIM_U4_W::new(self)
    }
    #[doc = "Bits 16:31 - This register is used to confiugre thr_l_lim value for unit4."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_l_lim_u4(&mut self) -> CNT_L_LIM_U4_W<16> {
        CNT_L_LIM_U4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u4_conf2](index.html) module"]
pub struct U4_CONF2_SPEC;
impl crate::RegisterSpec for U4_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u4_conf2::R](R) reader structure"]
impl crate::Readable for U4_CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u4_conf2::W](W) writer structure"]
impl crate::Writable for U4_CONF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets U4_CONF2 to value 0"]
impl crate::Resettable for U4_CONF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

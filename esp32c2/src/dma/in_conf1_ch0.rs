#[doc = "Register `IN_CONF1_CH0` reader"]
pub struct R(crate::R<IN_CONF1_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_CONF1_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_CONF1_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_CONF1_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_CONF1_CH0` writer"]
pub struct W(crate::W<IN_CONF1_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_CONF1_CH0_SPEC>;
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
impl From<crate::W<IN_CONF1_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_CONF1_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_CHECK_OWNER_CH0` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_CH0_R = crate::BitReader<bool>;
#[doc = "Field `IN_CHECK_OWNER_CH0` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_CH0_W<'a> = crate::BitWriter<'a, u32, IN_CONF1_CH0_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner_ch0(&self) -> IN_CHECK_OWNER_CH0_R {
        IN_CHECK_OWNER_CH0_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner_ch0(&mut self) -> IN_CHECK_OWNER_CH0_W {
        IN_CHECK_OWNER_CH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_CONF1_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_conf1_ch0](index.html) module"]
pub struct IN_CONF1_CH0_SPEC;
impl crate::RegisterSpec for IN_CONF1_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_conf1_ch0::R](R) reader structure"]
impl crate::Readable for IN_CONF1_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_conf1_ch0::W](W) writer structure"]
impl crate::Writable for IN_CONF1_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_CONF1_CH0 to value 0"]
impl crate::Resettable for IN_CONF1_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

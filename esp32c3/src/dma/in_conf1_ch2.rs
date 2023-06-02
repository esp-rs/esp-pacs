#[doc = "Register `IN_CONF1_CH2` reader"]
pub struct R(crate::R<IN_CONF1_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_CONF1_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_CONF1_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_CONF1_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_CONF1_CH2` writer"]
pub struct W(crate::W<IN_CONF1_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_CONF1_CH2_SPEC>;
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
impl From<crate::W<IN_CONF1_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_CONF1_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `IN_CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_W<'a, const O: u8> = crate::BitWriter<'a, IN_CONF1_CH2_SPEC, O>;
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner(&self) -> IN_CHECK_OWNER_R {
        IN_CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF1_CH2")
            .field(
                "in_check_owner",
                &format_args!("{}", self.in_check_owner().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_CONF1_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn in_check_owner(&mut self) -> IN_CHECK_OWNER_W<12> {
        IN_CHECK_OWNER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_CONF1_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_conf1_ch2](index.html) module"]
pub struct IN_CONF1_CH2_SPEC;
impl crate::RegisterSpec for IN_CONF1_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_conf1_ch2::R](R) reader structure"]
impl crate::Readable for IN_CONF1_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_conf1_ch2::W](W) writer structure"]
impl crate::Writable for IN_CONF1_CH2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_CONF1_CH2 to value 0"]
impl crate::Resettable for IN_CONF1_CH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

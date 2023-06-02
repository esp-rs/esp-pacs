#[doc = "Register `RX_ERR_CNT` reader"]
pub struct R(crate::R<RX_ERR_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ERR_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ERR_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ERR_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_ERR_CNT` writer"]
pub struct W(crate::W<RX_ERR_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_ERR_CNT_SPEC>;
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
impl From<crate::W<RX_ERR_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_ERR_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_ERR_CNT` reader - The RX error counter register, reflects value changes under reception status."]
pub type RX_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `RX_ERR_CNT` writer - The RX error counter register, reflects value changes under reception status."]
pub type RX_ERR_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, RX_ERR_CNT_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The RX error counter register, reflects value changes under reception status."]
    #[inline(always)]
    pub fn rx_err_cnt(&self) -> RX_ERR_CNT_R {
        RX_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ERR_CNT")
            .field("rx_err_cnt", &format_args!("{}", self.rx_err_cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_ERR_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - The RX error counter register, reflects value changes under reception status."]
    #[inline(always)]
    #[must_use]
    pub fn rx_err_cnt(&mut self) -> RX_ERR_CNT_W<0> {
        RX_ERR_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_err_cnt](index.html) module"]
pub struct RX_ERR_CNT_SPEC;
impl crate::RegisterSpec for RX_ERR_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_err_cnt::R](R) reader structure"]
impl crate::Readable for RX_ERR_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_err_cnt::W](W) writer structure"]
impl crate::Writable for RX_ERR_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_ERR_CNT to value 0"]
impl crate::Resettable for RX_ERR_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

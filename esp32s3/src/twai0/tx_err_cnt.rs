#[doc = "Register `TX_ERR_CNT` reader"]
pub struct R(crate::R<TX_ERR_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_ERR_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_ERR_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_ERR_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_ERR_CNT` writer"]
pub struct W(crate::W<TX_ERR_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_ERR_CNT_SPEC>;
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
impl From<crate::W<TX_ERR_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_ERR_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_ERR_CNT` reader - The TX error counter register, reflects value changes under transmission status."]
pub type TX_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `TX_ERR_CNT` writer - The TX error counter register, reflects value changes under transmission status."]
pub type TX_ERR_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, TX_ERR_CNT_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The TX error counter register, reflects value changes under transmission status."]
    #[inline(always)]
    pub fn tx_err_cnt(&self) -> TX_ERR_CNT_R {
        TX_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_ERR_CNT")
            .field("tx_err_cnt", &format_args!("{}", self.tx_err_cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_ERR_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - The TX error counter register, reflects value changes under transmission status."]
    #[inline(always)]
    #[must_use]
    pub fn tx_err_cnt(&mut self) -> TX_ERR_CNT_W<0> {
        TX_ERR_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_err_cnt](index.html) module"]
pub struct TX_ERR_CNT_SPEC;
impl crate::RegisterSpec for TX_ERR_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_err_cnt::R](R) reader structure"]
impl crate::Readable for TX_ERR_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_err_cnt::W](W) writer structure"]
impl crate::Writable for TX_ERR_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_ERR_CNT to value 0"]
impl crate::Resettable for TX_ERR_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

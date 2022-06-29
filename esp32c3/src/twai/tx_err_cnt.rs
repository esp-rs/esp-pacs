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
pub type TX_ERR_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_ERR_CNT` writer - The TX error counter register, reflects value changes under transmission status."]
pub type TX_ERR_CNT_W<'a> = crate::FieldWriter<'a, u32, TX_ERR_CNT_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - The TX error counter register, reflects value changes under transmission status."]
    #[inline(always)]
    pub fn tx_err_cnt(&self) -> TX_ERR_CNT_R {
        TX_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The TX error counter register, reflects value changes under transmission status."]
    #[inline(always)]
    pub fn tx_err_cnt(&mut self) -> TX_ERR_CNT_W {
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
}
#[doc = "`reset()` method sets TX_ERR_CNT to value 0"]
impl crate::Resettable for TX_ERR_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

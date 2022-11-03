#[doc = "Register `TX_CFG1` reader"]
pub struct R(crate::R<TX_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CFG1` writer"]
pub struct W(crate::W<TX_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CFG1_SPEC>;
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
impl From<crate::W<TX_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_IDLE_VALUE` reader - Configures data value on tx bus when IDLE state."]
pub type TX_IDLE_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_IDLE_VALUE` writer - Configures data value on tx bus when IDLE state."]
pub type TX_IDLE_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_CFG1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - Configures data value on tx bus when IDLE state."]
    #[inline(always)]
    pub fn tx_idle_value(&self) -> TX_IDLE_VALUE_R {
        TX_IDLE_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Configures data value on tx bus when IDLE state."]
    #[inline(always)]
    #[must_use]
    pub fn tx_idle_value(&mut self) -> TX_IDLE_VALUE_W<16> {
        TX_IDLE_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel TX module configuration register1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_cfg1](index.html) module"]
pub struct TX_CFG1_SPEC;
impl crate::RegisterSpec for TX_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_cfg1::R](R) reader structure"]
impl crate::Readable for TX_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_cfg1::W](W) writer structure"]
impl crate::Writable for TX_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CFG1 to value 0"]
impl crate::Resettable for TX_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

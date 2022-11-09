#[doc = "Register `CH2_RX_CARRIER_RM` reader"]
pub struct R(crate::R<CH2_RX_CARRIER_RM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2_RX_CARRIER_RM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2_RX_CARRIER_RM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2_RX_CARRIER_RM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2_RX_CARRIER_RM` writer"]
pub struct W(crate::W<CH2_RX_CARRIER_RM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2_RX_CARRIER_RM_SPEC>;
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
impl From<crate::W<CH2_RX_CARRIER_RM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2_RX_CARRIER_RM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARRIER_LOW_THRES` reader - reg_carrier_low_thres_ch2."]
pub type CARRIER_LOW_THRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CARRIER_LOW_THRES` writer - reg_carrier_low_thres_ch2."]
pub type CARRIER_LOW_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH2_RX_CARRIER_RM_SPEC, u16, u16, 16, O>;
#[doc = "Field `CARRIER_HIGH_THRES` reader - reg_carrier_high_thres_ch2."]
pub type CARRIER_HIGH_THRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CARRIER_HIGH_THRES` writer - reg_carrier_high_thres_ch2."]
pub type CARRIER_HIGH_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH2_RX_CARRIER_RM_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - reg_carrier_low_thres_ch2."]
    #[inline(always)]
    pub fn carrier_low_thres(&self) -> CARRIER_LOW_THRES_R {
        CARRIER_LOW_THRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reg_carrier_high_thres_ch2."]
    #[inline(always)]
    pub fn carrier_high_thres(&self) -> CARRIER_HIGH_THRES_R {
        CARRIER_HIGH_THRES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - reg_carrier_low_thres_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_low_thres(&mut self) -> CARRIER_LOW_THRES_W<0> {
        CARRIER_LOW_THRES_W::new(self)
    }
    #[doc = "Bits 16:31 - reg_carrier_high_thres_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_high_thres(&mut self) -> CARRIER_HIGH_THRES_W<16> {
        CARRIER_HIGH_THRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH2_RX_CARRIER_RM_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_rx_carrier_rm](index.html) module"]
pub struct CH2_RX_CARRIER_RM_SPEC;
impl crate::RegisterSpec for CH2_RX_CARRIER_RM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2_rx_carrier_rm::R](R) reader structure"]
impl crate::Readable for CH2_RX_CARRIER_RM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2_rx_carrier_rm::W](W) writer structure"]
impl crate::Writable for CH2_RX_CARRIER_RM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2_RX_CARRIER_RM to value 0"]
impl crate::Resettable for CH2_RX_CARRIER_RM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

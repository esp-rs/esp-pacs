#[doc = "Register `BLK3_RDATA3` reader"]
pub struct R(crate::R<BLK3_RDATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK3_RDATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK3_RDATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK3_RDATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK3_RDATA3` writer"]
pub struct W(crate::W<BLK3_RDATA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK3_RDATA3_SPEC>;
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
impl From<crate::W<BLK3_RDATA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK3_RDATA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK3_DOUT3` reader - read for BLOCK3"]
pub type BLK3_DOUT3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RD_ADC1_TP_LOW` reader - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub type RD_ADC1_TP_LOW_R = crate::FieldReader;
#[doc = "Field `RD_ADC1_TP_LOW` writer - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub type RD_ADC1_TP_LOW_W<'a, const O: u8> = crate::FieldWriter<'a, BLK3_RDATA3_SPEC, 7, O>;
#[doc = "Field `RD_ADC1_TP_HIGH` reader - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub type RD_ADC1_TP_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RD_ADC1_TP_HIGH` writer - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub type RD_ADC1_TP_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, BLK3_RDATA3_SPEC, 9, O, u16, u16>;
#[doc = "Field `RD_ADC2_TP_LOW` reader - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub type RD_ADC2_TP_LOW_R = crate::FieldReader;
#[doc = "Field `RD_ADC2_TP_LOW` writer - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub type RD_ADC2_TP_LOW_W<'a, const O: u8> = crate::FieldWriter<'a, BLK3_RDATA3_SPEC, 7, O>;
#[doc = "Field `RD_ADC2_TP_HIGH` reader - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub type RD_ADC2_TP_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RD_ADC2_TP_HIGH` writer - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub type RD_ADC2_TP_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, BLK3_RDATA3_SPEC, 9, O, u16, u16>;
impl R {
    #[doc = "Bits 0:31 - read for BLOCK3"]
    #[inline(always)]
    pub fn blk3_dout3(&self) -> BLK3_DOUT3_R {
        BLK3_DOUT3_R::new(self.bits)
    }
    #[doc = "Bits 0:6 - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn rd_adc1_tp_low(&self) -> RD_ADC1_TP_LOW_R {
        RD_ADC1_TP_LOW_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:15 - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn rd_adc1_tp_high(&self) -> RD_ADC1_TP_HIGH_R {
        RD_ADC1_TP_HIGH_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:22 - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn rd_adc2_tp_low(&self) -> RD_ADC2_TP_LOW_R {
        RD_ADC2_TP_LOW_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:31 - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn rd_adc2_tp_high(&self) -> RD_ADC2_TP_HIGH_R {
        RD_ADC2_TP_HIGH_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_RDATA3")
            .field("blk3_dout3", &format_args!("{}", self.blk3_dout3().bits()))
            .field(
                "rd_adc1_tp_low",
                &format_args!("{}", self.rd_adc1_tp_low().bits()),
            )
            .field(
                "rd_adc1_tp_high",
                &format_args!("{}", self.rd_adc1_tp_high().bits()),
            )
            .field(
                "rd_adc2_tp_low",
                &format_args!("{}", self.rd_adc2_tp_low().bits()),
            )
            .field(
                "rd_adc2_tp_high",
                &format_args!("{}", self.rd_adc2_tp_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK3_RDATA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    #[must_use]
    pub fn rd_adc1_tp_low(&mut self) -> RD_ADC1_TP_LOW_W<0> {
        RD_ADC1_TP_LOW_W::new(self)
    }
    #[doc = "Bits 7:15 - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    #[must_use]
    pub fn rd_adc1_tp_high(&mut self) -> RD_ADC1_TP_HIGH_W<7> {
        RD_ADC1_TP_HIGH_W::new(self)
    }
    #[doc = "Bits 16:22 - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    #[must_use]
    pub fn rd_adc2_tp_low(&mut self) -> RD_ADC2_TP_LOW_W<16> {
        RD_ADC2_TP_LOW_W::new(self)
    }
    #[doc = "Bits 23:31 - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    #[must_use]
    pub fn rd_adc2_tp_high(&mut self) -> RD_ADC2_TP_HIGH_W<23> {
        RD_ADC2_TP_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk3_rdata3](index.html) module"]
pub struct BLK3_RDATA3_SPEC;
impl crate::RegisterSpec for BLK3_RDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk3_rdata3::R](R) reader structure"]
impl crate::Readable for BLK3_RDATA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk3_rdata3::W](W) writer structure"]
impl crate::Writable for BLK3_RDATA3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK3_RDATA3 to value 0"]
impl crate::Resettable for BLK3_RDATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

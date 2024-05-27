#[doc = "Register `BLK3_WDATA3` reader"]
pub type R = crate::R<BLK3_WDATA3_SPEC>;
#[doc = "Register `BLK3_WDATA3` writer"]
pub type W = crate::W<BLK3_WDATA3_SPEC>;
#[doc = "Field `ADC1_TP_LOW` reader - "]
pub type ADC1_TP_LOW_R = crate::FieldReader;
#[doc = "Field `ADC1_TP_LOW` writer - "]
pub type ADC1_TP_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADC1_TP_HIGH` reader - "]
pub type ADC1_TP_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `ADC1_TP_HIGH` writer - "]
pub type ADC1_TP_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ADC2_TP_LOW` reader - "]
pub type ADC2_TP_LOW_R = crate::FieldReader;
#[doc = "Field `ADC2_TP_LOW` writer - "]
pub type ADC2_TP_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADC2_TP_HIGH` reader - "]
pub type ADC2_TP_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `ADC2_TP_HIGH` writer - "]
pub type ADC2_TP_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn adc1_tp_low(&self) -> ADC1_TP_LOW_R {
        ADC1_TP_LOW_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn adc1_tp_high(&self) -> ADC1_TP_HIGH_R {
        ADC1_TP_HIGH_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn adc2_tp_low(&self) -> ADC2_TP_LOW_R {
        ADC2_TP_LOW_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn adc2_tp_high(&self) -> ADC2_TP_HIGH_R {
        ADC2_TP_HIGH_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_WDATA3")
            .field("adc1_tp_low", &self.adc1_tp_low())
            .field("adc1_tp_high", &self.adc1_tp_high())
            .field("adc2_tp_low", &self.adc2_tp_low())
            .field("adc2_tp_high", &self.adc2_tp_high())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_tp_low(&mut self) -> ADC1_TP_LOW_W<BLK3_WDATA3_SPEC> {
        ADC1_TP_LOW_W::new(self, 0)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_tp_high(&mut self) -> ADC1_TP_HIGH_W<BLK3_WDATA3_SPEC> {
        ADC1_TP_HIGH_W::new(self, 7)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_tp_low(&mut self) -> ADC2_TP_LOW_W<BLK3_WDATA3_SPEC> {
        ADC2_TP_LOW_W::new(self, 16)
    }
    #[doc = "Bits 23:31"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_tp_high(&mut self) -> ADC2_TP_HIGH_W<BLK3_WDATA3_SPEC> {
        ADC2_TP_HIGH_W::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_WDATA3_SPEC;
impl crate::RegisterSpec for BLK3_WDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_wdata3::R`](R) reader structure"]
impl crate::Readable for BLK3_WDATA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk3_wdata3::W`](W) writer structure"]
impl crate::Writable for BLK3_WDATA3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK3_WDATA3 to value 0"]
impl crate::Resettable for BLK3_WDATA3_SPEC {
    const RESET_VALUE: u32 = 0;
}

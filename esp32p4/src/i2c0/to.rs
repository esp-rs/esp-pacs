#[doc = "Register `TO` reader"]
pub type R = crate::R<TO_SPEC>;
#[doc = "Register `TO` writer"]
pub type W = crate::W<TO_SPEC>;
#[doc = "Field `TIME_OUT_VALUE` reader - Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2^(reg_time_out_value). Measurement unit: i2c_sclk."]
pub type TIME_OUT_VALUE_R = crate::FieldReader;
#[doc = "Field `TIME_OUT_VALUE` writer - Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2^(reg_time_out_value). Measurement unit: i2c_sclk."]
pub type TIME_OUT_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TIME_OUT_EN` reader - Configures to enable time out control. 0: No effect 1: Enable"]
pub type TIME_OUT_EN_R = crate::BitReader;
#[doc = "Field `TIME_OUT_EN` writer - Configures to enable time out control. 0: No effect 1: Enable"]
pub type TIME_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2^(reg_time_out_value). Measurement unit: i2c_sclk."]
    #[inline(always)]
    pub fn time_out_value(&self) -> TIME_OUT_VALUE_R {
        TIME_OUT_VALUE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Configures to enable time out control. 0: No effect 1: Enable"]
    #[inline(always)]
    pub fn time_out_en(&self) -> TIME_OUT_EN_R {
        TIME_OUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TO")
            .field("time_out_value", &self.time_out_value())
            .field("time_out_en", &self.time_out_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Configures the timeout threshold period for SCL stucking at high or low level. The actual period is 2^(reg_time_out_value). Measurement unit: i2c_sclk."]
    #[inline(always)]
    pub fn time_out_value(&mut self) -> TIME_OUT_VALUE_W<TO_SPEC> {
        TIME_OUT_VALUE_W::new(self, 0)
    }
    #[doc = "Bit 5 - Configures to enable time out control. 0: No effect 1: Enable"]
    #[inline(always)]
    pub fn time_out_en(&mut self) -> TIME_OUT_EN_W<TO_SPEC> {
        TIME_OUT_EN_W::new(self, 5)
    }
}
#[doc = "Setting time out control for receiving data.\n\nYou can [`read`](crate::Reg::read) this register and get [`to::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TO_SPEC;
impl crate::RegisterSpec for TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`to::R`](R) reader structure"]
impl crate::Readable for TO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`to::W`](W) writer structure"]
impl crate::Writable for TO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TO to value 0x10"]
impl crate::Resettable for TO_SPEC {
    const RESET_VALUE: u32 = 0x10;
}

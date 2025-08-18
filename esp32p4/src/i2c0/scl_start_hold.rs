#[doc = "Register `SCL_START_HOLD` reader"]
pub type R = crate::R<SCL_START_HOLD_SPEC>;
#[doc = "Register `SCL_START_HOLD` writer"]
pub type W = crate::W<SCL_START_HOLD_SPEC>;
#[doc = "Field `TIME` reader - Configures the time between the falling edge of SDA and the falling edge of SCL for a START condition. Measurement unit: i2c_sclk."]
pub type TIME_R = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - Configures the time between the falling edge of SDA and the falling edge of SCL for a START condition. Measurement unit: i2c_sclk."]
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Configures the time between the falling edge of SDA and the falling edge of SCL for a START condition. Measurement unit: i2c_sclk."]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_START_HOLD")
            .field("time", &self.time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Configures the time between the falling edge of SDA and the falling edge of SCL for a START condition. Measurement unit: i2c_sclk."]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W<'_, SCL_START_HOLD_SPEC> {
        TIME_W::new(self, 0)
    }
}
#[doc = "Configures the delay between the SDA and SCL negative edge for a start condition\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_start_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_start_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_START_HOLD_SPEC;
impl crate::RegisterSpec for SCL_START_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_start_hold::R`](R) reader structure"]
impl crate::Readable for SCL_START_HOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_start_hold::W`](W) writer structure"]
impl crate::Writable for SCL_START_HOLD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_START_HOLD to value 0x08"]
impl crate::Resettable for SCL_START_HOLD_SPEC {
    const RESET_VALUE: u32 = 0x08;
}

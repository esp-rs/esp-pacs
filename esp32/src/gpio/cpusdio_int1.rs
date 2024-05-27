#[doc = "Register `CPUSDIO_INT1` reader"]
pub type R = crate::R<CPUSDIO_INT1_SPEC>;
#[doc = "Register `CPUSDIO_INT1` writer"]
pub type W = crate::W<CPUSDIO_INT1_SPEC>;
#[doc = "Field `SDIO_INT_H` reader - SDIO's extent GPIO32~39 interrupt"]
pub type SDIO_INT_H_R = crate::FieldReader;
#[doc = "Field `PIN_PAD_DRIVER` reader - "]
pub type PIN_PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `PIN_PAD_DRIVER` writer - "]
pub type PIN_PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN_INT_TYPE` reader - "]
pub type PIN_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `PIN_INT_TYPE` writer - "]
pub type PIN_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PIN_WAKEUP_ENABLE` reader - "]
pub type PIN_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `PIN_WAKEUP_ENABLE` writer - "]
pub type PIN_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN_CONFIG` reader - "]
pub type PIN_CONFIG_R = crate::FieldReader;
#[doc = "Field `PIN_CONFIG` writer - "]
pub type PIN_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN_INT_ENA` reader - "]
pub type PIN_INT_ENA_R = crate::FieldReader;
#[doc = "Field `PIN_INT_ENA` writer - "]
pub type PIN_INT_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - SDIO's extent GPIO32~39 interrupt"]
    #[inline(always)]
    pub fn sdio_int_h(&self) -> SDIO_INT_H_R {
        SDIO_INT_H_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin_pad_driver(&self) -> PIN_PAD_DRIVER_R {
        PIN_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn pin_int_type(&self) -> PIN_INT_TYPE_R {
        PIN_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin_wakeup_enable(&self) -> PIN_WAKEUP_ENABLE_R {
        PIN_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn pin_config(&self) -> PIN_CONFIG_R {
        PIN_CONFIG_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn pin_int_ena(&self) -> PIN_INT_ENA_R {
        PIN_INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUSDIO_INT1")
            .field("sdio_int_h", &self.sdio_int_h())
            .field("pin_pad_driver", &self.pin_pad_driver())
            .field("pin_int_type", &self.pin_int_type())
            .field("pin_wakeup_enable", &self.pin_wakeup_enable())
            .field("pin_config", &self.pin_config())
            .field("pin_int_ena", &self.pin_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pin_pad_driver(&mut self) -> PIN_PAD_DRIVER_W<CPUSDIO_INT1_SPEC> {
        PIN_PAD_DRIVER_W::new(self, 2)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    #[must_use]
    pub fn pin_int_type(&mut self) -> PIN_INT_TYPE_W<CPUSDIO_INT1_SPEC> {
        PIN_INT_TYPE_W::new(self, 7)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pin_wakeup_enable(&mut self) -> PIN_WAKEUP_ENABLE_W<CPUSDIO_INT1_SPEC> {
        PIN_WAKEUP_ENABLE_W::new(self, 10)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn pin_config(&mut self) -> PIN_CONFIG_W<CPUSDIO_INT1_SPEC> {
        PIN_CONFIG_W::new(self, 11)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    #[must_use]
    pub fn pin_int_ena(&mut self) -> PIN_INT_ENA_W<CPUSDIO_INT1_SPEC> {
        PIN_INT_ENA_W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpusdio_int1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUSDIO_INT1_SPEC;
impl crate::RegisterSpec for CPUSDIO_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpusdio_int1::R`](R) reader structure"]
impl crate::Readable for CPUSDIO_INT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpusdio_int1::W`](W) writer structure"]
impl crate::Writable for CPUSDIO_INT1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUSDIO_INT1 to value 0"]
impl crate::Resettable for CPUSDIO_INT1_SPEC {
    const RESET_VALUE: u32 = 0;
}

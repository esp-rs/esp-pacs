#[doc = "Register `FLASH_WAITI_CTRL1` reader"]
pub type R = crate::R<FLASH_WAITI_CTRL1_SPEC>;
#[doc = "Register `FLASH_WAITI_CTRL1` writer"]
pub type W = crate::W<FLASH_WAITI_CTRL1_SPEC>;
#[doc = "Field `WAITI_IDLE_DELAY_TIME` reader - SPI1 wait idle gap time configuration. SPI1 slv fsm will count during SPI1 IDLE."]
pub type WAITI_IDLE_DELAY_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `WAITI_IDLE_DELAY_TIME` writer - SPI1 wait idle gap time configuration. SPI1 slv fsm will count during SPI1 IDLE."]
pub type WAITI_IDLE_DELAY_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `WAITI_IDLE_DELAY_TIME_EN` reader - Enable SPI1 wait idle gap time count functon. 1: Enable. 0: Disable."]
pub type WAITI_IDLE_DELAY_TIME_EN_R = crate::BitReader;
#[doc = "Field `WAITI_IDLE_DELAY_TIME_EN` writer - Enable SPI1 wait idle gap time count functon. 1: Enable. 0: Disable."]
pub type WAITI_IDLE_DELAY_TIME_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - SPI1 wait idle gap time configuration. SPI1 slv fsm will count during SPI1 IDLE."]
    #[inline(always)]
    pub fn waiti_idle_delay_time(&self) -> WAITI_IDLE_DELAY_TIME_R {
        WAITI_IDLE_DELAY_TIME_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Enable SPI1 wait idle gap time count functon. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn waiti_idle_delay_time_en(&self) -> WAITI_IDLE_DELAY_TIME_EN_R {
        WAITI_IDLE_DELAY_TIME_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_WAITI_CTRL1")
            .field("waiti_idle_delay_time", &self.waiti_idle_delay_time())
            .field("waiti_idle_delay_time_en", &self.waiti_idle_delay_time_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - SPI1 wait idle gap time configuration. SPI1 slv fsm will count during SPI1 IDLE."]
    #[inline(always)]
    pub fn waiti_idle_delay_time(&mut self) -> WAITI_IDLE_DELAY_TIME_W<FLASH_WAITI_CTRL1_SPEC> {
        WAITI_IDLE_DELAY_TIME_W::new(self, 0)
    }
    #[doc = "Bit 10 - Enable SPI1 wait idle gap time count functon. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn waiti_idle_delay_time_en(
        &mut self,
    ) -> WAITI_IDLE_DELAY_TIME_EN_W<FLASH_WAITI_CTRL1_SPEC> {
        WAITI_IDLE_DELAY_TIME_EN_W::new(self, 10)
    }
}
#[doc = "SPI1 wait idle control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_waiti_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_waiti_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_WAITI_CTRL1_SPEC;
impl crate::RegisterSpec for FLASH_WAITI_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_waiti_ctrl1::R`](R) reader structure"]
impl crate::Readable for FLASH_WAITI_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_waiti_ctrl1::W`](W) writer structure"]
impl crate::Writable for FLASH_WAITI_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_WAITI_CTRL1 to value 0"]
impl crate::Resettable for FLASH_WAITI_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SPI_MEM_FLASH_WAITI_CTRL1` reader"]
pub type R = crate::R<SPI_MEM_FLASH_WAITI_CTRL1_SPEC>;
#[doc = "Register `SPI_MEM_FLASH_WAITI_CTRL1` writer"]
pub type W = crate::W<SPI_MEM_FLASH_WAITI_CTRL1_SPEC>;
#[doc = "Field `SPI_MEM_WAITI_IDLE_DELAY_TIME` reader - SPI1 wait idle gap time configuration. SPI1 slv fsm will count during SPI1 IDLE."]
pub type SPI_MEM_WAITI_IDLE_DELAY_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_WAITI_IDLE_DELAY_TIME` writer - SPI1 wait idle gap time configuration. SPI1 slv fsm will count during SPI1 IDLE."]
pub type SPI_MEM_WAITI_IDLE_DELAY_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SPI_MEM_WAITI_IDLE_DELAY_TIME_EN` reader - Enable SPI1 wait idle gap time count functon. 1: Enable. 0: Disable."]
pub type SPI_MEM_WAITI_IDLE_DELAY_TIME_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WAITI_IDLE_DELAY_TIME_EN` writer - Enable SPI1 wait idle gap time count functon. 1: Enable. 0: Disable."]
pub type SPI_MEM_WAITI_IDLE_DELAY_TIME_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - SPI1 wait idle gap time configuration. SPI1 slv fsm will count during SPI1 IDLE."]
    #[inline(always)]
    pub fn spi_mem_waiti_idle_delay_time(&self) -> SPI_MEM_WAITI_IDLE_DELAY_TIME_R {
        SPI_MEM_WAITI_IDLE_DELAY_TIME_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Enable SPI1 wait idle gap time count functon. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn spi_mem_waiti_idle_delay_time_en(&self) -> SPI_MEM_WAITI_IDLE_DELAY_TIME_EN_R {
        SPI_MEM_WAITI_IDLE_DELAY_TIME_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_FLASH_WAITI_CTRL1")
            .field(
                "spi_mem_waiti_idle_delay_time",
                &self.spi_mem_waiti_idle_delay_time(),
            )
            .field(
                "spi_mem_waiti_idle_delay_time_en",
                &self.spi_mem_waiti_idle_delay_time_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - SPI1 wait idle gap time configuration. SPI1 slv fsm will count during SPI1 IDLE."]
    #[inline(always)]
    pub fn spi_mem_waiti_idle_delay_time(
        &mut self,
    ) -> SPI_MEM_WAITI_IDLE_DELAY_TIME_W<'_, SPI_MEM_FLASH_WAITI_CTRL1_SPEC> {
        SPI_MEM_WAITI_IDLE_DELAY_TIME_W::new(self, 0)
    }
    #[doc = "Bit 10 - Enable SPI1 wait idle gap time count functon. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn spi_mem_waiti_idle_delay_time_en(
        &mut self,
    ) -> SPI_MEM_WAITI_IDLE_DELAY_TIME_EN_W<'_, SPI_MEM_FLASH_WAITI_CTRL1_SPEC> {
        SPI_MEM_WAITI_IDLE_DELAY_TIME_EN_W::new(self, 10)
    }
}
#[doc = "SPI1 wait idle control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_flash_waiti_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_flash_waiti_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_FLASH_WAITI_CTRL1_SPEC;
impl crate::RegisterSpec for SPI_MEM_FLASH_WAITI_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_flash_waiti_ctrl1::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_FLASH_WAITI_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_flash_waiti_ctrl1::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_FLASH_WAITI_CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_FLASH_WAITI_CTRL1 to value 0"]
impl crate::Resettable for SPI_MEM_FLASH_WAITI_CTRL1_SPEC {}

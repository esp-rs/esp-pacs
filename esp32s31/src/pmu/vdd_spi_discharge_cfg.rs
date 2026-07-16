#[doc = "Register `VDD_SPI_DISCHARGE_CFG` reader"]
pub type R = crate::R<VDD_SPI_DISCHARGE_CFG_SPEC>;
#[doc = "Register `VDD_SPI_DISCHARGE_CFG` writer"]
pub type W = crate::W<VDD_SPI_DISCHARGE_CFG_SPEC>;
#[doc = "Field `FLASH_DISCHARGE_TIME` reader - need_des"]
pub type FLASH_DISCHARGE_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_DISCHARGE_TIME` writer - need_des"]
pub type FLASH_DISCHARGE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - need_des"]
    #[inline(always)]
    pub fn flash_discharge_time(&self) -> FLASH_DISCHARGE_TIME_R {
        FLASH_DISCHARGE_TIME_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDD_SPI_DISCHARGE_CFG")
            .field("flash_discharge_time", &self.flash_discharge_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - need_des"]
    #[inline(always)]
    pub fn flash_discharge_time(
        &mut self,
    ) -> FLASH_DISCHARGE_TIME_W<'_, VDD_SPI_DISCHARGE_CFG_SPEC> {
        FLASH_DISCHARGE_TIME_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_spi_discharge_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdd_spi_discharge_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDD_SPI_DISCHARGE_CFG_SPEC;
impl crate::RegisterSpec for VDD_SPI_DISCHARGE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_spi_discharge_cfg::R`](R) reader structure"]
impl crate::Readable for VDD_SPI_DISCHARGE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vdd_spi_discharge_cfg::W`](W) writer structure"]
impl crate::Writable for VDD_SPI_DISCHARGE_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDD_SPI_DISCHARGE_CFG to value 0x08"]
impl crate::Resettable for VDD_SPI_DISCHARGE_CFG_SPEC {
    const RESET_VALUE: u32 = 0x08;
}

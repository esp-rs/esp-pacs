#[doc = "Register `SPI_CMD` reader"]
pub type R = crate::R<SPI_CMD_SPEC>;
#[doc = "Register `SPI_CMD` writer"]
pub type W = crate::W<SPI_CMD_SPEC>;
#[doc = "Field `SPI_CONF_BITLEN` reader - Define the APB cycles of SPI_CONF state. Can be configured in CONF state."]
pub type SPI_CONF_BITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_CONF_BITLEN` writer - Define the APB cycles of SPI_CONF state. Can be configured in CONF state."]
pub type SPI_CONF_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SPI_UPDATE` writer - Set this bit to synchronize SPI registers from APB clock domain into SPI module clock domain, which is only used in SPI master mode."]
pub type SPI_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_USR` reader - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf."]
pub type SPI_USR_R = crate::BitReader;
#[doc = "Field `SPI_USR` writer - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf."]
pub type SPI_USR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:17 - Define the APB cycles of SPI_CONF state. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_conf_bitlen(&self) -> SPI_CONF_BITLEN_R {
        SPI_CONF_BITLEN_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bit 24 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn spi_usr(&self) -> SPI_USR_R {
        SPI_USR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CMD")
            .field("spi_conf_bitlen", &self.spi_conf_bitlen())
            .field("spi_usr", &self.spi_usr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - Define the APB cycles of SPI_CONF state. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_conf_bitlen(&mut self) -> SPI_CONF_BITLEN_W<'_, SPI_CMD_SPEC> {
        SPI_CONF_BITLEN_W::new(self, 0)
    }
    #[doc = "Bit 23 - Set this bit to synchronize SPI registers from APB clock domain into SPI module clock domain, which is only used in SPI master mode."]
    #[inline(always)]
    pub fn spi_update(&mut self) -> SPI_UPDATE_W<'_, SPI_CMD_SPEC> {
        SPI_UPDATE_W::new(self, 23)
    }
    #[doc = "Bit 24 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn spi_usr(&mut self) -> SPI_USR_W<'_, SPI_CMD_SPEC> {
        SPI_USR_W::new(self, 24)
    }
}
#[doc = "Command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CMD_SPEC;
impl crate::RegisterSpec for SPI_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_cmd::R`](R) reader structure"]
impl crate::Readable for SPI_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_cmd::W`](W) writer structure"]
impl crate::Writable for SPI_CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_CMD to value 0"]
impl crate::Resettable for SPI_CMD_SPEC {}

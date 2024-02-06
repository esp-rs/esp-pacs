#[doc = "Register `SPI_SLAVE1` reader"]
pub type R = crate::R<SPI_SLAVE1_SPEC>;
#[doc = "Register `SPI_SLAVE1` writer"]
pub type W = crate::W<SPI_SLAVE1_SPEC>;
#[doc = "Field `SPI_SLV_DATA_BITLEN` reader - The transferred data bit length in SPI slave FD and HD mode."]
pub type SPI_SLV_DATA_BITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_SLV_DATA_BITLEN` writer - The transferred data bit length in SPI slave FD and HD mode."]
pub type SPI_SLV_DATA_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SPI_SLV_LAST_COMMAND` reader - In the slave mode it is the value of command."]
pub type SPI_SLV_LAST_COMMAND_R = crate::FieldReader;
#[doc = "Field `SPI_SLV_LAST_COMMAND` writer - In the slave mode it is the value of command."]
pub type SPI_SLV_LAST_COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_SLV_LAST_ADDR` reader - In the slave mode it is the value of address."]
pub type SPI_SLV_LAST_ADDR_R = crate::FieldReader;
#[doc = "Field `SPI_SLV_LAST_ADDR` writer - In the slave mode it is the value of address."]
pub type SPI_SLV_LAST_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:17 - The transferred data bit length in SPI slave FD and HD mode."]
    #[inline(always)]
    pub fn spi_slv_data_bitlen(&self) -> SPI_SLV_DATA_BITLEN_R {
        SPI_SLV_DATA_BITLEN_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:25 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn spi_slv_last_command(&self) -> SPI_SLV_LAST_COMMAND_R {
        SPI_SLV_LAST_COMMAND_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 26:31 - In the slave mode it is the value of address."]
    #[inline(always)]
    pub fn spi_slv_last_addr(&self) -> SPI_SLV_LAST_ADDR_R {
        SPI_SLV_LAST_ADDR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SLAVE1")
            .field(
                "spi_slv_data_bitlen",
                &format_args!("{}", self.spi_slv_data_bitlen().bits()),
            )
            .field(
                "spi_slv_last_command",
                &format_args!("{}", self.spi_slv_last_command().bits()),
            )
            .field(
                "spi_slv_last_addr",
                &format_args!("{}", self.spi_slv_last_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SLAVE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:17 - The transferred data bit length in SPI slave FD and HD mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_data_bitlen(&mut self) -> SPI_SLV_DATA_BITLEN_W<SPI_SLAVE1_SPEC> {
        SPI_SLV_DATA_BITLEN_W::new(self, 0)
    }
    #[doc = "Bits 18:25 - In the slave mode it is the value of command."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_last_command(&mut self) -> SPI_SLV_LAST_COMMAND_W<SPI_SLAVE1_SPEC> {
        SPI_SLV_LAST_COMMAND_W::new(self, 18)
    }
    #[doc = "Bits 26:31 - In the slave mode it is the value of address."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_last_addr(&mut self) -> SPI_SLV_LAST_ADDR_W<SPI_SLAVE1_SPEC> {
        SPI_SLV_LAST_ADDR_W::new(self, 26)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI slave control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SLAVE1_SPEC;
impl crate::RegisterSpec for SPI_SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_slave1::R`](R) reader structure"]
impl crate::Readable for SPI_SLAVE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_slave1::W`](W) writer structure"]
impl crate::Writable for SPI_SLAVE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_SLAVE1 to value 0"]
impl crate::Resettable for SPI_SLAVE1_SPEC {
    const RESET_VALUE: u32 = 0;
}

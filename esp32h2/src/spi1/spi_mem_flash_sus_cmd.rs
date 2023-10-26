#[doc = "Register `SPI_MEM_FLASH_SUS_CMD` reader"]
pub type R = crate::R<SPI_MEM_FLASH_SUS_CMD_SPEC>;
#[doc = "Register `SPI_MEM_FLASH_SUS_CMD` writer"]
pub type W = crate::W<SPI_MEM_FLASH_SUS_CMD_SPEC>;
#[doc = "Field `SPI_MEM_FLASH_PES_COMMAND` reader - Program/Erase suspend command."]
pub type SPI_MEM_FLASH_PES_COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_FLASH_PES_COMMAND` writer - Program/Erase suspend command."]
pub type SPI_MEM_FLASH_PES_COMMAND_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `SPI_MEM_WAIT_PESR_COMMAND` reader - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
pub type SPI_MEM_WAIT_PESR_COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_WAIT_PESR_COMMAND` writer - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
pub type SPI_MEM_WAIT_PESR_COMMAND_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Program/Erase suspend command."]
    #[inline(always)]
    pub fn spi_mem_flash_pes_command(&self) -> SPI_MEM_FLASH_PES_COMMAND_R {
        SPI_MEM_FLASH_PES_COMMAND_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
    #[inline(always)]
    pub fn spi_mem_wait_pesr_command(&self) -> SPI_MEM_WAIT_PESR_COMMAND_R {
        SPI_MEM_WAIT_PESR_COMMAND_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_FLASH_SUS_CMD")
            .field(
                "spi_mem_flash_pes_command",
                &format_args!("{}", self.spi_mem_flash_pes_command().bits()),
            )
            .field(
                "spi_mem_wait_pesr_command",
                &format_args!("{}", self.spi_mem_wait_pesr_command().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_FLASH_SUS_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Program/Erase suspend command."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_pes_command(
        &mut self,
    ) -> SPI_MEM_FLASH_PES_COMMAND_W<SPI_MEM_FLASH_SUS_CMD_SPEC, 0> {
        SPI_MEM_FLASH_PES_COMMAND_W::new(self)
    }
    #[doc = "Bits 16:31 - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wait_pesr_command(
        &mut self,
    ) -> SPI_MEM_WAIT_PESR_COMMAND_W<SPI_MEM_FLASH_SUS_CMD_SPEC, 16> {
        SPI_MEM_WAIT_PESR_COMMAND_W::new(self)
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
#[doc = "SPI1 flash suspend command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_flash_sus_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_flash_sus_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_FLASH_SUS_CMD_SPEC;
impl crate::RegisterSpec for SPI_MEM_FLASH_SUS_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_flash_sus_cmd::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_FLASH_SUS_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_flash_sus_cmd::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_FLASH_SUS_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_FLASH_SUS_CMD to value 0x0005_7575"]
impl crate::Resettable for SPI_MEM_FLASH_SUS_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0005_7575;
}

#[doc = "Register `SPI_MEM_FLASH_SUS_CMD` reader"]
pub struct R(crate::R<SPI_MEM_FLASH_SUS_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_FLASH_SUS_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_FLASH_SUS_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_FLASH_SUS_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_FLASH_SUS_CMD` writer"]
pub struct W(crate::W<SPI_MEM_FLASH_SUS_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_FLASH_SUS_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_MEM_FLASH_SUS_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_FLASH_SUS_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_FLASH_PES_COMMAND` reader - Program/Erase suspend command."]
pub type SPI_MEM_FLASH_PES_COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_FLASH_PES_COMMAND` writer - Program/Erase suspend command."]
pub type SPI_MEM_FLASH_PES_COMMAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_FLASH_SUS_CMD_SPEC, 16, O, u16>;
#[doc = "Field `SPI_MEM_WAIT_PESR_COMMAND` reader - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
pub type SPI_MEM_WAIT_PESR_COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_WAIT_PESR_COMMAND` writer - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
pub type SPI_MEM_WAIT_PESR_COMMAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_FLASH_SUS_CMD_SPEC, 16, O, u16>;
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
    pub fn spi_mem_flash_pes_command(&mut self) -> SPI_MEM_FLASH_PES_COMMAND_W<0> {
        SPI_MEM_FLASH_PES_COMMAND_W::new(self)
    }
    #[doc = "Bits 16:31 - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wait_pesr_command(&mut self) -> SPI_MEM_WAIT_PESR_COMMAND_W<16> {
        SPI_MEM_WAIT_PESR_COMMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 flash suspend command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_flash_sus_cmd](index.html) module"]
pub struct SPI_MEM_FLASH_SUS_CMD_SPEC;
impl crate::RegisterSpec for SPI_MEM_FLASH_SUS_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_flash_sus_cmd::R](R) reader structure"]
impl crate::Readable for SPI_MEM_FLASH_SUS_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_flash_sus_cmd::W](W) writer structure"]
impl crate::Writable for SPI_MEM_FLASH_SUS_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_FLASH_SUS_CMD to value 0x0005_7575"]
impl crate::Resettable for SPI_MEM_FLASH_SUS_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0005_7575;
}

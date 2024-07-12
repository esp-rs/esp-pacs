#[doc = "Register `SRAM_DWR_CMD` reader"]
pub type R = crate::R<SRAM_DWR_CMD_SPEC>;
#[doc = "Register `SRAM_DWR_CMD` writer"]
pub type W = crate::W<SRAM_DWR_CMD_SPEC>;
#[doc = "Field `CACHE_SRAM_USR_WR_CMD_VALUE` reader - When SPI0 writes Ext_RAM, it is the command value of CMD phase."]
pub type CACHE_SRAM_USR_WR_CMD_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `CACHE_SRAM_USR_WR_CMD_VALUE` writer - When SPI0 writes Ext_RAM, it is the command value of CMD phase."]
pub type CACHE_SRAM_USR_WR_CMD_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CACHE_SRAM_USR_WR_CMD_BITLEN` reader - When SPI0 writes Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
pub type CACHE_SRAM_USR_WR_CMD_BITLEN_R = crate::FieldReader;
#[doc = "Field `CACHE_SRAM_USR_WR_CMD_BITLEN` writer - When SPI0 writes Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
pub type CACHE_SRAM_USR_WR_CMD_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - When SPI0 writes Ext_RAM, it is the command value of CMD phase."]
    #[inline(always)]
    pub fn cache_sram_usr_wr_cmd_value(&self) -> CACHE_SRAM_USR_WR_CMD_VALUE_R {
        CACHE_SRAM_USR_WR_CMD_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - When SPI0 writes Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn cache_sram_usr_wr_cmd_bitlen(&self) -> CACHE_SRAM_USR_WR_CMD_BITLEN_R {
        CACHE_SRAM_USR_WR_CMD_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_DWR_CMD")
            .field(
                "cache_sram_usr_wr_cmd_value",
                &self.cache_sram_usr_wr_cmd_value(),
            )
            .field(
                "cache_sram_usr_wr_cmd_bitlen",
                &self.cache_sram_usr_wr_cmd_bitlen(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - When SPI0 writes Ext_RAM, it is the command value of CMD phase."]
    #[inline(always)]
    #[must_use]
    pub fn cache_sram_usr_wr_cmd_value(
        &mut self,
    ) -> CACHE_SRAM_USR_WR_CMD_VALUE_W<SRAM_DWR_CMD_SPEC> {
        CACHE_SRAM_USR_WR_CMD_VALUE_W::new(self, 0)
    }
    #[doc = "Bits 28:31 - When SPI0 writes Ext_RAM, it is the length in bits of CMD phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn cache_sram_usr_wr_cmd_bitlen(
        &mut self,
    ) -> CACHE_SRAM_USR_WR_CMD_BITLEN_W<SRAM_DWR_CMD_SPEC> {
        CACHE_SRAM_USR_WR_CMD_BITLEN_W::new(self, 28)
    }
}
#[doc = "SPI0 external RAM DDR write command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_dwr_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_dwr_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_DWR_CMD_SPEC;
impl crate::RegisterSpec for SRAM_DWR_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_dwr_cmd::R`](R) reader structure"]
impl crate::Readable for SRAM_DWR_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_dwr_cmd::W`](W) writer structure"]
impl crate::Writable for SRAM_DWR_CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_DWR_CMD to value 0"]
impl crate::Resettable for SRAM_DWR_CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}

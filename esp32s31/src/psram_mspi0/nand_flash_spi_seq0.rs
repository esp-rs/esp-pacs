#[doc = "Register `NAND_FLASH_SPI_SEQ0` reader"]
pub type R = crate::R<NAND_FLASH_SPI_SEQ0_SPEC>;
#[doc = "Register `NAND_FLASH_SPI_SEQ0` writer"]
pub type W = crate::W<NAND_FLASH_SPI_SEQ0_SPEC>;
#[doc = "Field `NAND_FLASH_SEQ_TAIL_FLG0` reader - "]
pub type NAND_FLASH_SEQ_TAIL_FLG0_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_SEQ_TAIL_FLG0` writer - "]
pub type NAND_FLASH_SEQ_TAIL_FLG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAND_FLASH_SR_CHK_EN0` reader - "]
pub type NAND_FLASH_SR_CHK_EN0_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_SR_CHK_EN0` writer - "]
pub type NAND_FLASH_SR_CHK_EN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAND_FLASH_DIN_INDEX0` reader - "]
pub type NAND_FLASH_DIN_INDEX0_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_DIN_INDEX0` writer - "]
pub type NAND_FLASH_DIN_INDEX0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NAND_FLASH_ADDR_INDEX0` reader - "]
pub type NAND_FLASH_ADDR_INDEX0_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_ADDR_INDEX0` writer - "]
pub type NAND_FLASH_ADDR_INDEX0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NAND_FLASH_REQ_OR_CFG0` reader - "]
pub type NAND_FLASH_REQ_OR_CFG0_R = crate::BitReader;
#[doc = "Field `NAND_FLASH_REQ_OR_CFG0` writer - "]
pub type NAND_FLASH_REQ_OR_CFG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAND_FLASH_CMD_INDEX0` reader - "]
pub type NAND_FLASH_CMD_INDEX0_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_CMD_INDEX0` writer - "]
pub type NAND_FLASH_CMD_INDEX0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nand_flash_seq_tail_flg0(&self) -> NAND_FLASH_SEQ_TAIL_FLG0_R {
        NAND_FLASH_SEQ_TAIL_FLG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nand_flash_sr_chk_en0(&self) -> NAND_FLASH_SR_CHK_EN0_R {
        NAND_FLASH_SR_CHK_EN0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn nand_flash_din_index0(&self) -> NAND_FLASH_DIN_INDEX0_R {
        NAND_FLASH_DIN_INDEX0_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn nand_flash_addr_index0(&self) -> NAND_FLASH_ADDR_INDEX0_R {
        NAND_FLASH_ADDR_INDEX0_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn nand_flash_req_or_cfg0(&self) -> NAND_FLASH_REQ_OR_CFG0_R {
        NAND_FLASH_REQ_OR_CFG0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn nand_flash_cmd_index0(&self) -> NAND_FLASH_CMD_INDEX0_R {
        NAND_FLASH_CMD_INDEX0_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_SPI_SEQ0")
            .field("nand_flash_seq_tail_flg0", &self.nand_flash_seq_tail_flg0())
            .field("nand_flash_sr_chk_en0", &self.nand_flash_sr_chk_en0())
            .field("nand_flash_din_index0", &self.nand_flash_din_index0())
            .field("nand_flash_addr_index0", &self.nand_flash_addr_index0())
            .field("nand_flash_req_or_cfg0", &self.nand_flash_req_or_cfg0())
            .field("nand_flash_cmd_index0", &self.nand_flash_cmd_index0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nand_flash_seq_tail_flg0(
        &mut self,
    ) -> NAND_FLASH_SEQ_TAIL_FLG0_W<'_, NAND_FLASH_SPI_SEQ0_SPEC> {
        NAND_FLASH_SEQ_TAIL_FLG0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn nand_flash_sr_chk_en0(
        &mut self,
    ) -> NAND_FLASH_SR_CHK_EN0_W<'_, NAND_FLASH_SPI_SEQ0_SPEC> {
        NAND_FLASH_SR_CHK_EN0_W::new(self, 1)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn nand_flash_din_index0(
        &mut self,
    ) -> NAND_FLASH_DIN_INDEX0_W<'_, NAND_FLASH_SPI_SEQ0_SPEC> {
        NAND_FLASH_DIN_INDEX0_W::new(self, 2)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn nand_flash_addr_index0(
        &mut self,
    ) -> NAND_FLASH_ADDR_INDEX0_W<'_, NAND_FLASH_SPI_SEQ0_SPEC> {
        NAND_FLASH_ADDR_INDEX0_W::new(self, 6)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn nand_flash_req_or_cfg0(
        &mut self,
    ) -> NAND_FLASH_REQ_OR_CFG0_W<'_, NAND_FLASH_SPI_SEQ0_SPEC> {
        NAND_FLASH_REQ_OR_CFG0_W::new(self, 10)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn nand_flash_cmd_index0(
        &mut self,
    ) -> NAND_FLASH_CMD_INDEX0_W<'_, NAND_FLASH_SPI_SEQ0_SPEC> {
        NAND_FLASH_CMD_INDEX0_W::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nand_flash_spi_seq0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_SPI_SEQ0_SPEC;
impl crate::RegisterSpec for NAND_FLASH_SPI_SEQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_spi_seq0::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_SPI_SEQ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nand_flash_spi_seq0::W`](W) writer structure"]
impl crate::Writable for NAND_FLASH_SPI_SEQ0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NAND_FLASH_SPI_SEQ0 to value 0"]
impl crate::Resettable for NAND_FLASH_SPI_SEQ0_SPEC {}

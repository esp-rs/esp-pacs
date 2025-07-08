#[doc = "Register `MEM_CTRL2` reader"]
pub type R = crate::R<MEM_CTRL2_SPEC>;
#[doc = "Register `MEM_CTRL2` writer"]
pub type W = crate::W<MEM_CTRL2_SPEC>;
#[doc = "Field `MEM_CS_SETUP_TIME` reader - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
pub type MEM_CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `MEM_CS_SETUP_TIME` writer - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
pub type MEM_CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MEM_CS_HOLD_TIME` reader - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
pub type MEM_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `MEM_CS_HOLD_TIME` writer - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
pub type MEM_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MEM_ECC_CS_HOLD_TIME` reader - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash."]
pub type MEM_ECC_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `MEM_ECC_CS_HOLD_TIME` writer - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash."]
pub type MEM_ECC_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_ECC_SKIP_PAGE_CORNER` reader - 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash."]
pub type MEM_ECC_SKIP_PAGE_CORNER_R = crate::BitReader;
#[doc = "Field `MEM_ECC_SKIP_PAGE_CORNER` writer - 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash."]
pub type MEM_ECC_SKIP_PAGE_CORNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_ECC_16TO18_BYTE_EN` reader - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
pub type MEM_ECC_16TO18_BYTE_EN_R = crate::BitReader;
#[doc = "Field `MEM_ECC_16TO18_BYTE_EN` writer - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
pub type MEM_ECC_16TO18_BYTE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SPLIT_TRANS_EN` reader - Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
pub type MEM_SPLIT_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_SPLIT_TRANS_EN` writer - Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
pub type MEM_SPLIT_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type MEM_CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `MEM_CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type MEM_CS_HOLD_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MEM_SYNC_RESET` writer - The spi0_mst_st and spi0_slv_st will be reset."]
pub type MEM_SYNC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn mem_cs_setup_time(&self) -> MEM_CS_SETUP_TIME_R {
        MEM_CS_SETUP_TIME_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn mem_cs_hold_time(&self) -> MEM_CS_HOLD_TIME_R {
        MEM_CS_HOLD_TIME_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:12 - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash."]
    #[inline(always)]
    pub fn mem_ecc_cs_hold_time(&self) -> MEM_ECC_CS_HOLD_TIME_R {
        MEM_ECC_CS_HOLD_TIME_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash."]
    #[inline(always)]
    pub fn mem_ecc_skip_page_corner(&self) -> MEM_ECC_SKIP_PAGE_CORNER_R {
        MEM_ECC_SKIP_PAGE_CORNER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
    #[inline(always)]
    pub fn mem_ecc_16to18_byte_en(&self) -> MEM_ECC_16TO18_BYTE_EN_R {
        MEM_ECC_16TO18_BYTE_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
    #[inline(always)]
    pub fn mem_split_trans_en(&self) -> MEM_SPLIT_TRANS_EN_R {
        MEM_SPLIT_TRANS_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn mem_cs_hold_delay(&self) -> MEM_CS_HOLD_DELAY_R {
        MEM_CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CTRL2")
            .field("mem_cs_setup_time", &self.mem_cs_setup_time())
            .field("mem_cs_hold_time", &self.mem_cs_hold_time())
            .field("mem_ecc_cs_hold_time", &self.mem_ecc_cs_hold_time())
            .field("mem_ecc_skip_page_corner", &self.mem_ecc_skip_page_corner())
            .field("mem_ecc_16to18_byte_en", &self.mem_ecc_16to18_byte_en())
            .field("mem_split_trans_en", &self.mem_split_trans_en())
            .field("mem_cs_hold_delay", &self.mem_cs_hold_delay())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn mem_cs_setup_time(&mut self) -> MEM_CS_SETUP_TIME_W<MEM_CTRL2_SPEC> {
        MEM_CS_SETUP_TIME_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn mem_cs_hold_time(&mut self) -> MEM_CS_HOLD_TIME_W<MEM_CTRL2_SPEC> {
        MEM_CS_HOLD_TIME_W::new(self, 5)
    }
    #[doc = "Bits 10:12 - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash."]
    #[inline(always)]
    pub fn mem_ecc_cs_hold_time(&mut self) -> MEM_ECC_CS_HOLD_TIME_W<MEM_CTRL2_SPEC> {
        MEM_ECC_CS_HOLD_TIME_W::new(self, 10)
    }
    #[doc = "Bit 13 - 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash."]
    #[inline(always)]
    pub fn mem_ecc_skip_page_corner(&mut self) -> MEM_ECC_SKIP_PAGE_CORNER_W<MEM_CTRL2_SPEC> {
        MEM_ECC_SKIP_PAGE_CORNER_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
    #[inline(always)]
    pub fn mem_ecc_16to18_byte_en(&mut self) -> MEM_ECC_16TO18_BYTE_EN_W<MEM_CTRL2_SPEC> {
        MEM_ECC_16TO18_BYTE_EN_W::new(self, 14)
    }
    #[doc = "Bit 24 - Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
    #[inline(always)]
    pub fn mem_split_trans_en(&mut self) -> MEM_SPLIT_TRANS_EN_W<MEM_CTRL2_SPEC> {
        MEM_SPLIT_TRANS_EN_W::new(self, 24)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn mem_cs_hold_delay(&mut self) -> MEM_CS_HOLD_DELAY_W<MEM_CTRL2_SPEC> {
        MEM_CS_HOLD_DELAY_W::new(self, 25)
    }
    #[doc = "Bit 31 - The spi0_mst_st and spi0_slv_st will be reset."]
    #[inline(always)]
    pub fn mem_sync_reset(&mut self) -> MEM_SYNC_RESET_W<MEM_CTRL2_SPEC> {
        MEM_SYNC_RESET_W::new(self, 31)
    }
}
#[doc = "SPI0 control2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CTRL2_SPEC;
impl crate::RegisterSpec for MEM_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ctrl2::R`](R) reader structure"]
impl crate::Readable for MEM_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_ctrl2::W`](W) writer structure"]
impl crate::Writable for MEM_CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CTRL2 to value 0x2c21"]
impl crate::Resettable for MEM_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x2c21;
}

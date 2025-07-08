#[doc = "Register `MEM_SRAM_CMD` reader"]
pub type R = crate::R<MEM_SRAM_CMD_SPEC>;
#[doc = "Register `MEM_SRAM_CMD` writer"]
pub type W = crate::W<MEM_SRAM_CMD_SPEC>;
#[doc = "Field `MEM_SCLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
pub type MEM_SCLK_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_SCLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
pub type MEM_SCLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_SWB_MODE` reader - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type MEM_SWB_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_SWB_MODE` writer - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type MEM_SWB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_SDIN_DUAL` reader - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type MEM_SDIN_DUAL_R = crate::BitReader;
#[doc = "Field `MEM_SDIN_DUAL` writer - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type MEM_SDIN_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SDOUT_DUAL` reader - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type MEM_SDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `MEM_SDOUT_DUAL` writer - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type MEM_SDOUT_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SADDR_DUAL` reader - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type MEM_SADDR_DUAL_R = crate::BitReader;
#[doc = "Field `MEM_SADDR_DUAL` writer - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type MEM_SADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SDIN_QUAD` reader - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type MEM_SDIN_QUAD_R = crate::BitReader;
#[doc = "Field `MEM_SDIN_QUAD` writer - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type MEM_SDIN_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SDOUT_QUAD` reader - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type MEM_SDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `MEM_SDOUT_QUAD` writer - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type MEM_SDOUT_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SADDR_QUAD` reader - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type MEM_SADDR_QUAD_R = crate::BitReader;
#[doc = "Field `MEM_SADDR_QUAD` writer - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type MEM_SADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SCMD_QUAD` reader - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type MEM_SCMD_QUAD_R = crate::BitReader;
#[doc = "Field `MEM_SCMD_QUAD` writer - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type MEM_SCMD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SDIN_OCT` reader - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
pub type MEM_SDIN_OCT_R = crate::BitReader;
#[doc = "Field `MEM_SDIN_OCT` writer - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
pub type MEM_SDIN_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SDOUT_OCT` reader - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
pub type MEM_SDOUT_OCT_R = crate::BitReader;
#[doc = "Field `MEM_SDOUT_OCT` writer - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
pub type MEM_SDOUT_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SADDR_OCT` reader - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
pub type MEM_SADDR_OCT_R = crate::BitReader;
#[doc = "Field `MEM_SADDR_OCT` writer - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
pub type MEM_SADDR_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SCMD_OCT` reader - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
pub type MEM_SCMD_OCT_R = crate::BitReader;
#[doc = "Field `MEM_SCMD_OCT` writer - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
pub type MEM_SCMD_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SDUMMY_RIN` reader - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type MEM_SDUMMY_RIN_R = crate::BitReader;
#[doc = "Field `MEM_SDUMMY_RIN` writer - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type MEM_SDUMMY_RIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SDUMMY_WOUT` reader - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type MEM_SDUMMY_WOUT_R = crate::BitReader;
#[doc = "Field `MEM_SDUMMY_WOUT` writer - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type MEM_SDUMMY_WOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_WDUMMY_DQS_ALWAYS_OUT` reader - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
pub type SMEM_WDUMMY_DQS_ALWAYS_OUT_R = crate::BitReader;
#[doc = "Field `SMEM_WDUMMY_DQS_ALWAYS_OUT` writer - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
pub type SMEM_WDUMMY_DQS_ALWAYS_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_WDUMMY_ALWAYS_OUT` reader - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
pub type SMEM_WDUMMY_ALWAYS_OUT_R = crate::BitReader;
#[doc = "Field `SMEM_WDUMMY_ALWAYS_OUT` writer - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
pub type SMEM_WDUMMY_ALWAYS_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SDIN_HEX` reader - For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable."]
pub type MEM_SDIN_HEX_R = crate::BitReader;
#[doc = "Field `MEM_SDOUT_HEX` reader - For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable."]
pub type MEM_SDOUT_HEX_R = crate::BitReader;
#[doc = "Field `SMEM_DQS_IE_ALWAYS_ON` reader - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
pub type SMEM_DQS_IE_ALWAYS_ON_R = crate::BitReader;
#[doc = "Field `SMEM_DQS_IE_ALWAYS_ON` writer - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
pub type SMEM_DQS_IE_ALWAYS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_DATA_IE_ALWAYS_ON` reader - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
pub type SMEM_DATA_IE_ALWAYS_ON_R = crate::BitReader;
#[doc = "Field `SMEM_DATA_IE_ALWAYS_ON` writer - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
pub type SMEM_DATA_IE_ALWAYS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
    #[inline(always)]
    pub fn mem_sclk_mode(&self) -> MEM_SCLK_MODE_R {
        MEM_SCLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9 - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn mem_swb_mode(&self) -> MEM_SWB_MODE_R {
        MEM_SWB_MODE_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10 - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn mem_sdin_dual(&self) -> MEM_SDIN_DUAL_R {
        MEM_SDIN_DUAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn mem_sdout_dual(&self) -> MEM_SDOUT_DUAL_R {
        MEM_SDOUT_DUAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn mem_saddr_dual(&self) -> MEM_SADDR_DUAL_R {
        MEM_SADDR_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn mem_sdin_quad(&self) -> MEM_SDIN_QUAD_R {
        MEM_SDIN_QUAD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn mem_sdout_quad(&self) -> MEM_SDOUT_QUAD_R {
        MEM_SDOUT_QUAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn mem_saddr_quad(&self) -> MEM_SADDR_QUAD_R {
        MEM_SADDR_QUAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn mem_scmd_quad(&self) -> MEM_SCMD_QUAD_R {
        MEM_SCMD_QUAD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_sdin_oct(&self) -> MEM_SDIN_OCT_R {
        MEM_SDIN_OCT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_sdout_oct(&self) -> MEM_SDOUT_OCT_R {
        MEM_SDOUT_OCT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_saddr_oct(&self) -> MEM_SADDR_OCT_R {
        MEM_SADDR_OCT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_scmd_oct(&self) -> MEM_SCMD_OCT_R {
        MEM_SCMD_OCT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn mem_sdummy_rin(&self) -> MEM_SDUMMY_RIN_R {
        MEM_SDUMMY_RIN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn mem_sdummy_wout(&self) -> MEM_SDUMMY_WOUT_R {
        MEM_SDUMMY_WOUT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
    #[inline(always)]
    pub fn smem_wdummy_dqs_always_out(&self) -> SMEM_WDUMMY_DQS_ALWAYS_OUT_R {
        SMEM_WDUMMY_DQS_ALWAYS_OUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
    #[inline(always)]
    pub fn smem_wdummy_always_out(&self) -> SMEM_WDUMMY_ALWAYS_OUT_R {
        SMEM_WDUMMY_ALWAYS_OUT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_sdin_hex(&self) -> MEM_SDIN_HEX_R {
        MEM_SDIN_HEX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_sdout_hex(&self) -> MEM_SDOUT_HEX_R {
        MEM_SDOUT_HEX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
    #[inline(always)]
    pub fn smem_dqs_ie_always_on(&self) -> SMEM_DQS_IE_ALWAYS_ON_R {
        SMEM_DQS_IE_ALWAYS_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
    #[inline(always)]
    pub fn smem_data_ie_always_on(&self) -> SMEM_DATA_IE_ALWAYS_ON_R {
        SMEM_DATA_IE_ALWAYS_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_SRAM_CMD")
            .field("mem_sclk_mode", &self.mem_sclk_mode())
            .field("mem_swb_mode", &self.mem_swb_mode())
            .field("mem_sdin_dual", &self.mem_sdin_dual())
            .field("mem_sdout_dual", &self.mem_sdout_dual())
            .field("mem_saddr_dual", &self.mem_saddr_dual())
            .field("mem_sdin_quad", &self.mem_sdin_quad())
            .field("mem_sdout_quad", &self.mem_sdout_quad())
            .field("mem_saddr_quad", &self.mem_saddr_quad())
            .field("mem_scmd_quad", &self.mem_scmd_quad())
            .field("mem_sdin_oct", &self.mem_sdin_oct())
            .field("mem_sdout_oct", &self.mem_sdout_oct())
            .field("mem_saddr_oct", &self.mem_saddr_oct())
            .field("mem_scmd_oct", &self.mem_scmd_oct())
            .field("mem_sdummy_rin", &self.mem_sdummy_rin())
            .field("mem_sdummy_wout", &self.mem_sdummy_wout())
            .field(
                "smem_wdummy_dqs_always_out",
                &self.smem_wdummy_dqs_always_out(),
            )
            .field("smem_wdummy_always_out", &self.smem_wdummy_always_out())
            .field("mem_sdin_hex", &self.mem_sdin_hex())
            .field("mem_sdout_hex", &self.mem_sdout_hex())
            .field("smem_dqs_ie_always_on", &self.smem_dqs_ie_always_on())
            .field("smem_data_ie_always_on", &self.smem_data_ie_always_on())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
    #[inline(always)]
    pub fn mem_sclk_mode(&mut self) -> MEM_SCLK_MODE_W<MEM_SRAM_CMD_SPEC> {
        MEM_SCLK_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:9 - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn mem_swb_mode(&mut self) -> MEM_SWB_MODE_W<MEM_SRAM_CMD_SPEC> {
        MEM_SWB_MODE_W::new(self, 2)
    }
    #[doc = "Bit 10 - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn mem_sdin_dual(&mut self) -> MEM_SDIN_DUAL_W<MEM_SRAM_CMD_SPEC> {
        MEM_SDIN_DUAL_W::new(self, 10)
    }
    #[doc = "Bit 11 - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn mem_sdout_dual(&mut self) -> MEM_SDOUT_DUAL_W<MEM_SRAM_CMD_SPEC> {
        MEM_SDOUT_DUAL_W::new(self, 11)
    }
    #[doc = "Bit 12 - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn mem_saddr_dual(&mut self) -> MEM_SADDR_DUAL_W<MEM_SRAM_CMD_SPEC> {
        MEM_SADDR_DUAL_W::new(self, 12)
    }
    #[doc = "Bit 14 - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn mem_sdin_quad(&mut self) -> MEM_SDIN_QUAD_W<MEM_SRAM_CMD_SPEC> {
        MEM_SDIN_QUAD_W::new(self, 14)
    }
    #[doc = "Bit 15 - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn mem_sdout_quad(&mut self) -> MEM_SDOUT_QUAD_W<MEM_SRAM_CMD_SPEC> {
        MEM_SDOUT_QUAD_W::new(self, 15)
    }
    #[doc = "Bit 16 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn mem_saddr_quad(&mut self) -> MEM_SADDR_QUAD_W<MEM_SRAM_CMD_SPEC> {
        MEM_SADDR_QUAD_W::new(self, 16)
    }
    #[doc = "Bit 17 - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn mem_scmd_quad(&mut self) -> MEM_SCMD_QUAD_W<MEM_SRAM_CMD_SPEC> {
        MEM_SCMD_QUAD_W::new(self, 17)
    }
    #[doc = "Bit 18 - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_sdin_oct(&mut self) -> MEM_SDIN_OCT_W<MEM_SRAM_CMD_SPEC> {
        MEM_SDIN_OCT_W::new(self, 18)
    }
    #[doc = "Bit 19 - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_sdout_oct(&mut self) -> MEM_SDOUT_OCT_W<MEM_SRAM_CMD_SPEC> {
        MEM_SDOUT_OCT_W::new(self, 19)
    }
    #[doc = "Bit 20 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_saddr_oct(&mut self) -> MEM_SADDR_OCT_W<MEM_SRAM_CMD_SPEC> {
        MEM_SADDR_OCT_W::new(self, 20)
    }
    #[doc = "Bit 21 - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_scmd_oct(&mut self) -> MEM_SCMD_OCT_W<MEM_SRAM_CMD_SPEC> {
        MEM_SCMD_OCT_W::new(self, 21)
    }
    #[doc = "Bit 22 - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn mem_sdummy_rin(&mut self) -> MEM_SDUMMY_RIN_W<MEM_SRAM_CMD_SPEC> {
        MEM_SDUMMY_RIN_W::new(self, 22)
    }
    #[doc = "Bit 23 - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn mem_sdummy_wout(&mut self) -> MEM_SDUMMY_WOUT_W<MEM_SRAM_CMD_SPEC> {
        MEM_SDUMMY_WOUT_W::new(self, 23)
    }
    #[doc = "Bit 24 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
    #[inline(always)]
    pub fn smem_wdummy_dqs_always_out(
        &mut self,
    ) -> SMEM_WDUMMY_DQS_ALWAYS_OUT_W<MEM_SRAM_CMD_SPEC> {
        SMEM_WDUMMY_DQS_ALWAYS_OUT_W::new(self, 24)
    }
    #[doc = "Bit 25 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
    #[inline(always)]
    pub fn smem_wdummy_always_out(&mut self) -> SMEM_WDUMMY_ALWAYS_OUT_W<MEM_SRAM_CMD_SPEC> {
        SMEM_WDUMMY_ALWAYS_OUT_W::new(self, 25)
    }
    #[doc = "Bit 30 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
    #[inline(always)]
    pub fn smem_dqs_ie_always_on(&mut self) -> SMEM_DQS_IE_ALWAYS_ON_W<MEM_SRAM_CMD_SPEC> {
        SMEM_DQS_IE_ALWAYS_ON_W::new(self, 30)
    }
    #[doc = "Bit 31 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
    #[inline(always)]
    pub fn smem_data_ie_always_on(&mut self) -> SMEM_DATA_IE_ALWAYS_ON_W<MEM_SRAM_CMD_SPEC> {
        SMEM_DATA_IE_ALWAYS_ON_W::new(self, 31)
    }
}
#[doc = "SPI0 external RAM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_sram_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_sram_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_SRAM_CMD_SPEC;
impl crate::RegisterSpec for MEM_SRAM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_sram_cmd::R`](R) reader structure"]
impl crate::Readable for MEM_SRAM_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_sram_cmd::W`](W) writer structure"]
impl crate::Writable for MEM_SRAM_CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_SRAM_CMD to value 0x80c0_0000"]
impl crate::Resettable for MEM_SRAM_CMD_SPEC {
    const RESET_VALUE: u32 = 0x80c0_0000;
}

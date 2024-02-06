#[doc = "Register `SPI_MEM_FLASH_SUS_CTRL` reader"]
pub type R = crate::R<SPI_MEM_FLASH_SUS_CTRL_SPEC>;
#[doc = "Register `SPI_MEM_FLASH_SUS_CTRL` writer"]
pub type W = crate::W<SPI_MEM_FLASH_SUS_CTRL_SPEC>;
#[doc = "Field `SPI_MEM_FLASH_PER` reader - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_PER_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_PER` writer - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_PES` reader - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_PES_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_PES` writer - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type SPI_MEM_FLASH_PES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_PER_WAIT_EN` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
pub type SPI_MEM_FLASH_PER_WAIT_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_PER_WAIT_EN` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
pub type SPI_MEM_FLASH_PER_WAIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_PES_WAIT_EN` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
pub type SPI_MEM_FLASH_PES_WAIT_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_PES_WAIT_EN` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
pub type SPI_MEM_FLASH_PES_WAIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_PES_PER_EN` reader - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
pub type SPI_MEM_PES_PER_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PES_PER_EN` writer - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
pub type SPI_MEM_PES_PER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_FLASH_PES_EN` reader - Set this bit to enable Auto-suspending function."]
pub type SPI_MEM_FLASH_PES_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FLASH_PES_EN` writer - Set this bit to enable Auto-suspending function."]
pub type SPI_MEM_FLASH_PES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_PESR_END_MSK` reader - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\](only status_in\\[7:0\\] is valid when only one byte of data is read out, status_in\\[15:0\\] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]^ SPI_MEM_PESR_END_MSK\\[15:0\\]."]
pub type SPI_MEM_PESR_END_MSK_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_PESR_END_MSK` writer - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\](only status_in\\[7:0\\] is valid when only one byte of data is read out, status_in\\[15:0\\] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]^ SPI_MEM_PESR_END_MSK\\[15:0\\]."]
pub type SPI_MEM_PESR_END_MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SPI_FMEM_RD_SUS_2B` reader - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
pub type SPI_FMEM_RD_SUS_2B_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_RD_SUS_2B` writer - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
pub type SPI_FMEM_RD_SUS_2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_PER_END_EN` reader - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
pub type SPI_MEM_PER_END_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PER_END_EN` writer - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
pub type SPI_MEM_PER_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_PES_END_EN` reader - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
pub type SPI_MEM_PES_END_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PES_END_EN` writer - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
pub type SPI_MEM_PES_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_SUS_TIMEOUT_CNT` reader - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\] times, it will be treated as check pass."]
pub type SPI_MEM_SUS_TIMEOUT_CNT_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_SUS_TIMEOUT_CNT` writer - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\] times, it will be treated as check pass."]
pub type SPI_MEM_SUS_TIMEOUT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_per(&self) -> SPI_MEM_FLASH_PER_R {
        SPI_MEM_FLASH_PER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_flash_pes(&self) -> SPI_MEM_FLASH_PES_R {
        SPI_MEM_FLASH_PES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
    #[inline(always)]
    pub fn spi_mem_flash_per_wait_en(&self) -> SPI_MEM_FLASH_PER_WAIT_EN_R {
        SPI_MEM_FLASH_PER_WAIT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
    #[inline(always)]
    pub fn spi_mem_flash_pes_wait_en(&self) -> SPI_MEM_FLASH_PES_WAIT_EN_R {
        SPI_MEM_FLASH_PES_WAIT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
    #[inline(always)]
    pub fn spi_mem_pes_per_en(&self) -> SPI_MEM_PES_PER_EN_R {
        SPI_MEM_PES_PER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable Auto-suspending function."]
    #[inline(always)]
    pub fn spi_mem_flash_pes_en(&self) -> SPI_MEM_FLASH_PES_EN_R {
        SPI_MEM_FLASH_PES_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:21 - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\](only status_in\\[7:0\\] is valid when only one byte of data is read out, status_in\\[15:0\\] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]^ SPI_MEM_PESR_END_MSK\\[15:0\\]."]
    #[inline(always)]
    pub fn spi_mem_pesr_end_msk(&self) -> SPI_MEM_PESR_END_MSK_R {
        SPI_MEM_PESR_END_MSK_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 22 - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
    #[inline(always)]
    pub fn spi_fmem_rd_sus_2b(&self) -> SPI_FMEM_RD_SUS_2B_R {
        SPI_FMEM_RD_SUS_2B_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    pub fn spi_mem_per_end_en(&self) -> SPI_MEM_PER_END_EN_R {
        SPI_MEM_PER_END_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    pub fn spi_mem_pes_end_en(&self) -> SPI_MEM_PES_END_EN_R {
        SPI_MEM_PES_END_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\] times, it will be treated as check pass."]
    #[inline(always)]
    pub fn spi_mem_sus_timeout_cnt(&self) -> SPI_MEM_SUS_TIMEOUT_CNT_R {
        SPI_MEM_SUS_TIMEOUT_CNT_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_FLASH_SUS_CTRL")
            .field(
                "spi_mem_flash_per",
                &format_args!("{}", self.spi_mem_flash_per().bit()),
            )
            .field(
                "spi_mem_flash_pes",
                &format_args!("{}", self.spi_mem_flash_pes().bit()),
            )
            .field(
                "spi_mem_flash_per_wait_en",
                &format_args!("{}", self.spi_mem_flash_per_wait_en().bit()),
            )
            .field(
                "spi_mem_flash_pes_wait_en",
                &format_args!("{}", self.spi_mem_flash_pes_wait_en().bit()),
            )
            .field(
                "spi_mem_pes_per_en",
                &format_args!("{}", self.spi_mem_pes_per_en().bit()),
            )
            .field(
                "spi_mem_flash_pes_en",
                &format_args!("{}", self.spi_mem_flash_pes_en().bit()),
            )
            .field(
                "spi_mem_pesr_end_msk",
                &format_args!("{}", self.spi_mem_pesr_end_msk().bits()),
            )
            .field(
                "spi_fmem_rd_sus_2b",
                &format_args!("{}", self.spi_fmem_rd_sus_2b().bit()),
            )
            .field(
                "spi_mem_per_end_en",
                &format_args!("{}", self.spi_mem_per_end_en().bit()),
            )
            .field(
                "spi_mem_pes_end_en",
                &format_args!("{}", self.spi_mem_pes_end_en().bit()),
            )
            .field(
                "spi_mem_sus_timeout_cnt",
                &format_args!("{}", self.spi_mem_sus_timeout_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_per(&mut self) -> SPI_MEM_FLASH_PER_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_MEM_FLASH_PER_W::new(self, 0)
    }
    #[doc = "Bit 1 - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_pes(&mut self) -> SPI_MEM_FLASH_PES_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_MEM_FLASH_PES_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_per_wait_en(
        &mut self,
    ) -> SPI_MEM_FLASH_PER_WAIT_EN_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_MEM_FLASH_PER_WAIT_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_pes_wait_en(
        &mut self,
    ) -> SPI_MEM_FLASH_PES_WAIT_EN_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_MEM_FLASH_PES_WAIT_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_pes_per_en(&mut self) -> SPI_MEM_PES_PER_EN_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_MEM_PES_PER_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable Auto-suspending function."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_flash_pes_en(&mut self) -> SPI_MEM_FLASH_PES_EN_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_MEM_FLASH_PES_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:21 - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\](only status_in\\[7:0\\] is valid when only one byte of data is read out, status_in\\[15:0\\] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]^ SPI_MEM_PESR_END_MSK\\[15:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_pesr_end_msk(&mut self) -> SPI_MEM_PESR_END_MSK_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_MEM_PESR_END_MSK_W::new(self, 6)
    }
    #[doc = "Bit 22 - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_rd_sus_2b(&mut self) -> SPI_FMEM_RD_SUS_2B_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_FMEM_RD_SUS_2B_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_per_end_en(&mut self) -> SPI_MEM_PER_END_EN_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_MEM_PER_END_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_pes_end_en(&mut self) -> SPI_MEM_PES_END_EN_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_MEM_PES_END_EN_W::new(self, 24)
    }
    #[doc = "Bits 25:31 - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\] times, it will be treated as check pass."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_sus_timeout_cnt(
        &mut self,
    ) -> SPI_MEM_SUS_TIMEOUT_CNT_W<SPI_MEM_FLASH_SUS_CTRL_SPEC> {
        SPI_MEM_SUS_TIMEOUT_CNT_W::new(self, 25)
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
#[doc = "SPI1 flash suspend control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_flash_sus_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_flash_sus_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_FLASH_SUS_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_FLASH_SUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_flash_sus_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_FLASH_SUS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_flash_sus_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_FLASH_SUS_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_FLASH_SUS_CTRL to value 0x0800_2000"]
impl crate::Resettable for SPI_MEM_FLASH_SUS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0800_2000;
}

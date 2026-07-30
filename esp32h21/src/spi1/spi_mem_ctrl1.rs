#[doc = "Register `SPI_MEM_CTRL1` reader"]
pub type R = crate::R<SPI_MEM_CTRL1_SPEC>;
#[doc = "Register `SPI_MEM_CTRL1` writer"]
pub type W = crate::W<SPI_MEM_CTRL1_SPEC>;
#[doc = "Field `SPI_MEM_CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type SPI_MEM_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type SPI_MEM_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_CS_HOLD_DLY_RES` reader - After RES/DP/HPM/PES command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles."]
pub type SPI_MEM_CS_HOLD_DLY_RES_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_CS_HOLD_DLY_RES` writer - After RES/DP/HPM/PES command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles."]
pub type SPI_MEM_CS_HOLD_DLY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SPI_MEM_CS_HOLD_DLY_PER` reader - After PER command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DLY_PER\\[9:0\\] * 128) SPI_CLK cycles."]
pub type SPI_MEM_CS_HOLD_DLY_PER_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_CS_HOLD_DLY_PER` writer - After PER command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DLY_PER\\[9:0\\] * 128) SPI_CLK cycles."]
pub type SPI_MEM_CS_HOLD_DLY_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn spi_mem_clk_mode(&self) -> SPI_MEM_CLK_MODE_R {
        SPI_MEM_CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM/PES command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_dly_res(&self) -> SPI_MEM_CS_HOLD_DLY_RES_R {
        SPI_MEM_CS_HOLD_DLY_RES_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - After PER command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DLY_PER\\[9:0\\] * 128) SPI_CLK cycles."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_dly_per(&self) -> SPI_MEM_CS_HOLD_DLY_PER_R {
        SPI_MEM_CS_HOLD_DLY_PER_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CTRL1")
            .field("spi_mem_clk_mode", &self.spi_mem_clk_mode())
            .field("spi_mem_cs_hold_dly_res", &self.spi_mem_cs_hold_dly_res())
            .field("spi_mem_cs_hold_dly_per", &self.spi_mem_cs_hold_dly_per())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn spi_mem_clk_mode(&mut self) -> SPI_MEM_CLK_MODE_W<'_, SPI_MEM_CTRL1_SPEC> {
        SPI_MEM_CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM/PES command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_dly_res(&mut self) -> SPI_MEM_CS_HOLD_DLY_RES_W<'_, SPI_MEM_CTRL1_SPEC> {
        SPI_MEM_CS_HOLD_DLY_RES_W::new(self, 2)
    }
    #[doc = "Bits 12:21 - After PER command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DLY_PER\\[9:0\\] * 128) SPI_CLK cycles."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_dly_per(&mut self) -> SPI_MEM_CS_HOLD_DLY_PER_W<'_, SPI_MEM_CTRL1_SPEC> {
        SPI_MEM_CS_HOLD_DLY_PER_W::new(self, 12)
    }
}
#[doc = "SPI1 control1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_CTRL1_SPEC;
impl crate::RegisterSpec for SPI_MEM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_ctrl1::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_ctrl1::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_CTRL1 to value 0x003f_fffc"]
impl crate::Resettable for SPI_MEM_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x003f_fffc;
}

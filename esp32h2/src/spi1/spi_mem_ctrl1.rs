#[doc = "Register `SPI_MEM_CTRL1` reader"]
pub type R = crate::R<SPI_MEM_CTRL1_SPEC>;
#[doc = "Register `SPI_MEM_CTRL1` writer"]
pub type W = crate::W<SPI_MEM_CTRL1_SPEC>;
#[doc = "Field `SPI_MEM_CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type SPI_MEM_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type SPI_MEM_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_CS_HOLD_DLY_RES` reader - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 512) SPI_CLK cycles."]
pub type SPI_MEM_CS_HOLD_DLY_RES_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_CS_HOLD_DLY_RES` writer - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 512) SPI_CLK cycles."]
pub type SPI_MEM_CS_HOLD_DLY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn spi_mem_clk_mode(&self) -> SPI_MEM_CLK_MODE_R {
        SPI_MEM_CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 512) SPI_CLK cycles."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_dly_res(&self) -> SPI_MEM_CS_HOLD_DLY_RES_R {
        SPI_MEM_CS_HOLD_DLY_RES_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CTRL1")
            .field(
                "spi_mem_clk_mode",
                &format_args!("{}", self.spi_mem_clk_mode().bits()),
            )
            .field(
                "spi_mem_cs_hold_dly_res",
                &format_args!("{}", self.spi_mem_cs_hold_dly_res().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_clk_mode(&mut self) -> SPI_MEM_CLK_MODE_W<SPI_MEM_CTRL1_SPEC> {
        SPI_MEM_CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 512) SPI_CLK cycles."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cs_hold_dly_res(&mut self) -> SPI_MEM_CS_HOLD_DLY_RES_W<SPI_MEM_CTRL1_SPEC> {
        SPI_MEM_CS_HOLD_DLY_RES_W::new(self, 2)
    }
}
#[doc = "SPI1 control1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_CTRL1_SPEC;
impl crate::RegisterSpec for SPI_MEM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_ctrl1::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_ctrl1::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CTRL1 to value 0x0ffc"]
impl crate::Resettable for SPI_MEM_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x0ffc;
}

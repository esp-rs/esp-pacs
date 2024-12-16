#[doc = "Register `ANA_PLL_CTRL0` reader"]
pub type R = crate::R<ANA_PLL_CTRL0_SPEC>;
#[doc = "Register `ANA_PLL_CTRL0` writer"]
pub type W = crate::W<ANA_PLL_CTRL0_SPEC>;
#[doc = "Field `PLLA_CAL_END` reader - Reserved"]
pub type PLLA_CAL_END_R = crate::BitReader;
#[doc = "Field `PLLA_CAL_STOP` reader - Reserved"]
pub type PLLA_CAL_STOP_R = crate::BitReader;
#[doc = "Field `PLLA_CAL_STOP` writer - Reserved"]
pub type PLLA_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_PLL_CAL_END` reader - Reserved"]
pub type CPU_PLL_CAL_END_R = crate::BitReader;
#[doc = "Field `CPU_PLL_CAL_STOP` reader - Reserved"]
pub type CPU_PLL_CAL_STOP_R = crate::BitReader;
#[doc = "Field `CPU_PLL_CAL_STOP` writer - Reserved"]
pub type CPU_PLL_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PLL_CAL_END` reader - Reserved"]
pub type SDIO_PLL_CAL_END_R = crate::BitReader;
#[doc = "Field `SDIO_PLL_CAL_STOP` reader - Reserved"]
pub type SDIO_PLL_CAL_STOP_R = crate::BitReader;
#[doc = "Field `SDIO_PLL_CAL_STOP` writer - Reserved"]
pub type SDIO_PLL_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_PLL_CAL_END` reader - Reserved"]
pub type SYS_PLL_CAL_END_R = crate::BitReader;
#[doc = "Field `SYS_PLL_CAL_STOP` reader - Reserved"]
pub type SYS_PLL_CAL_STOP_R = crate::BitReader;
#[doc = "Field `SYS_PLL_CAL_STOP` writer - Reserved"]
pub type SYS_PLL_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSPI_CAL_END` reader - Reserved"]
pub type MSPI_CAL_END_R = crate::BitReader;
#[doc = "Field `MSPI_CAL_STOP` reader - Reserved"]
pub type MSPI_CAL_STOP_R = crate::BitReader;
#[doc = "Field `MSPI_CAL_STOP` writer - Reserved"]
pub type MSPI_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn plla_cal_end(&self) -> PLLA_CAL_END_R {
        PLLA_CAL_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn plla_cal_stop(&self) -> PLLA_CAL_STOP_R {
        PLLA_CAL_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn cpu_pll_cal_end(&self) -> CPU_PLL_CAL_END_R {
        CPU_PLL_CAL_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn cpu_pll_cal_stop(&self) -> CPU_PLL_CAL_STOP_R {
        CPU_PLL_CAL_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn sdio_pll_cal_end(&self) -> SDIO_PLL_CAL_END_R {
        SDIO_PLL_CAL_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn sdio_pll_cal_stop(&self) -> SDIO_PLL_CAL_STOP_R {
        SDIO_PLL_CAL_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn sys_pll_cal_end(&self) -> SYS_PLL_CAL_END_R {
        SYS_PLL_CAL_END_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn sys_pll_cal_stop(&self) -> SYS_PLL_CAL_STOP_R {
        SYS_PLL_CAL_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn mspi_cal_end(&self) -> MSPI_CAL_END_R {
        MSPI_CAL_END_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn mspi_cal_stop(&self) -> MSPI_CAL_STOP_R {
        MSPI_CAL_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_PLL_CTRL0")
            .field("plla_cal_end", &self.plla_cal_end())
            .field("plla_cal_stop", &self.plla_cal_stop())
            .field("cpu_pll_cal_end", &self.cpu_pll_cal_end())
            .field("cpu_pll_cal_stop", &self.cpu_pll_cal_stop())
            .field("sdio_pll_cal_end", &self.sdio_pll_cal_end())
            .field("sdio_pll_cal_stop", &self.sdio_pll_cal_stop())
            .field("sys_pll_cal_end", &self.sys_pll_cal_end())
            .field("sys_pll_cal_stop", &self.sys_pll_cal_stop())
            .field("mspi_cal_end", &self.mspi_cal_end())
            .field("mspi_cal_stop", &self.mspi_cal_stop())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn plla_cal_stop(&mut self) -> PLLA_CAL_STOP_W<ANA_PLL_CTRL0_SPEC> {
        PLLA_CAL_STOP_W::new(self, 1)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn cpu_pll_cal_stop(&mut self) -> CPU_PLL_CAL_STOP_W<ANA_PLL_CTRL0_SPEC> {
        CPU_PLL_CAL_STOP_W::new(self, 3)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn sdio_pll_cal_stop(&mut self) -> SDIO_PLL_CAL_STOP_W<ANA_PLL_CTRL0_SPEC> {
        SDIO_PLL_CAL_STOP_W::new(self, 5)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn sys_pll_cal_stop(&mut self) -> SYS_PLL_CAL_STOP_W<ANA_PLL_CTRL0_SPEC> {
        SYS_PLL_CAL_STOP_W::new(self, 7)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn mspi_cal_stop(&mut self) -> MSPI_CAL_STOP_W<ANA_PLL_CTRL0_SPEC> {
        MSPI_CAL_STOP_W::new(self, 9)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_pll_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_pll_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_PLL_CTRL0_SPEC;
impl crate::RegisterSpec for ANA_PLL_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_pll_ctrl0::R`](R) reader structure"]
impl crate::Readable for ANA_PLL_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_pll_ctrl0::W`](W) writer structure"]
impl crate::Writable for ANA_PLL_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_PLL_CTRL0 to value 0"]
impl crate::Resettable for ANA_PLL_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}

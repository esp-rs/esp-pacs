#[doc = "Register `ANA_PLL_CTRL0` reader"]
pub type R = crate::R<ANA_PLL_CTRL0_SPEC>;
#[doc = "Register `ANA_PLL_CTRL0` writer"]
pub type W = crate::W<ANA_PLL_CTRL0_SPEC>;
#[doc = "Field `REG_PLLA_CAL_END` reader - need_des"]
pub type REG_PLLA_CAL_END_R = crate::BitReader;
#[doc = "Field `REG_PLLA_CAL_STOP` reader - need_des"]
pub type REG_PLLA_CAL_STOP_R = crate::BitReader;
#[doc = "Field `REG_PLLA_CAL_STOP` writer - need_des"]
pub type REG_PLLA_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_CPU_PLL_CAL_END` reader - need_des"]
pub type REG_CPU_PLL_CAL_END_R = crate::BitReader;
#[doc = "Field `REG_CPU_PLL_CAL_STOP` reader - need_des"]
pub type REG_CPU_PLL_CAL_STOP_R = crate::BitReader;
#[doc = "Field `REG_CPU_PLL_CAL_STOP` writer - need_des"]
pub type REG_CPU_PLL_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDIO_PLL_CAL_END` reader - need_des"]
pub type REG_SDIO_PLL_CAL_END_R = crate::BitReader;
#[doc = "Field `REG_SDIO_PLL_CAL_STOP` reader - need_des"]
pub type REG_SDIO_PLL_CAL_STOP_R = crate::BitReader;
#[doc = "Field `REG_SDIO_PLL_CAL_STOP` writer - need_des"]
pub type REG_SDIO_PLL_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_BBPLL_CAL_END` reader - need_des"]
pub type REG_BBPLL_CAL_END_R = crate::BitReader;
#[doc = "Field `REG_BBPLL_CAL_STOP` reader - need_des"]
pub type REG_BBPLL_CAL_STOP_R = crate::BitReader;
#[doc = "Field `REG_BBPLL_CAL_STOP` writer - need_des"]
pub type REG_BBPLL_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MSPI_CAL_END` reader - need_des"]
pub type REG_MSPI_CAL_END_R = crate::BitReader;
#[doc = "Field `REG_MSPI_CAL_STOP` reader - need_des"]
pub type REG_MSPI_CAL_STOP_R = crate::BitReader;
#[doc = "Field `REG_MSPI_CAL_STOP` writer - need_des"]
pub type REG_MSPI_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PLL_CAL_END` reader - need_des"]
pub type REG_PLL_CAL_END_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_plla_cal_end(&self) -> REG_PLLA_CAL_END_R {
        REG_PLLA_CAL_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_plla_cal_stop(&self) -> REG_PLLA_CAL_STOP_R {
        REG_PLLA_CAL_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_cpu_pll_cal_end(&self) -> REG_CPU_PLL_CAL_END_R {
        REG_CPU_PLL_CAL_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn reg_cpu_pll_cal_stop(&self) -> REG_CPU_PLL_CAL_STOP_R {
        REG_CPU_PLL_CAL_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn reg_sdio_pll_cal_end(&self) -> REG_SDIO_PLL_CAL_END_R {
        REG_SDIO_PLL_CAL_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn reg_sdio_pll_cal_stop(&self) -> REG_SDIO_PLL_CAL_STOP_R {
        REG_SDIO_PLL_CAL_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn reg_bbpll_cal_end(&self) -> REG_BBPLL_CAL_END_R {
        REG_BBPLL_CAL_END_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn reg_bbpll_cal_stop(&self) -> REG_BBPLL_CAL_STOP_R {
        REG_BBPLL_CAL_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn reg_mspi_cal_end(&self) -> REG_MSPI_CAL_END_R {
        REG_MSPI_CAL_END_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn reg_mspi_cal_stop(&self) -> REG_MSPI_CAL_STOP_R {
        REG_MSPI_CAL_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn reg_pll_cal_end(&self) -> REG_PLL_CAL_END_R {
        REG_PLL_CAL_END_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_PLL_CTRL0")
            .field("reg_plla_cal_end", &self.reg_plla_cal_end())
            .field("reg_plla_cal_stop", &self.reg_plla_cal_stop())
            .field("reg_cpu_pll_cal_end", &self.reg_cpu_pll_cal_end())
            .field("reg_cpu_pll_cal_stop", &self.reg_cpu_pll_cal_stop())
            .field("reg_sdio_pll_cal_end", &self.reg_sdio_pll_cal_end())
            .field("reg_sdio_pll_cal_stop", &self.reg_sdio_pll_cal_stop())
            .field("reg_bbpll_cal_end", &self.reg_bbpll_cal_end())
            .field("reg_bbpll_cal_stop", &self.reg_bbpll_cal_stop())
            .field("reg_mspi_cal_end", &self.reg_mspi_cal_end())
            .field("reg_mspi_cal_stop", &self.reg_mspi_cal_stop())
            .field("reg_pll_cal_end", &self.reg_pll_cal_end())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_plla_cal_stop(&mut self) -> REG_PLLA_CAL_STOP_W<'_, ANA_PLL_CTRL0_SPEC> {
        REG_PLLA_CAL_STOP_W::new(self, 1)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn reg_cpu_pll_cal_stop(&mut self) -> REG_CPU_PLL_CAL_STOP_W<'_, ANA_PLL_CTRL0_SPEC> {
        REG_CPU_PLL_CAL_STOP_W::new(self, 3)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn reg_sdio_pll_cal_stop(&mut self) -> REG_SDIO_PLL_CAL_STOP_W<'_, ANA_PLL_CTRL0_SPEC> {
        REG_SDIO_PLL_CAL_STOP_W::new(self, 5)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn reg_bbpll_cal_stop(&mut self) -> REG_BBPLL_CAL_STOP_W<'_, ANA_PLL_CTRL0_SPEC> {
        REG_BBPLL_CAL_STOP_W::new(self, 7)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn reg_mspi_cal_stop(&mut self) -> REG_MSPI_CAL_STOP_W<'_, ANA_PLL_CTRL0_SPEC> {
        REG_MSPI_CAL_STOP_W::new(self, 9)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_pll_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_pll_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_PLL_CTRL0_SPEC;
impl crate::RegisterSpec for ANA_PLL_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_pll_ctrl0::R`](R) reader structure"]
impl crate::Readable for ANA_PLL_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_pll_ctrl0::W`](W) writer structure"]
impl crate::Writable for ANA_PLL_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANA_PLL_CTRL0 to value 0"]
impl crate::Resettable for ANA_PLL_CTRL0_SPEC {}

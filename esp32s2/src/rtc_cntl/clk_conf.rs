#[doc = "Register `CLK_CONF` reader"]
pub type R = crate::R<CLK_CONF_SPEC>;
#[doc = "Register `CLK_CONF` writer"]
pub type W = crate::W<CLK_CONF_SPEC>;
#[doc = "Field `CK8M_DIV_SEL_VLD` reader - Synchronizes the reg_ck8m_div_sel. Not that you have to invalidate the bus before switching clock, and validate the new clock."]
pub type CK8M_DIV_SEL_VLD_R = crate::BitReader;
#[doc = "Field `CK8M_DIV_SEL_VLD` writer - Synchronizes the reg_ck8m_div_sel. Not that you have to invalidate the bus before switching clock, and validate the new clock."]
pub type CK8M_DIV_SEL_VLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK8M_DIV` reader - Set the CK8M_D256_OUT divider. 00: divided by 128 01: divided by 256 10: divided by 512 11: divided by 1024."]
pub type CK8M_DIV_R = crate::FieldReader;
#[doc = "Field `CK8M_DIV` writer - Set the CK8M_D256_OUT divider. 00: divided by 128 01: divided by 256 10: divided by 512 11: divided by 1024."]
pub type CK8M_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENB_CK8M` reader - Set this bit to disable CK8M and CK8M_D256_OUT."]
pub type ENB_CK8M_R = crate::BitReader;
#[doc = "Field `ENB_CK8M` writer - Set this bit to disable CK8M and CK8M_D256_OUT."]
pub type ENB_CK8M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENB_CK8M_DIV` reader - Selects the CK8M_D256_OUT. 1: CK8M 0: CK8M divided by 256."]
pub type ENB_CK8M_DIV_R = crate::BitReader;
#[doc = "Field `ENB_CK8M_DIV` writer - Selects the CK8M_D256_OUT. 1: CK8M 0: CK8M divided by 256."]
pub type ENB_CK8M_DIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIG_XTAL32K_EN` reader - Set this bit to enable CK_XTAL_32K clock for the digital core."]
pub type DIG_XTAL32K_EN_R = crate::BitReader;
#[doc = "Field `DIG_XTAL32K_EN` writer - Set this bit to enable CK_XTAL_32K clock for the digital core."]
pub type DIG_XTAL32K_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIG_CLK8M_D256_EN` reader - Set this bit to enable CK8M_D256_OUT clock for the digital core."]
pub type DIG_CLK8M_D256_EN_R = crate::BitReader;
#[doc = "Field `DIG_CLK8M_D256_EN` writer - Set this bit to enable CK8M_D256_OUT clock for the digital core."]
pub type DIG_CLK8M_D256_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIG_CLK8M_EN` reader - Set this bit to enable 8 MHz clock for the digital core."]
pub type DIG_CLK8M_EN_R = crate::BitReader;
#[doc = "Field `DIG_CLK8M_EN` writer - Set this bit to enable 8 MHz clock for the digital core."]
pub type DIG_CLK8M_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK8M_DIV_SEL` reader - Stores the 8 MHz divider, which is reg_ck8m_div_sel + 1"]
pub type CK8M_DIV_SEL_R = crate::FieldReader;
#[doc = "Field `CK8M_DIV_SEL` writer - Stores the 8 MHz divider, which is reg_ck8m_div_sel + 1"]
pub type CK8M_DIV_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XTAL_FORCE_NOGATING` reader - Set this bit to force no gating to crystal during sleep"]
pub type XTAL_FORCE_NOGATING_R = crate::BitReader;
#[doc = "Field `XTAL_FORCE_NOGATING` writer - Set this bit to force no gating to crystal during sleep"]
pub type XTAL_FORCE_NOGATING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK8M_FORCE_NOGATING` reader - Set this bit to disable force gating to 8 MHz crystal during sleep."]
pub type CK8M_FORCE_NOGATING_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_NOGATING` writer - Set this bit to disable force gating to 8 MHz crystal during sleep."]
pub type CK8M_FORCE_NOGATING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK8M_DFREQ` reader - CK8M_DFREQ"]
pub type CK8M_DFREQ_R = crate::FieldReader;
#[doc = "Field `CK8M_DFREQ` writer - CK8M_DFREQ"]
pub type CK8M_DFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CK8M_FORCE_PD` reader - Set this bit to FPD the 8 MHz clock."]
pub type CK8M_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_PD` writer - Set this bit to FPD the 8 MHz clock."]
pub type CK8M_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK8M_FORCE_PU` reader - Set this bit to FPU the 8 MHz clock."]
pub type CK8M_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_PU` writer - Set this bit to FPU the 8 MHz clock."]
pub type CK8M_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST_CLK_RTC_SEL` reader - Set this bit to select the RTC fast clock. 0: XTAL div 4, 1: CK8M."]
pub type FAST_CLK_RTC_SEL_R = crate::BitReader;
#[doc = "Field `FAST_CLK_RTC_SEL` writer - Set this bit to select the RTC fast clock. 0: XTAL div 4, 1: CK8M."]
pub type FAST_CLK_RTC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_CLK_RTC_SEL` reader - Set this bit to select the RTC slow clock. 0: 90K rtc_clk 1: 32k XTAL 2: 8md256."]
pub type ANA_CLK_RTC_SEL_R = crate::FieldReader;
#[doc = "Field `ANA_CLK_RTC_SEL` writer - Set this bit to select the RTC slow clock. 0: 90K rtc_clk 1: 32k XTAL 2: 8md256."]
pub type ANA_CLK_RTC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 3 - Synchronizes the reg_ck8m_div_sel. Not that you have to invalidate the bus before switching clock, and validate the new clock."]
    #[inline(always)]
    pub fn ck8m_div_sel_vld(&self) -> CK8M_DIV_SEL_VLD_R {
        CK8M_DIV_SEL_VLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Set the CK8M_D256_OUT divider. 00: divided by 128 01: divided by 256 10: divided by 512 11: divided by 1024."]
    #[inline(always)]
    pub fn ck8m_div(&self) -> CK8M_DIV_R {
        CK8M_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Set this bit to disable CK8M and CK8M_D256_OUT."]
    #[inline(always)]
    pub fn enb_ck8m(&self) -> ENB_CK8M_R {
        ENB_CK8M_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects the CK8M_D256_OUT. 1: CK8M 0: CK8M divided by 256."]
    #[inline(always)]
    pub fn enb_ck8m_div(&self) -> ENB_CK8M_DIV_R {
        ENB_CK8M_DIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable CK_XTAL_32K clock for the digital core."]
    #[inline(always)]
    pub fn dig_xtal32k_en(&self) -> DIG_XTAL32K_EN_R {
        DIG_XTAL32K_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable CK8M_D256_OUT clock for the digital core."]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&self) -> DIG_CLK8M_D256_EN_R {
        DIG_CLK8M_D256_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable 8 MHz clock for the digital core."]
    #[inline(always)]
    pub fn dig_clk8m_en(&self) -> DIG_CLK8M_EN_R {
        DIG_CLK8M_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Stores the 8 MHz divider, which is reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn ck8m_div_sel(&self) -> CK8M_DIV_SEL_R {
        CK8M_DIV_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Set this bit to force no gating to crystal during sleep"]
    #[inline(always)]
    pub fn xtal_force_nogating(&self) -> XTAL_FORCE_NOGATING_R {
        XTAL_FORCE_NOGATING_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to disable force gating to 8 MHz crystal during sleep."]
    #[inline(always)]
    pub fn ck8m_force_nogating(&self) -> CK8M_FORCE_NOGATING_R {
        CK8M_FORCE_NOGATING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn ck8m_dfreq(&self) -> CK8M_DFREQ_R {
        CK8M_DFREQ_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bit 25 - Set this bit to FPD the 8 MHz clock."]
    #[inline(always)]
    pub fn ck8m_force_pd(&self) -> CK8M_FORCE_PD_R {
        CK8M_FORCE_PD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to FPU the 8 MHz clock."]
    #[inline(always)]
    pub fn ck8m_force_pu(&self) -> CK8M_FORCE_PU_R {
        CK8M_FORCE_PU_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to select the RTC fast clock. 0: XTAL div 4, 1: CK8M."]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&self) -> FAST_CLK_RTC_SEL_R {
        FAST_CLK_RTC_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Set this bit to select the RTC slow clock. 0: 90K rtc_clk 1: 32k XTAL 2: 8md256."]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&self) -> ANA_CLK_RTC_SEL_R {
        ANA_CLK_RTC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field("ck8m_div_sel_vld", &self.ck8m_div_sel_vld())
            .field("ck8m_div", &self.ck8m_div())
            .field("enb_ck8m", &self.enb_ck8m())
            .field("enb_ck8m_div", &self.enb_ck8m_div())
            .field("dig_xtal32k_en", &self.dig_xtal32k_en())
            .field("dig_clk8m_d256_en", &self.dig_clk8m_d256_en())
            .field("dig_clk8m_en", &self.dig_clk8m_en())
            .field("ck8m_div_sel", &self.ck8m_div_sel())
            .field("xtal_force_nogating", &self.xtal_force_nogating())
            .field("ck8m_force_nogating", &self.ck8m_force_nogating())
            .field("ck8m_dfreq", &self.ck8m_dfreq())
            .field("ck8m_force_pd", &self.ck8m_force_pd())
            .field("ck8m_force_pu", &self.ck8m_force_pu())
            .field("fast_clk_rtc_sel", &self.fast_clk_rtc_sel())
            .field("ana_clk_rtc_sel", &self.ana_clk_rtc_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Synchronizes the reg_ck8m_div_sel. Not that you have to invalidate the bus before switching clock, and validate the new clock."]
    #[inline(always)]
    pub fn ck8m_div_sel_vld(&mut self) -> CK8M_DIV_SEL_VLD_W<CLK_CONF_SPEC> {
        CK8M_DIV_SEL_VLD_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Set the CK8M_D256_OUT divider. 00: divided by 128 01: divided by 256 10: divided by 512 11: divided by 1024."]
    #[inline(always)]
    pub fn ck8m_div(&mut self) -> CK8M_DIV_W<CLK_CONF_SPEC> {
        CK8M_DIV_W::new(self, 4)
    }
    #[doc = "Bit 6 - Set this bit to disable CK8M and CK8M_D256_OUT."]
    #[inline(always)]
    pub fn enb_ck8m(&mut self) -> ENB_CK8M_W<CLK_CONF_SPEC> {
        ENB_CK8M_W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects the CK8M_D256_OUT. 1: CK8M 0: CK8M divided by 256."]
    #[inline(always)]
    pub fn enb_ck8m_div(&mut self) -> ENB_CK8M_DIV_W<CLK_CONF_SPEC> {
        ENB_CK8M_DIV_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to enable CK_XTAL_32K clock for the digital core."]
    #[inline(always)]
    pub fn dig_xtal32k_en(&mut self) -> DIG_XTAL32K_EN_W<CLK_CONF_SPEC> {
        DIG_XTAL32K_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to enable CK8M_D256_OUT clock for the digital core."]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&mut self) -> DIG_CLK8M_D256_EN_W<CLK_CONF_SPEC> {
        DIG_CLK8M_D256_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to enable 8 MHz clock for the digital core."]
    #[inline(always)]
    pub fn dig_clk8m_en(&mut self) -> DIG_CLK8M_EN_W<CLK_CONF_SPEC> {
        DIG_CLK8M_EN_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - Stores the 8 MHz divider, which is reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn ck8m_div_sel(&mut self) -> CK8M_DIV_SEL_W<CLK_CONF_SPEC> {
        CK8M_DIV_SEL_W::new(self, 12)
    }
    #[doc = "Bit 15 - Set this bit to force no gating to crystal during sleep"]
    #[inline(always)]
    pub fn xtal_force_nogating(&mut self) -> XTAL_FORCE_NOGATING_W<CLK_CONF_SPEC> {
        XTAL_FORCE_NOGATING_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to disable force gating to 8 MHz crystal during sleep."]
    #[inline(always)]
    pub fn ck8m_force_nogating(&mut self) -> CK8M_FORCE_NOGATING_W<CLK_CONF_SPEC> {
        CK8M_FORCE_NOGATING_W::new(self, 16)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn ck8m_dfreq(&mut self) -> CK8M_DFREQ_W<CLK_CONF_SPEC> {
        CK8M_DFREQ_W::new(self, 17)
    }
    #[doc = "Bit 25 - Set this bit to FPD the 8 MHz clock."]
    #[inline(always)]
    pub fn ck8m_force_pd(&mut self) -> CK8M_FORCE_PD_W<CLK_CONF_SPEC> {
        CK8M_FORCE_PD_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to FPU the 8 MHz clock."]
    #[inline(always)]
    pub fn ck8m_force_pu(&mut self) -> CK8M_FORCE_PU_W<CLK_CONF_SPEC> {
        CK8M_FORCE_PU_W::new(self, 26)
    }
    #[doc = "Bit 29 - Set this bit to select the RTC fast clock. 0: XTAL div 4, 1: CK8M."]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W<CLK_CONF_SPEC> {
        FAST_CLK_RTC_SEL_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - Set this bit to select the RTC slow clock. 0: 90K rtc_clk 1: 32k XTAL 2: 8md256."]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&mut self) -> ANA_CLK_RTC_SEL_W<CLK_CONF_SPEC> {
        ANA_CLK_RTC_SEL_W::new(self, 30)
    }
}
#[doc = "RTC clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x0158_3218"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0158_3218;
}

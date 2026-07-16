#[doc = "Register `LPPERI_PWR_CLK_CFG` reader"]
pub type R = crate::R<LPPERI_PWR_CLK_CFG_SPEC>;
#[doc = "Register `LPPERI_PWR_CLK_CFG` writer"]
pub type W = crate::W<LPPERI_PWR_CLK_CFG_SPEC>;
#[doc = "Field `LPPERI_SW_CLK_MODE` reader - software configs LPPERI clk mode"]
pub type LPPERI_SW_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `LPPERI_SW_CLK_MODE` writer - software configs LPPERI clk mode"]
pub type LPPERI_SW_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPPERI_SW_PWR_MODE` reader - software configs LPPERI pwr mode"]
pub type LPPERI_SW_PWR_MODE_R = crate::FieldReader;
#[doc = "Field `LPPERI_SW_PWR_MODE` writer - software configs LPPERI pwr mode"]
pub type LPPERI_SW_PWR_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - software configs LPPERI clk mode"]
    #[inline(always)]
    pub fn lpperi_sw_clk_mode(&self) -> LPPERI_SW_CLK_MODE_R {
        LPPERI_SW_CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - software configs LPPERI pwr mode"]
    #[inline(always)]
    pub fn lpperi_sw_pwr_mode(&self) -> LPPERI_SW_PWR_MODE_R {
        LPPERI_SW_PWR_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPPERI_PWR_CLK_CFG")
            .field("lpperi_sw_clk_mode", &self.lpperi_sw_clk_mode())
            .field("lpperi_sw_pwr_mode", &self.lpperi_sw_pwr_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - software configs LPPERI clk mode"]
    #[inline(always)]
    pub fn lpperi_sw_clk_mode(&mut self) -> LPPERI_SW_CLK_MODE_W<'_, LPPERI_PWR_CLK_CFG_SPEC> {
        LPPERI_SW_CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - software configs LPPERI pwr mode"]
    #[inline(always)]
    pub fn lpperi_sw_pwr_mode(&mut self) -> LPPERI_SW_PWR_MODE_W<'_, LPPERI_PWR_CLK_CFG_SPEC> {
        LPPERI_SW_PWR_MODE_W::new(self, 2)
    }
}
#[doc = "ctrl register for lpcpu power clock mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`lpperi_pwr_clk_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpperi_pwr_clk_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPPERI_PWR_CLK_CFG_SPEC;
impl crate::RegisterSpec for LPPERI_PWR_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpperi_pwr_clk_cfg::R`](R) reader structure"]
impl crate::Readable for LPPERI_PWR_CLK_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpperi_pwr_clk_cfg::W`](W) writer structure"]
impl crate::Writable for LPPERI_PWR_CLK_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPPERI_PWR_CLK_CFG to value 0x0b"]
impl crate::Resettable for LPPERI_PWR_CLK_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0b;
}

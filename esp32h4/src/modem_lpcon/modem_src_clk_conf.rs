#[doc = "Register `MODEM_SRC_CLK_CONF` reader"]
pub type R = crate::R<MODEM_SRC_CLK_CONF_SPEC>;
#[doc = "Register `MODEM_SRC_CLK_CONF` writer"]
pub type W = crate::W<MODEM_SRC_CLK_CONF_SPEC>;
#[doc = "Field `CLK_MODEM_AON_FORCE` reader - "]
pub type CLK_MODEM_AON_FORCE_R = crate::FieldReader;
#[doc = "Field `CLK_MODEM_AON_FORCE` writer - "]
pub type CLK_MODEM_AON_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODEM_PWR_32K_FO` reader - "]
pub type MODEM_PWR_32K_FO_R = crate::BitReader;
#[doc = "Field `MODEM_PWR_32K_FO` writer - "]
pub type MODEM_PWR_32K_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_PWR_FOSC_FO` reader - "]
pub type MODEM_PWR_FOSC_FO_R = crate::BitReader;
#[doc = "Field `MODEM_PWR_FOSC_FO` writer - "]
pub type MODEM_PWR_FOSC_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_PWR_SOSC_FO` reader - "]
pub type MODEM_PWR_SOSC_FO_R = crate::BitReader;
#[doc = "Field `MODEM_PWR_SOSC_FO` writer - "]
pub type MODEM_PWR_SOSC_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_PWR_XTAL_FO` reader - "]
pub type MODEM_PWR_XTAL_FO_R = crate::BitReader;
#[doc = "Field `MODEM_PWR_XTAL_FO` writer - "]
pub type MODEM_PWR_XTAL_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_modem_aon_force(&self) -> CLK_MODEM_AON_FORCE_R {
        CLK_MODEM_AON_FORCE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn modem_pwr_32k_fo(&self) -> MODEM_PWR_32K_FO_R {
        MODEM_PWR_32K_FO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn modem_pwr_fosc_fo(&self) -> MODEM_PWR_FOSC_FO_R {
        MODEM_PWR_FOSC_FO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn modem_pwr_sosc_fo(&self) -> MODEM_PWR_SOSC_FO_R {
        MODEM_PWR_SOSC_FO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn modem_pwr_xtal_fo(&self) -> MODEM_PWR_XTAL_FO_R {
        MODEM_PWR_XTAL_FO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_SRC_CLK_CONF")
            .field("clk_modem_aon_force", &self.clk_modem_aon_force())
            .field("modem_pwr_32k_fo", &self.modem_pwr_32k_fo())
            .field("modem_pwr_fosc_fo", &self.modem_pwr_fosc_fo())
            .field("modem_pwr_sosc_fo", &self.modem_pwr_sosc_fo())
            .field("modem_pwr_xtal_fo", &self.modem_pwr_xtal_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_modem_aon_force(&mut self) -> CLK_MODEM_AON_FORCE_W<'_, MODEM_SRC_CLK_CONF_SPEC> {
        CLK_MODEM_AON_FORCE_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn modem_pwr_32k_fo(&mut self) -> MODEM_PWR_32K_FO_W<'_, MODEM_SRC_CLK_CONF_SPEC> {
        MODEM_PWR_32K_FO_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn modem_pwr_fosc_fo(&mut self) -> MODEM_PWR_FOSC_FO_W<'_, MODEM_SRC_CLK_CONF_SPEC> {
        MODEM_PWR_FOSC_FO_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn modem_pwr_sosc_fo(&mut self) -> MODEM_PWR_SOSC_FO_W<'_, MODEM_SRC_CLK_CONF_SPEC> {
        MODEM_PWR_SOSC_FO_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn modem_pwr_xtal_fo(&mut self) -> MODEM_PWR_XTAL_FO_W<'_, MODEM_SRC_CLK_CONF_SPEC> {
        MODEM_PWR_XTAL_FO_W::new(self, 5)
    }
}
#[doc = "MODEM_SRC_CLK_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_src_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_src_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_SRC_CLK_CONF_SPEC;
impl crate::RegisterSpec for MODEM_SRC_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_src_clk_conf::R`](R) reader structure"]
impl crate::Readable for MODEM_SRC_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_src_clk_conf::W`](W) writer structure"]
impl crate::Writable for MODEM_SRC_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_SRC_CLK_CONF to value 0"]
impl crate::Resettable for MODEM_SRC_CLK_CONF_SPEC {}

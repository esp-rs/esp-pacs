#[doc = "Register `MODEM_APB_CONF` reader"]
pub type R = crate::R<MODEM_APB_CONF_SPEC>;
#[doc = "Register `MODEM_APB_CONF` writer"]
pub type W = crate::W<MODEM_APB_CONF_SPEC>;
#[doc = "Field `MODEM_APB_CLK_EN` reader - This field indicates if modem_apb clock is enable. 0: disable, 1: enable(default)."]
pub type MODEM_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `MODEM_APB_CLK_EN` writer - This field indicates if modem_apb clock is enable. 0: disable, 1: enable(default)."]
pub type MODEM_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_RST_EN` reader - Set this file as 1 to reset modem-subsystem."]
pub type MODEM_RST_EN_R = crate::BitReader;
#[doc = "Field `MODEM_RST_EN` writer - Set this file as 1 to reset modem-subsystem."]
pub type MODEM_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field indicates if modem_apb clock is enable. 0: disable, 1: enable(default)."]
    #[inline(always)]
    pub fn modem_apb_clk_en(&self) -> MODEM_APB_CLK_EN_R {
        MODEM_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this file as 1 to reset modem-subsystem."]
    #[inline(always)]
    pub fn modem_rst_en(&self) -> MODEM_RST_EN_R {
        MODEM_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_APB_CONF")
            .field(
                "modem_apb_clk_en",
                &format_args!("{}", self.modem_apb_clk_en().bit()),
            )
            .field(
                "modem_rst_en",
                &format_args!("{}", self.modem_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEM_APB_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This field indicates if modem_apb clock is enable. 0: disable, 1: enable(default)."]
    #[inline(always)]
    #[must_use]
    pub fn modem_apb_clk_en(&mut self) -> MODEM_APB_CLK_EN_W<MODEM_APB_CONF_SPEC> {
        MODEM_APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this file as 1 to reset modem-subsystem."]
    #[inline(always)]
    #[must_use]
    pub fn modem_rst_en(&mut self) -> MODEM_RST_EN_W<MODEM_APB_CONF_SPEC> {
        MODEM_RST_EN_W::new(self, 1)
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
#[doc = "MODEM_APB configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modem_apb_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modem_apb_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_APB_CONF_SPEC;
impl crate::RegisterSpec for MODEM_APB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_apb_conf::R`](R) reader structure"]
impl crate::Readable for MODEM_APB_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_apb_conf::W`](W) writer structure"]
impl crate::Writable for MODEM_APB_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEM_APB_CONF to value 0x01"]
impl crate::Resettable for MODEM_APB_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

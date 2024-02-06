#[doc = "Register `PARL_IO_CONF` reader"]
pub type R = crate::R<PARL_IO_CONF_SPEC>;
#[doc = "Register `PARL_IO_CONF` writer"]
pub type W = crate::W<PARL_IO_CONF_SPEC>;
#[doc = "Field `PARL_CLK_EN` reader - Set 1 to enable parl apb clock"]
pub type PARL_CLK_EN_R = crate::BitReader;
#[doc = "Field `PARL_CLK_EN` writer - Set 1 to enable parl apb clock"]
pub type PARL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARL_RST_EN` reader - Set 0 to reset parl apb reg"]
pub type PARL_RST_EN_R = crate::BitReader;
#[doc = "Field `PARL_RST_EN` writer - Set 0 to reset parl apb reg"]
pub type PARL_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARL_READY` reader - Query this field after reset parl module"]
pub type PARL_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable parl apb clock"]
    #[inline(always)]
    pub fn parl_clk_en(&self) -> PARL_CLK_EN_R {
        PARL_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset parl apb reg"]
    #[inline(always)]
    pub fn parl_rst_en(&self) -> PARL_RST_EN_R {
        PARL_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset parl module"]
    #[inline(always)]
    pub fn parl_ready(&self) -> PARL_READY_R {
        PARL_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARL_IO_CONF")
            .field("parl_clk_en", &format_args!("{}", self.parl_clk_en().bit()))
            .field("parl_rst_en", &format_args!("{}", self.parl_rst_en().bit()))
            .field("parl_ready", &format_args!("{}", self.parl_ready().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PARL_IO_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable parl apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn parl_clk_en(&mut self) -> PARL_CLK_EN_W<PARL_IO_CONF_SPEC> {
        PARL_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset parl apb reg"]
    #[inline(always)]
    #[must_use]
    pub fn parl_rst_en(&mut self) -> PARL_RST_EN_W<PARL_IO_CONF_SPEC> {
        PARL_RST_EN_W::new(self, 1)
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
#[doc = "PARL_IO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`parl_io_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parl_io_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PARL_IO_CONF_SPEC;
impl crate::RegisterSpec for PARL_IO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`parl_io_conf::R`](R) reader structure"]
impl crate::Readable for PARL_IO_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`parl_io_conf::W`](W) writer structure"]
impl crate::Writable for PARL_IO_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARL_IO_CONF to value 0x05"]
impl crate::Resettable for PARL_IO_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}

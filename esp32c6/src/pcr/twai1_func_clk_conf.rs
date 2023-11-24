#[doc = "Register `TWAI1_FUNC_CLK_CONF` reader"]
pub type R = crate::R<TWAI1_FUNC_CLK_CONF_SPEC>;
#[doc = "Register `TWAI1_FUNC_CLK_CONF` writer"]
pub type W = crate::W<TWAI1_FUNC_CLK_CONF_SPEC>;
#[doc = "Field `TWAI1_FUNC_CLK_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
pub type TWAI1_FUNC_CLK_SEL_R = crate::BitReader;
#[doc = "Field `TWAI1_FUNC_CLK_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
pub type TWAI1_FUNC_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWAI1_FUNC_CLK_EN` reader - Set 1 to enable twai1 function clock"]
pub type TWAI1_FUNC_CLK_EN_R = crate::BitReader;
#[doc = "Field `TWAI1_FUNC_CLK_EN` writer - Set 1 to enable twai1 function clock"]
pub type TWAI1_FUNC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 20 - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
    #[inline(always)]
    pub fn twai1_func_clk_sel(&self) -> TWAI1_FUNC_CLK_SEL_R {
        TWAI1_FUNC_CLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Set 1 to enable twai1 function clock"]
    #[inline(always)]
    pub fn twai1_func_clk_en(&self) -> TWAI1_FUNC_CLK_EN_R {
        TWAI1_FUNC_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWAI1_FUNC_CLK_CONF")
            .field(
                "twai1_func_clk_sel",
                &format_args!("{}", self.twai1_func_clk_sel().bit()),
            )
            .field(
                "twai1_func_clk_en",
                &format_args!("{}", self.twai1_func_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TWAI1_FUNC_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 20 - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
    #[inline(always)]
    #[must_use]
    pub fn twai1_func_clk_sel(&mut self) -> TWAI1_FUNC_CLK_SEL_W<TWAI1_FUNC_CLK_CONF_SPEC> {
        TWAI1_FUNC_CLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - Set 1 to enable twai1 function clock"]
    #[inline(always)]
    #[must_use]
    pub fn twai1_func_clk_en(&mut self) -> TWAI1_FUNC_CLK_EN_W<TWAI1_FUNC_CLK_CONF_SPEC> {
        TWAI1_FUNC_CLK_EN_W::new(self, 22)
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
#[doc = "TWAI1_FUNC_CLK configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twai1_func_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twai1_func_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWAI1_FUNC_CLK_CONF_SPEC;
impl crate::RegisterSpec for TWAI1_FUNC_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twai1_func_clk_conf::R`](R) reader structure"]
impl crate::Readable for TWAI1_FUNC_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twai1_func_clk_conf::W`](W) writer structure"]
impl crate::Writable for TWAI1_FUNC_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWAI1_FUNC_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for TWAI1_FUNC_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}

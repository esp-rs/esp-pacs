#[doc = "Register `MODEM_32K_CLK_CONF` reader"]
pub type R = crate::R<MODEM_32K_CLK_CONF_SPEC>;
#[doc = "Register `MODEM_32K_CLK_CONF` writer"]
pub type W = crate::W<MODEM_32K_CLK_CONF_SPEC>;
#[doc = "Field `CLK_MODEM_32K_SEL` reader - "]
pub type CLK_MODEM_32K_SEL_R = crate::FieldReader;
#[doc = "Field `CLK_MODEM_32K_SEL` writer - "]
pub type CLK_MODEM_32K_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_modem_32k_sel(&self) -> CLK_MODEM_32K_SEL_R {
        CLK_MODEM_32K_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_32K_CLK_CONF")
            .field(
                "clk_modem_32k_sel",
                &format_args!("{}", self.clk_modem_32k_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEM_32K_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_32k_sel(&mut self) -> CLK_MODEM_32K_SEL_W<MODEM_32K_CLK_CONF_SPEC, 0> {
        CLK_MODEM_32K_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modem_32k_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modem_32k_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_32K_CLK_CONF_SPEC;
impl crate::RegisterSpec for MODEM_32K_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_32k_clk_conf::R`](R) reader structure"]
impl crate::Readable for MODEM_32K_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_32k_clk_conf::W`](W) writer structure"]
impl crate::Writable for MODEM_32K_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEM_32K_CLK_CONF to value 0"]
impl crate::Resettable for MODEM_32K_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

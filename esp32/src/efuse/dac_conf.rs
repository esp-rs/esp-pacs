#[doc = "Register `DAC_CONF` reader"]
pub type R = crate::R<DAC_CONF_SPEC>;
#[doc = "Register `DAC_CONF` writer"]
pub type W = crate::W<DAC_CONF_SPEC>;
#[doc = "Field `DAC_CLK_DIV` reader - "]
pub type DAC_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `DAC_CLK_DIV` writer - "]
pub type DAC_CLK_DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DAC_CLK_PAD_SEL` reader - "]
pub type DAC_CLK_PAD_SEL_R = crate::BitReader;
#[doc = "Field `DAC_CLK_PAD_SEL` writer - "]
pub type DAC_CLK_PAD_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dac_clk_div(&self) -> DAC_CLK_DIV_R {
        DAC_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dac_clk_pad_sel(&self) -> DAC_CLK_PAD_SEL_R {
        DAC_CLK_PAD_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CONF")
            .field(
                "dac_clk_div",
                &format_args!("{}", self.dac_clk_div().bits()),
            )
            .field(
                "dac_clk_pad_sel",
                &format_args!("{}", self.dac_clk_pad_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DAC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_div(&mut self) -> DAC_CLK_DIV_W<DAC_CONF_SPEC, 0> {
        DAC_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_pad_sel(&mut self) -> DAC_CLK_PAD_SEL_W<DAC_CONF_SPEC, 8> {
        DAC_CLK_PAD_SEL_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_CONF_SPEC;
impl crate::RegisterSpec for DAC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_conf::R`](R) reader structure"]
impl crate::Readable for DAC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac_conf::W`](W) writer structure"]
impl crate::Writable for DAC_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_CONF to value 0x28"]
impl crate::Resettable for DAC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x28;
}

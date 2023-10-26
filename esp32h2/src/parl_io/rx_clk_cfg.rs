#[doc = "Register `RX_CLK_CFG` reader"]
pub type R = crate::R<RX_CLK_CFG_SPEC>;
#[doc = "Register `RX_CLK_CFG` writer"]
pub type W = crate::W<RX_CLK_CFG_SPEC>;
#[doc = "Field `RX_CLK_I_INV` reader - Set this bit to invert the input Rx core clock."]
pub type RX_CLK_I_INV_R = crate::BitReader;
#[doc = "Field `RX_CLK_I_INV` writer - Set this bit to invert the input Rx core clock."]
pub type RX_CLK_I_INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_CLK_O_INV` reader - Set this bit to invert the output Rx core clock."]
pub type RX_CLK_O_INV_R = crate::BitReader;
#[doc = "Field `RX_CLK_O_INV` writer - Set this bit to invert the output Rx core clock."]
pub type RX_CLK_O_INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 30 - Set this bit to invert the input Rx core clock."]
    #[inline(always)]
    pub fn rx_clk_i_inv(&self) -> RX_CLK_I_INV_R {
        RX_CLK_I_INV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to invert the output Rx core clock."]
    #[inline(always)]
    pub fn rx_clk_o_inv(&self) -> RX_CLK_O_INV_R {
        RX_CLK_O_INV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CLK_CFG")
            .field(
                "rx_clk_i_inv",
                &format_args!("{}", self.rx_clk_i_inv().bit()),
            )
            .field(
                "rx_clk_o_inv",
                &format_args!("{}", self.rx_clk_o_inv().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CLK_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - Set this bit to invert the input Rx core clock."]
    #[inline(always)]
    #[must_use]
    pub fn rx_clk_i_inv(&mut self) -> RX_CLK_I_INV_W<RX_CLK_CFG_SPEC, 30> {
        RX_CLK_I_INV_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to invert the output Rx core clock."]
    #[inline(always)]
    #[must_use]
    pub fn rx_clk_o_inv(&mut self) -> RX_CLK_O_INV_W<RX_CLK_CFG_SPEC, 31> {
        RX_CLK_O_INV_W::new(self)
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
#[doc = "Parallel IO RX clk configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_clk_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_clk_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CLK_CFG_SPEC;
impl crate::RegisterSpec for RX_CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_clk_cfg::R`](R) reader structure"]
impl crate::Readable for RX_CLK_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_clk_cfg::W`](W) writer structure"]
impl crate::Writable for RX_CLK_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CLK_CFG to value 0"]
impl crate::Resettable for RX_CLK_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

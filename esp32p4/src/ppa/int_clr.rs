#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SR_EOF` writer - Set this bit to clear the PPA_SR_EOF_INT interrupt."]
pub type SR_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BLEND_EOF` writer - Set this bit to clear the PPA_BLEND_EOF_INT interrupt."]
pub type BLEND_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SR_PARAM_CFG_ERR` writer - Set this bit to clear the PPA_SR_RX_YSCAL_ERR_INT interrupt."]
pub type SR_PARAM_CFG_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the PPA_SR_EOF_INT interrupt."]
    #[inline(always)]
    pub fn sr_eof(&mut self) -> SR_EOF_W<INT_CLR_SPEC> {
        SR_EOF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the PPA_BLEND_EOF_INT interrupt."]
    #[inline(always)]
    pub fn blend_eof(&mut self) -> BLEND_EOF_W<INT_CLR_SPEC> {
        BLEND_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the PPA_SR_RX_YSCAL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn sr_param_cfg_err(&mut self) -> SR_PARAM_CFG_ERR_W<INT_CLR_SPEC> {
        SR_PARAM_CFG_ERR_W::new(self, 2)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}

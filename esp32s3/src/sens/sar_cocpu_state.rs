#[doc = "Register `SAR_COCPU_STATE` reader"]
pub type R = crate::R<SAR_COCPU_STATE_SPEC>;
#[doc = "Register `SAR_COCPU_STATE` writer"]
pub type W = crate::W<SAR_COCPU_STATE_SPEC>;
#[doc = "Field `SAR_COCPU_DBG_TRIGGER` writer - trigger cocpu debug registers"]
pub type SAR_COCPU_DBG_TRIGGER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAR_COCPU_CLK_EN_ST` reader - check cocpu whether clk on"]
pub type SAR_COCPU_CLK_EN_ST_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_RESET_N` reader - check cocpu whether in reset state"]
pub type SAR_COCPU_RESET_N_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_EOI` reader - check cocpu whether in interrupt state"]
pub type SAR_COCPU_EOI_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_TRAP` reader - check cocpu whether in trap state"]
pub type SAR_COCPU_TRAP_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_EBREAK` reader - check cocpu whether in ebreak"]
pub type SAR_COCPU_EBREAK_R = crate::BitReader;
impl R {
    #[doc = "Bit 26 - check cocpu whether clk on"]
    #[inline(always)]
    pub fn sar_cocpu_clk_en_st(&self) -> SAR_COCPU_CLK_EN_ST_R {
        SAR_COCPU_CLK_EN_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - check cocpu whether in reset state"]
    #[inline(always)]
    pub fn sar_cocpu_reset_n(&self) -> SAR_COCPU_RESET_N_R {
        SAR_COCPU_RESET_N_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - check cocpu whether in interrupt state"]
    #[inline(always)]
    pub fn sar_cocpu_eoi(&self) -> SAR_COCPU_EOI_R {
        SAR_COCPU_EOI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - check cocpu whether in trap state"]
    #[inline(always)]
    pub fn sar_cocpu_trap(&self) -> SAR_COCPU_TRAP_R {
        SAR_COCPU_TRAP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - check cocpu whether in ebreak"]
    #[inline(always)]
    pub fn sar_cocpu_ebreak(&self) -> SAR_COCPU_EBREAK_R {
        SAR_COCPU_EBREAK_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_COCPU_STATE")
            .field(
                "sar_cocpu_clk_en_st",
                &format_args!("{}", self.sar_cocpu_clk_en_st().bit()),
            )
            .field(
                "sar_cocpu_reset_n",
                &format_args!("{}", self.sar_cocpu_reset_n().bit()),
            )
            .field(
                "sar_cocpu_eoi",
                &format_args!("{}", self.sar_cocpu_eoi().bit()),
            )
            .field(
                "sar_cocpu_trap",
                &format_args!("{}", self.sar_cocpu_trap().bit()),
            )
            .field(
                "sar_cocpu_ebreak",
                &format_args!("{}", self.sar_cocpu_ebreak().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_COCPU_STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 25 - trigger cocpu debug registers"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_dbg_trigger(&mut self) -> SAR_COCPU_DBG_TRIGGER_W<SAR_COCPU_STATE_SPEC, 25> {
        SAR_COCPU_DBG_TRIGGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "get cocpu status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_COCPU_STATE_SPEC;
impl crate::RegisterSpec for SAR_COCPU_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_cocpu_state::R`](R) reader structure"]
impl crate::Readable for SAR_COCPU_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_cocpu_state::W`](W) writer structure"]
impl crate::Writable for SAR_COCPU_STATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_COCPU_STATE to value 0"]
impl crate::Resettable for SAR_COCPU_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

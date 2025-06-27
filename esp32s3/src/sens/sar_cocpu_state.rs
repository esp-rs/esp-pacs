#[doc = "Register `SAR_COCPU_STATE` reader"]
pub type R = crate::R<SAR_COCPU_STATE_SPEC>;
#[doc = "Register `SAR_COCPU_STATE` writer"]
pub type W = crate::W<SAR_COCPU_STATE_SPEC>;
#[doc = "Field `SAR_COCPU_DBG_TRIGGER` writer - trigger cocpu debug registers"]
pub type SAR_COCPU_DBG_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("sar_cocpu_clk_en_st", &self.sar_cocpu_clk_en_st())
            .field("sar_cocpu_reset_n", &self.sar_cocpu_reset_n())
            .field("sar_cocpu_eoi", &self.sar_cocpu_eoi())
            .field("sar_cocpu_trap", &self.sar_cocpu_trap())
            .field("sar_cocpu_ebreak", &self.sar_cocpu_ebreak())
            .finish()
    }
}
impl W {
    #[doc = "Bit 25 - trigger cocpu debug registers"]
    #[inline(always)]
    pub fn sar_cocpu_dbg_trigger(&mut self) -> SAR_COCPU_DBG_TRIGGER_W<SAR_COCPU_STATE_SPEC> {
        SAR_COCPU_DBG_TRIGGER_W::new(self, 25)
    }
}
#[doc = "get cocpu status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_COCPU_STATE_SPEC;
impl crate::RegisterSpec for SAR_COCPU_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_cocpu_state::R`](R) reader structure"]
impl crate::Readable for SAR_COCPU_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_cocpu_state::W`](W) writer structure"]
impl crate::Writable for SAR_COCPU_STATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_COCPU_STATE to value 0"]
impl crate::Resettable for SAR_COCPU_STATE_SPEC {}

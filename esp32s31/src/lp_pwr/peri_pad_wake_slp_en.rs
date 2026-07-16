#[doc = "Register `PERI_PAD_WAKE_SLP_EN` reader"]
pub type R = crate::R<PERI_PAD_WAKE_SLP_EN_SPEC>;
#[doc = "Register `PERI_PAD_WAKE_SLP_EN` writer"]
pub type W = crate::W<PERI_PAD_WAKE_SLP_EN_SPEC>;
#[doc = "Field `UART_WAKEUP_EN` reader - 1:enables uart wakeup after hpcpu stall 0:disables uart wakeup after hpcpu stall"]
pub type UART_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `UART_WAKEUP_EN` writer - 1:enables uart wakeup after hpcpu stall 0:disables uart wakeup after hpcpu stall"]
pub type UART_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_PAD_SLP_SEL` reader - 1:hppad will sleep after hpcpu stall 0:hppad will not sleep after hpcpu stall"]
pub type HP_PAD_SLP_SEL_R = crate::BitReader;
#[doc = "Field `HP_PAD_SLP_SEL` writer - 1:hppad will sleep after hpcpu stall 0:hppad will not sleep after hpcpu stall"]
pub type HP_PAD_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PAD_SLP_SEL` reader - 1:hppad will sleep after hpcpu stall 0:hppad will not sleep after hpcpu stall"]
pub type LP_PAD_SLP_SEL_R = crate::BitReader;
#[doc = "Field `LP_PAD_SLP_SEL` writer - 1:hppad will sleep after hpcpu stall 0:hppad will not sleep after hpcpu stall"]
pub type LP_PAD_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1:enables uart wakeup after hpcpu stall 0:disables uart wakeup after hpcpu stall"]
    #[inline(always)]
    pub fn uart_wakeup_en(&self) -> UART_WAKEUP_EN_R {
        UART_WAKEUP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:hppad will sleep after hpcpu stall 0:hppad will not sleep after hpcpu stall"]
    #[inline(always)]
    pub fn hp_pad_slp_sel(&self) -> HP_PAD_SLP_SEL_R {
        HP_PAD_SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1:hppad will sleep after hpcpu stall 0:hppad will not sleep after hpcpu stall"]
    #[inline(always)]
    pub fn lp_pad_slp_sel(&self) -> LP_PAD_SLP_SEL_R {
        LP_PAD_SLP_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_PAD_WAKE_SLP_EN")
            .field("uart_wakeup_en", &self.uart_wakeup_en())
            .field("hp_pad_slp_sel", &self.hp_pad_slp_sel())
            .field("lp_pad_slp_sel", &self.lp_pad_slp_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1:enables uart wakeup after hpcpu stall 0:disables uart wakeup after hpcpu stall"]
    #[inline(always)]
    pub fn uart_wakeup_en(&mut self) -> UART_WAKEUP_EN_W<'_, PERI_PAD_WAKE_SLP_EN_SPEC> {
        UART_WAKEUP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:hppad will sleep after hpcpu stall 0:hppad will not sleep after hpcpu stall"]
    #[inline(always)]
    pub fn hp_pad_slp_sel(&mut self) -> HP_PAD_SLP_SEL_W<'_, PERI_PAD_WAKE_SLP_EN_SPEC> {
        HP_PAD_SLP_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1:hppad will sleep after hpcpu stall 0:hppad will not sleep after hpcpu stall"]
    #[inline(always)]
    pub fn lp_pad_slp_sel(&mut self) -> LP_PAD_SLP_SEL_W<'_, PERI_PAD_WAKE_SLP_EN_SPEC> {
        LP_PAD_SLP_SEL_W::new(self, 2)
    }
}
#[doc = "used for future potential eco, others don't care\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_pad_wake_slp_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_pad_wake_slp_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_PAD_WAKE_SLP_EN_SPEC;
impl crate::RegisterSpec for PERI_PAD_WAKE_SLP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_pad_wake_slp_en::R`](R) reader structure"]
impl crate::Readable for PERI_PAD_WAKE_SLP_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_pad_wake_slp_en::W`](W) writer structure"]
impl crate::Writable for PERI_PAD_WAKE_SLP_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_PAD_WAKE_SLP_EN to value 0"]
impl crate::Resettable for PERI_PAD_WAKE_SLP_EN_SPEC {}

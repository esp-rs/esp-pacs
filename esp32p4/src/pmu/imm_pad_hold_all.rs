#[doc = "Register `IMM_PAD_HOLD_ALL` reader"]
pub type R = crate::R<IMM_PAD_HOLD_ALL_SPEC>;
#[doc = "Register `IMM_PAD_HOLD_ALL` writer"]
pub type W = crate::W<IMM_PAD_HOLD_ALL_SPEC>;
#[doc = "Field `PAD_SLP_SEL` reader - need_des"]
pub type PAD_SLP_SEL_R = crate::BitReader;
#[doc = "Field `LP_PAD_HOLD_ALL` reader - need_des"]
pub type LP_PAD_HOLD_ALL_R = crate::BitReader;
#[doc = "Field `HP_PAD_HOLD_ALL` reader - need_des"]
pub type HP_PAD_HOLD_ALL_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_PAD_SLP_SEL` writer - need_des"]
pub type TIE_HIGH_PAD_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_PAD_SLP_SEL` writer - need_des"]
pub type TIE_LOW_PAD_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_LP_PAD_HOLD_ALL` writer - need_des"]
pub type TIE_HIGH_LP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_LP_PAD_HOLD_ALL` writer - need_des"]
pub type TIE_LOW_LP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_HP_PAD_HOLD_ALL` writer - need_des"]
pub type TIE_HIGH_HP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_HP_PAD_HOLD_ALL` writer - need_des"]
pub type TIE_LOW_HP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn pad_slp_sel(&self) -> PAD_SLP_SEL_R {
        PAD_SLP_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn lp_pad_hold_all(&self) -> LP_PAD_HOLD_ALL_R {
        LP_PAD_HOLD_ALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn hp_pad_hold_all(&self) -> HP_PAD_HOLD_ALL_R {
        HP_PAD_HOLD_ALL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMM_PAD_HOLD_ALL")
            .field("pad_slp_sel", &self.pad_slp_sel())
            .field("lp_pad_hold_all", &self.lp_pad_hold_all())
            .field("hp_pad_hold_all", &self.hp_pad_hold_all())
            .finish()
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn tie_high_pad_slp_sel(&mut self) -> TIE_HIGH_PAD_SLP_SEL_W<'_, IMM_PAD_HOLD_ALL_SPEC> {
        TIE_HIGH_PAD_SLP_SEL_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn tie_low_pad_slp_sel(&mut self) -> TIE_LOW_PAD_SLP_SEL_W<'_, IMM_PAD_HOLD_ALL_SPEC> {
        TIE_LOW_PAD_SLP_SEL_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn tie_high_lp_pad_hold_all(
        &mut self,
    ) -> TIE_HIGH_LP_PAD_HOLD_ALL_W<'_, IMM_PAD_HOLD_ALL_SPEC> {
        TIE_HIGH_LP_PAD_HOLD_ALL_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn tie_low_lp_pad_hold_all(
        &mut self,
    ) -> TIE_LOW_LP_PAD_HOLD_ALL_W<'_, IMM_PAD_HOLD_ALL_SPEC> {
        TIE_LOW_LP_PAD_HOLD_ALL_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn tie_high_hp_pad_hold_all(
        &mut self,
    ) -> TIE_HIGH_HP_PAD_HOLD_ALL_W<'_, IMM_PAD_HOLD_ALL_SPEC> {
        TIE_HIGH_HP_PAD_HOLD_ALL_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tie_low_hp_pad_hold_all(
        &mut self,
    ) -> TIE_LOW_HP_PAD_HOLD_ALL_W<'_, IMM_PAD_HOLD_ALL_SPEC> {
        TIE_LOW_HP_PAD_HOLD_ALL_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`imm_pad_hold_all::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_pad_hold_all::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMM_PAD_HOLD_ALL_SPEC;
impl crate::RegisterSpec for IMM_PAD_HOLD_ALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imm_pad_hold_all::R`](R) reader structure"]
impl crate::Readable for IMM_PAD_HOLD_ALL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imm_pad_hold_all::W`](W) writer structure"]
impl crate::Writable for IMM_PAD_HOLD_ALL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMM_PAD_HOLD_ALL to value 0"]
impl crate::Resettable for IMM_PAD_HOLD_ALL_SPEC {}

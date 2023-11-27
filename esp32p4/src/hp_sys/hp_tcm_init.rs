#[doc = "Register `HP_TCM_INIT` reader"]
pub type R = crate::R<HP_TCM_INIT_SPEC>;
#[doc = "Register `HP_TCM_INIT` writer"]
pub type W = crate::W<HP_TCM_INIT_SPEC>;
#[doc = "Field `HP_REG_TCM_INIT_EN` reader - NA"]
pub type HP_REG_TCM_INIT_EN_R = crate::BitReader;
#[doc = "Field `HP_REG_TCM_INIT_EN` writer - NA"]
pub type HP_REG_TCM_INIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_TCM_INIT_CNT_RESET` reader - Set 1 to reset tcm init cnt"]
pub type HP_REG_TCM_INIT_CNT_RESET_R = crate::BitReader;
#[doc = "Field `HP_REG_TCM_INIT_CNT_RESET` writer - Set 1 to reset tcm init cnt"]
pub type HP_REG_TCM_INIT_CNT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_TCM_INIT_DONE` reader - NA"]
pub type HP_REG_TCM_INIT_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn hp_reg_tcm_init_en(&self) -> HP_REG_TCM_INIT_EN_R {
        HP_REG_TCM_INIT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset tcm init cnt"]
    #[inline(always)]
    pub fn hp_reg_tcm_init_cnt_reset(&self) -> HP_REG_TCM_INIT_CNT_RESET_R {
        HP_REG_TCM_INIT_CNT_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn hp_reg_tcm_init_done(&self) -> HP_REG_TCM_INIT_DONE_R {
        HP_REG_TCM_INIT_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_TCM_INIT")
            .field(
                "hp_reg_tcm_init_en",
                &format_args!("{}", self.hp_reg_tcm_init_en().bit()),
            )
            .field(
                "hp_reg_tcm_init_cnt_reset",
                &format_args!("{}", self.hp_reg_tcm_init_cnt_reset().bit()),
            )
            .field(
                "hp_reg_tcm_init_done",
                &format_args!("{}", self.hp_reg_tcm_init_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_TCM_INIT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_init_en(&mut self) -> HP_REG_TCM_INIT_EN_W<HP_TCM_INIT_SPEC> {
        HP_REG_TCM_INIT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset tcm init cnt"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_init_cnt_reset(&mut self) -> HP_REG_TCM_INIT_CNT_RESET_W<HP_TCM_INIT_SPEC> {
        HP_REG_TCM_INIT_CNT_RESET_W::new(self, 1)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_TCM_INIT_SPEC;
impl crate::RegisterSpec for HP_TCM_INIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_tcm_init::R`](R) reader structure"]
impl crate::Readable for HP_TCM_INIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_tcm_init::W`](W) writer structure"]
impl crate::Writable for HP_TCM_INIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_TCM_INIT to value 0x02"]
impl crate::Resettable for HP_TCM_INIT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}

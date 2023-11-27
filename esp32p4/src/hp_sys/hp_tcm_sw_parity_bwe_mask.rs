#[doc = "Register `HP_TCM_SW_PARITY_BWE_MASK` reader"]
pub type R = crate::R<HP_TCM_SW_PARITY_BWE_MASK_SPEC>;
#[doc = "Register `HP_TCM_SW_PARITY_BWE_MASK` writer"]
pub type W = crate::W<HP_TCM_SW_PARITY_BWE_MASK_SPEC>;
#[doc = "Field `HP_REG_TCM_SW_PARITY_BWE_MASK_CTRL` reader - Set 1 to mask tcm bwe parity code bit"]
pub type HP_REG_TCM_SW_PARITY_BWE_MASK_CTRL_R = crate::BitReader;
#[doc = "Field `HP_REG_TCM_SW_PARITY_BWE_MASK_CTRL` writer - Set 1 to mask tcm bwe parity code bit"]
pub type HP_REG_TCM_SW_PARITY_BWE_MASK_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to mask tcm bwe parity code bit"]
    #[inline(always)]
    pub fn hp_reg_tcm_sw_parity_bwe_mask_ctrl(&self) -> HP_REG_TCM_SW_PARITY_BWE_MASK_CTRL_R {
        HP_REG_TCM_SW_PARITY_BWE_MASK_CTRL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_TCM_SW_PARITY_BWE_MASK")
            .field(
                "hp_reg_tcm_sw_parity_bwe_mask_ctrl",
                &format_args!("{}", self.hp_reg_tcm_sw_parity_bwe_mask_ctrl().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_TCM_SW_PARITY_BWE_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to mask tcm bwe parity code bit"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_sw_parity_bwe_mask_ctrl(
        &mut self,
    ) -> HP_REG_TCM_SW_PARITY_BWE_MASK_CTRL_W<HP_TCM_SW_PARITY_BWE_MASK_SPEC> {
        HP_REG_TCM_SW_PARITY_BWE_MASK_CTRL_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_sw_parity_bwe_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_sw_parity_bwe_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_TCM_SW_PARITY_BWE_MASK_SPEC;
impl crate::RegisterSpec for HP_TCM_SW_PARITY_BWE_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_tcm_sw_parity_bwe_mask::R`](R) reader structure"]
impl crate::Readable for HP_TCM_SW_PARITY_BWE_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_tcm_sw_parity_bwe_mask::W`](W) writer structure"]
impl crate::Writable for HP_TCM_SW_PARITY_BWE_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_TCM_SW_PARITY_BWE_MASK to value 0"]
impl crate::Resettable for HP_TCM_SW_PARITY_BWE_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

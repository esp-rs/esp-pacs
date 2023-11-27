#[doc = "Register `HP_L2_CACHE_PWR_CTRL` reader"]
pub type R = crate::R<HP_L2_CACHE_PWR_CTRL_SPEC>;
#[doc = "Register `HP_L2_CACHE_PWR_CTRL` writer"]
pub type W = crate::W<HP_L2_CACHE_PWR_CTRL_SPEC>;
#[doc = "Field `HP_REG_L2_CACHE_MEM_FO` reader - need_des"]
pub type HP_REG_L2_CACHE_MEM_FO_R = crate::FieldReader;
#[doc = "Field `HP_REG_L2_CACHE_MEM_FO` writer - need_des"]
pub type HP_REG_L2_CACHE_MEM_FO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn hp_reg_l2_cache_mem_fo(&self) -> HP_REG_L2_CACHE_MEM_FO_R {
        HP_REG_L2_CACHE_MEM_FO_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_L2_CACHE_PWR_CTRL")
            .field(
                "hp_reg_l2_cache_mem_fo",
                &format_args!("{}", self.hp_reg_l2_cache_mem_fo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_L2_CACHE_PWR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_cache_mem_fo(
        &mut self,
    ) -> HP_REG_L2_CACHE_MEM_FO_W<HP_L2_CACHE_PWR_CTRL_SPEC> {
        HP_REG_L2_CACHE_MEM_FO_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_cache_pwr_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_cache_pwr_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_L2_CACHE_PWR_CTRL_SPEC;
impl crate::RegisterSpec for HP_L2_CACHE_PWR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_l2_cache_pwr_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_L2_CACHE_PWR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_l2_cache_pwr_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_L2_CACHE_PWR_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_L2_CACHE_PWR_CTRL to value 0"]
impl crate::Resettable for HP_L2_CACHE_PWR_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

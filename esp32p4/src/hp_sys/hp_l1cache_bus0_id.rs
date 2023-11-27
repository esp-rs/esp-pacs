#[doc = "Register `HP_L1CACHE_BUS0_ID` reader"]
pub type R = crate::R<HP_L1CACHE_BUS0_ID_SPEC>;
#[doc = "Register `HP_L1CACHE_BUS0_ID` writer"]
pub type W = crate::W<HP_L1CACHE_BUS0_ID_SPEC>;
#[doc = "Field `HP_REG_L1_CACHE_BUS0_ID` reader - NA"]
pub type HP_REG_L1_CACHE_BUS0_ID_R = crate::FieldReader;
#[doc = "Field `HP_REG_L1_CACHE_BUS0_ID` writer - NA"]
pub type HP_REG_L1_CACHE_BUS0_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn hp_reg_l1_cache_bus0_id(&self) -> HP_REG_L1_CACHE_BUS0_ID_R {
        HP_REG_L1_CACHE_BUS0_ID_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_L1CACHE_BUS0_ID")
            .field(
                "hp_reg_l1_cache_bus0_id",
                &format_args!("{}", self.hp_reg_l1_cache_bus0_id().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_L1CACHE_BUS0_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l1_cache_bus0_id(
        &mut self,
    ) -> HP_REG_L1_CACHE_BUS0_ID_W<HP_L1CACHE_BUS0_ID_SPEC> {
        HP_REG_L1_CACHE_BUS0_ID_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l1cache_bus0_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l1cache_bus0_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_L1CACHE_BUS0_ID_SPEC;
impl crate::RegisterSpec for HP_L1CACHE_BUS0_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_l1cache_bus0_id::R`](R) reader structure"]
impl crate::Readable for HP_L1CACHE_BUS0_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_l1cache_bus0_id::W`](W) writer structure"]
impl crate::Writable for HP_L1CACHE_BUS0_ID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_L1CACHE_BUS0_ID to value 0"]
impl crate::Resettable for HP_L1CACHE_BUS0_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

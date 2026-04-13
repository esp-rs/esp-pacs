#[doc = "Register `SYSTEM_HP2LP_INTR_GROUP%s_EN` reader"]
pub type R = crate::R<SYSTEM_HP2LP_INTR_GROUP_EN_SPEC>;
#[doc = "Register `SYSTEM_HP2LP_INTR_GROUP%s_EN` writer"]
pub type W = crate::W<SYSTEM_HP2LP_INTR_GROUP_EN_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTEM_HP2LP_INTR_GROUP_EN")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, SYSTEM_HP2LP_INTR_GROUP_EN_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "HP to LP interrupt group %s enable\n\nYou can [`read`](crate::Reg::read) this register and get [`system_hp2lp_intr_group_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`system_hp2lp_intr_group_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTEM_HP2LP_INTR_GROUP_EN_SPEC;
impl crate::RegisterSpec for SYSTEM_HP2LP_INTR_GROUP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_hp2lp_intr_group_en::R`](R) reader structure"]
impl crate::Readable for SYSTEM_HP2LP_INTR_GROUP_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`system_hp2lp_intr_group_en::W`](W) writer structure"]
impl crate::Writable for SYSTEM_HP2LP_INTR_GROUP_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTEM_HP2LP_INTR_GROUP%s_EN to value 0"]
impl crate::Resettable for SYSTEM_HP2LP_INTR_GROUP_EN_SPEC {}

#[doc = "Register `HP2LP_INTR_GROUP3_EN` reader"]
pub type R = crate::R<HP2LP_INTR_GROUP3_EN_SPEC>;
#[doc = "Register `HP2LP_INTR_GROUP3_EN` writer"]
pub type W = crate::W<HP2LP_INTR_GROUP3_EN_SPEC>;
#[doc = "Field `H2LP_INTR_GROUP3_EN` reader - Set each bit to enable corresponding peripheral interrupt to LP CPU."]
pub type H2LP_INTR_GROUP3_EN_R = crate::FieldReader<u16>;
#[doc = "Field `H2LP_INTR_GROUP3_EN` writer - Set each bit to enable corresponding peripheral interrupt to LP CPU."]
pub type H2LP_INTR_GROUP3_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Set each bit to enable corresponding peripheral interrupt to LP CPU."]
    #[inline(always)]
    pub fn h2lp_intr_group3_en(&self) -> H2LP_INTR_GROUP3_EN_R {
        H2LP_INTR_GROUP3_EN_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP2LP_INTR_GROUP3_EN")
            .field("h2lp_intr_group3_en", &self.h2lp_intr_group3_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Set each bit to enable corresponding peripheral interrupt to LP CPU."]
    #[inline(always)]
    pub fn h2lp_intr_group3_en(&mut self) -> H2LP_INTR_GROUP3_EN_W<'_, HP2LP_INTR_GROUP3_EN_SPEC> {
        H2LP_INTR_GROUP3_EN_W::new(self, 0)
    }
}
#[doc = "HpP2LP Interrupt Enable Register Group3\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group3_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_intr_group3_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP2LP_INTR_GROUP3_EN_SPEC;
impl crate::RegisterSpec for HP2LP_INTR_GROUP3_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp2lp_intr_group3_en::R`](R) reader structure"]
impl crate::Readable for HP2LP_INTR_GROUP3_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp2lp_intr_group3_en::W`](W) writer structure"]
impl crate::Writable for HP2LP_INTR_GROUP3_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP2LP_INTR_GROUP3_EN to value 0x3fff"]
impl crate::Resettable for HP2LP_INTR_GROUP3_EN_SPEC {
    const RESET_VALUE: u32 = 0x3fff;
}

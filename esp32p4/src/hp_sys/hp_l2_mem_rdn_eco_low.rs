#[doc = "Register `HP_L2_MEM_RDN_ECO_LOW` reader"]
pub type R = crate::R<HP_L2_MEM_RDN_ECO_LOW_SPEC>;
#[doc = "Register `HP_L2_MEM_RDN_ECO_LOW` writer"]
pub type W = crate::W<HP_L2_MEM_RDN_ECO_LOW_SPEC>;
#[doc = "Field `HP_REG_L2_MEM_RDN_ECO_LOW` reader - NA"]
pub type HP_REG_L2_MEM_RDN_ECO_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `HP_REG_L2_MEM_RDN_ECO_LOW` writer - NA"]
pub type HP_REG_L2_MEM_RDN_ECO_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_mem_rdn_eco_low(&self) -> HP_REG_L2_MEM_RDN_ECO_LOW_R {
        HP_REG_L2_MEM_RDN_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_L2_MEM_RDN_ECO_LOW")
            .field(
                "hp_reg_l2_mem_rdn_eco_low",
                &format_args!("{}", self.hp_reg_l2_mem_rdn_eco_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_L2_MEM_RDN_ECO_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_mem_rdn_eco_low(
        &mut self,
    ) -> HP_REG_L2_MEM_RDN_ECO_LOW_W<HP_L2_MEM_RDN_ECO_LOW_SPEC> {
        HP_REG_L2_MEM_RDN_ECO_LOW_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_rdn_eco_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_rdn_eco_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_L2_MEM_RDN_ECO_LOW_SPEC;
impl crate::RegisterSpec for HP_L2_MEM_RDN_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_l2_mem_rdn_eco_low::R`](R) reader structure"]
impl crate::Readable for HP_L2_MEM_RDN_ECO_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_l2_mem_rdn_eco_low::W`](W) writer structure"]
impl crate::Writable for HP_L2_MEM_RDN_ECO_LOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_L2_MEM_RDN_ECO_LOW to value 0"]
impl crate::Resettable for HP_L2_MEM_RDN_ECO_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

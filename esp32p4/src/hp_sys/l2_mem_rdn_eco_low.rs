#[doc = "Register `L2_MEM_RDN_ECO_LOW` reader"]
pub type R = crate::R<L2_MEM_RDN_ECO_LOW_SPEC>;
#[doc = "Register `L2_MEM_RDN_ECO_LOW` writer"]
pub type W = crate::W<L2_MEM_RDN_ECO_LOW_SPEC>;
#[doc = "Field `REG_L2_MEM_RDN_ECO_LOW` reader - NA"]
pub type REG_L2_MEM_RDN_ECO_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `REG_L2_MEM_RDN_ECO_LOW` writer - NA"]
pub type REG_L2_MEM_RDN_ECO_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_low(&self) -> REG_L2_MEM_RDN_ECO_LOW_R {
        REG_L2_MEM_RDN_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_RDN_ECO_LOW")
            .field(
                "reg_l2_mem_rdn_eco_low",
                &format_args!("{}", self.reg_l2_mem_rdn_eco_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_MEM_RDN_ECO_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_rdn_eco_low(&mut self) -> REG_L2_MEM_RDN_ECO_LOW_W<L2_MEM_RDN_ECO_LOW_SPEC> {
        REG_L2_MEM_RDN_ECO_LOW_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_rdn_eco_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_rdn_eco_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_RDN_ECO_LOW_SPEC;
impl crate::RegisterSpec for L2_MEM_RDN_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_rdn_eco_low::R`](R) reader structure"]
impl crate::Readable for L2_MEM_RDN_ECO_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_rdn_eco_low::W`](W) writer structure"]
impl crate::Writable for L2_MEM_RDN_ECO_LOW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_MEM_RDN_ECO_LOW to value 0"]
impl crate::Resettable for L2_MEM_RDN_ECO_LOW_SPEC {
    const RESET_VALUE: u32 = 0;
}

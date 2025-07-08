#[doc = "Register `FORCE_ACC_HP` reader"]
pub type R = crate::R<FORCE_ACC_HP_SPEC>;
#[doc = "Register `FORCE_ACC_HP` writer"]
pub type W = crate::W<FORCE_ACC_HP_SPEC>;
#[doc = "Field `FORCE_ACC_HPMEM_EN` reader - Configures whether to allow LP CPU to force access to HP_MEM regardless of permission management.\\\\ 0: disable force access HP_MEM \\\\ 1: enable force access HP_MEM \\\\"]
pub type FORCE_ACC_HPMEM_EN_R = crate::BitReader;
#[doc = "Field `FORCE_ACC_HPMEM_EN` writer - Configures whether to allow LP CPU to force access to HP_MEM regardless of permission management.\\\\ 0: disable force access HP_MEM \\\\ 1: enable force access HP_MEM \\\\"]
pub type FORCE_ACC_HPMEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether to allow LP CPU to force access to HP_MEM regardless of permission management.\\\\ 0: disable force access HP_MEM \\\\ 1: enable force access HP_MEM \\\\"]
    #[inline(always)]
    pub fn force_acc_hpmem_en(&self) -> FORCE_ACC_HPMEM_EN_R {
        FORCE_ACC_HPMEM_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FORCE_ACC_HP")
            .field("force_acc_hpmem_en", &self.force_acc_hpmem_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to allow LP CPU to force access to HP_MEM regardless of permission management.\\\\ 0: disable force access HP_MEM \\\\ 1: enable force access HP_MEM \\\\"]
    #[inline(always)]
    pub fn force_acc_hpmem_en(&mut self) -> FORCE_ACC_HPMEM_EN_W<FORCE_ACC_HP_SPEC> {
        FORCE_ACC_HPMEM_EN_W::new(self, 0)
    }
}
#[doc = "Force access to hpmem configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`force_acc_hp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_acc_hp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FORCE_ACC_HP_SPEC;
impl crate::RegisterSpec for FORCE_ACC_HP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`force_acc_hp::R`](R) reader structure"]
impl crate::Readable for FORCE_ACC_HP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`force_acc_hp::W`](W) writer structure"]
impl crate::Writable for FORCE_ACC_HP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FORCE_ACC_HP to value 0"]
impl crate::Resettable for FORCE_ACC_HP_SPEC {}

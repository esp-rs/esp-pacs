#[doc = "Register `FE_CFG` reader"]
pub type R = crate::R<FE_CFG_SPEC>;
#[doc = "Register `FE_CFG` writer"]
pub type W = crate::W<FE_CFG_SPEC>;
#[doc = "Field `FE_CFG` reader - "]
pub type FE_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `FE_CFG` writer - "]
pub type FE_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fe_cfg(&self) -> FE_CFG_R {
        FE_CFG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FE_CFG")
            .field("fe_cfg", &self.fe_cfg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fe_cfg(&mut self) -> FE_CFG_W<'_, FE_CFG_SPEC> {
        FE_CFG_W::new(self, 0)
    }
}
#[doc = "FE_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`fe_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fe_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FE_CFG_SPEC;
impl crate::RegisterSpec for FE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fe_cfg::R`](R) reader structure"]
impl crate::Readable for FE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fe_cfg::W`](W) writer structure"]
impl crate::Writable for FE_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FE_CFG to value 0"]
impl crate::Resettable for FE_CFG_SPEC {}

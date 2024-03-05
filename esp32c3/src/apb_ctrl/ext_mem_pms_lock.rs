#[doc = "Register `EXT_MEM_PMS_LOCK` reader"]
pub type R = crate::R<EXT_MEM_PMS_LOCK_SPEC>;
#[doc = "Register `EXT_MEM_PMS_LOCK` writer"]
pub type W = crate::W<EXT_MEM_PMS_LOCK_SPEC>;
#[doc = "Field `EXT_MEM_PMS_LOCK` reader - reg_ext_mem_pms_lock"]
pub type EXT_MEM_PMS_LOCK_R = crate::BitReader;
#[doc = "Field `EXT_MEM_PMS_LOCK` writer - reg_ext_mem_pms_lock"]
pub type EXT_MEM_PMS_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_ext_mem_pms_lock"]
    #[inline(always)]
    pub fn ext_mem_pms_lock(&self) -> EXT_MEM_PMS_LOCK_R {
        EXT_MEM_PMS_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_MEM_PMS_LOCK")
            .field(
                "ext_mem_pms_lock",
                &format_args!("{}", self.ext_mem_pms_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_MEM_PMS_LOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_ext_mem_pms_lock"]
    #[inline(always)]
    #[must_use]
    pub fn ext_mem_pms_lock(&mut self) -> EXT_MEM_PMS_LOCK_W<EXT_MEM_PMS_LOCK_SPEC> {
        EXT_MEM_PMS_LOCK_W::new(self, 0)
    }
}
#[doc = "APB_CTRL_EXT_MEM_PMS_LOCK_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_mem_pms_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_mem_pms_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_MEM_PMS_LOCK_SPEC;
impl crate::RegisterSpec for EXT_MEM_PMS_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_mem_pms_lock::R`](R) reader structure"]
impl crate::Readable for EXT_MEM_PMS_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_mem_pms_lock::W`](W) writer structure"]
impl crate::Writable for EXT_MEM_PMS_LOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_MEM_PMS_LOCK to value 0"]
impl crate::Resettable for EXT_MEM_PMS_LOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}

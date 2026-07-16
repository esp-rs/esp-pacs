#[doc = "Register `SUPERVISOR_PRIV_SEL` reader"]
pub type R = crate::R<SUPERVISOR_PRIV_SEL_SPEC>;
#[doc = "Register `SUPERVISOR_PRIV_SEL` writer"]
pub type W = crate::W<SUPERVISOR_PRIV_SEL_SPEC>;
#[doc = "Field `CORE0_SUPERVISOR_PRIV_SEL` reader - Configures the privilege of the core0 supervisor mode .\\\\ 0: same as user_mode \\\\ 1: same as machine_mode \\\\"]
pub type CORE0_SUPERVISOR_PRIV_SEL_R = crate::BitReader;
#[doc = "Field `CORE0_SUPERVISOR_PRIV_SEL` writer - Configures the privilege of the core0 supervisor mode .\\\\ 0: same as user_mode \\\\ 1: same as machine_mode \\\\"]
pub type CORE0_SUPERVISOR_PRIV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_SUPERVISOR_PRIV_SEL` reader - Configures the privilege of the core0 supervisor mode .\\\\ 0: same as user_mode \\\\ 1: same as machine_mode \\\\"]
pub type CORE1_SUPERVISOR_PRIV_SEL_R = crate::BitReader;
#[doc = "Field `CORE1_SUPERVISOR_PRIV_SEL` writer - Configures the privilege of the core0 supervisor mode .\\\\ 0: same as user_mode \\\\ 1: same as machine_mode \\\\"]
pub type CORE1_SUPERVISOR_PRIV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_SUPERVISOR_PRIV_SEL_LOCK` reader - Set 1 to lock reg_core0_supervisor_priv_sel configuration"]
pub type CORE0_SUPERVISOR_PRIV_SEL_LOCK_R = crate::BitReader;
#[doc = "Field `CORE0_SUPERVISOR_PRIV_SEL_LOCK` writer - Set 1 to lock reg_core0_supervisor_priv_sel configuration"]
pub type CORE0_SUPERVISOR_PRIV_SEL_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_SUPERVISOR_PRIV_SEL_LOCK` reader - Set 1 to lock reg_core1_supervisor_priv_sel configuration"]
pub type CORE1_SUPERVISOR_PRIV_SEL_LOCK_R = crate::BitReader;
#[doc = "Field `CORE1_SUPERVISOR_PRIV_SEL_LOCK` writer - Set 1 to lock reg_core1_supervisor_priv_sel configuration"]
pub type CORE1_SUPERVISOR_PRIV_SEL_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the privilege of the core0 supervisor mode .\\\\ 0: same as user_mode \\\\ 1: same as machine_mode \\\\"]
    #[inline(always)]
    pub fn core0_supervisor_priv_sel(&self) -> CORE0_SUPERVISOR_PRIV_SEL_R {
        CORE0_SUPERVISOR_PRIV_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures the privilege of the core0 supervisor mode .\\\\ 0: same as user_mode \\\\ 1: same as machine_mode \\\\"]
    #[inline(always)]
    pub fn core1_supervisor_priv_sel(&self) -> CORE1_SUPERVISOR_PRIV_SEL_R {
        CORE1_SUPERVISOR_PRIV_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to lock reg_core0_supervisor_priv_sel configuration"]
    #[inline(always)]
    pub fn core0_supervisor_priv_sel_lock(&self) -> CORE0_SUPERVISOR_PRIV_SEL_LOCK_R {
        CORE0_SUPERVISOR_PRIV_SEL_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to lock reg_core1_supervisor_priv_sel configuration"]
    #[inline(always)]
    pub fn core1_supervisor_priv_sel_lock(&self) -> CORE1_SUPERVISOR_PRIV_SEL_LOCK_R {
        CORE1_SUPERVISOR_PRIV_SEL_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUPERVISOR_PRIV_SEL")
            .field(
                "core0_supervisor_priv_sel",
                &self.core0_supervisor_priv_sel(),
            )
            .field(
                "core1_supervisor_priv_sel",
                &self.core1_supervisor_priv_sel(),
            )
            .field(
                "core0_supervisor_priv_sel_lock",
                &self.core0_supervisor_priv_sel_lock(),
            )
            .field(
                "core1_supervisor_priv_sel_lock",
                &self.core1_supervisor_priv_sel_lock(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the privilege of the core0 supervisor mode .\\\\ 0: same as user_mode \\\\ 1: same as machine_mode \\\\"]
    #[inline(always)]
    pub fn core0_supervisor_priv_sel(
        &mut self,
    ) -> CORE0_SUPERVISOR_PRIV_SEL_W<'_, SUPERVISOR_PRIV_SEL_SPEC> {
        CORE0_SUPERVISOR_PRIV_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures the privilege of the core0 supervisor mode .\\\\ 0: same as user_mode \\\\ 1: same as machine_mode \\\\"]
    #[inline(always)]
    pub fn core1_supervisor_priv_sel(
        &mut self,
    ) -> CORE1_SUPERVISOR_PRIV_SEL_W<'_, SUPERVISOR_PRIV_SEL_SPEC> {
        CORE1_SUPERVISOR_PRIV_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to lock reg_core0_supervisor_priv_sel configuration"]
    #[inline(always)]
    pub fn core0_supervisor_priv_sel_lock(
        &mut self,
    ) -> CORE0_SUPERVISOR_PRIV_SEL_LOCK_W<'_, SUPERVISOR_PRIV_SEL_SPEC> {
        CORE0_SUPERVISOR_PRIV_SEL_LOCK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set 1 to lock reg_core1_supervisor_priv_sel configuration"]
    #[inline(always)]
    pub fn core1_supervisor_priv_sel_lock(
        &mut self,
    ) -> CORE1_SUPERVISOR_PRIV_SEL_LOCK_W<'_, SUPERVISOR_PRIV_SEL_SPEC> {
        CORE1_SUPERVISOR_PRIV_SEL_LOCK_W::new(self, 3)
    }
}
#[doc = "TEE mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`supervisor_priv_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`supervisor_priv_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUPERVISOR_PRIV_SEL_SPEC;
impl crate::RegisterSpec for SUPERVISOR_PRIV_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`supervisor_priv_sel::R`](R) reader structure"]
impl crate::Readable for SUPERVISOR_PRIV_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`supervisor_priv_sel::W`](W) writer structure"]
impl crate::Writable for SUPERVISOR_PRIV_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUPERVISOR_PRIV_SEL to value 0"]
impl crate::Resettable for SUPERVISOR_PRIV_SEL_SPEC {}

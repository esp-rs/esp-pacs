#[doc = "Register `CPU_APM_CTRL` reader"]
pub type R = crate::R<CPU_APM_CTRL_SPEC>;
#[doc = "Register `CPU_APM_CTRL` writer"]
pub type W = crate::W<CPU_APM_CTRL_SPEC>;
#[doc = "Field `READ_TEE_CPU_APM` reader - Configures cpu_apm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_CPU_APM_R = crate::BitReader;
#[doc = "Field `READ_TEE_CPU_APM` writer - Configures cpu_apm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_CPU_APM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_CPU_APM` reader - Configures cpu_apm registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_CPU_APM_R = crate::BitReader;
#[doc = "Field `READ_REE1_CPU_APM` reader - Configures cpu_apm registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_CPU_APM_R = crate::BitReader;
#[doc = "Field `READ_REE2_CPU_APM` reader - Configures cpu_apm registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_CPU_APM_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_CPU_APM` reader - Configures cpu_apm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_CPU_APM_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_CPU_APM` writer - Configures cpu_apm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_CPU_APM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_CPU_APM` reader - Configures cpu_apm registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_CPU_APM_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_CPU_APM` reader - Configures cpu_apm registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_CPU_APM_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_CPU_APM` reader - Configures cpu_apm registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_CPU_APM_R = crate::BitReader;
#[doc = "Field `CPU_APM_LOCK` reader - Set 1 to lock cpu_apm peri_apm configuration"]
pub type CPU_APM_LOCK_R = crate::BitReader;
#[doc = "Field `CPU_APM_LOCK` writer - Set 1 to lock cpu_apm peri_apm configuration"]
pub type CPU_APM_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures cpu_apm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_cpu_apm(&self) -> READ_TEE_CPU_APM_R {
        READ_TEE_CPU_APM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures cpu_apm registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_cpu_apm(&self) -> READ_REE0_CPU_APM_R {
        READ_REE0_CPU_APM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures cpu_apm registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_cpu_apm(&self) -> READ_REE1_CPU_APM_R {
        READ_REE1_CPU_APM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures cpu_apm registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_cpu_apm(&self) -> READ_REE2_CPU_APM_R {
        READ_REE2_CPU_APM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures cpu_apm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_cpu_apm(&self) -> WRITE_TEE_CPU_APM_R {
        WRITE_TEE_CPU_APM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures cpu_apm registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_cpu_apm(&self) -> WRITE_REE0_CPU_APM_R {
        WRITE_REE0_CPU_APM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures cpu_apm registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_cpu_apm(&self) -> WRITE_REE1_CPU_APM_R {
        WRITE_REE1_CPU_APM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures cpu_apm registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_cpu_apm(&self) -> WRITE_REE2_CPU_APM_R {
        WRITE_REE2_CPU_APM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to lock cpu_apm peri_apm configuration"]
    #[inline(always)]
    pub fn cpu_apm_lock(&self) -> CPU_APM_LOCK_R {
        CPU_APM_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_APM_CTRL")
            .field("read_tee_cpu_apm", &self.read_tee_cpu_apm())
            .field("read_ree0_cpu_apm", &self.read_ree0_cpu_apm())
            .field("read_ree1_cpu_apm", &self.read_ree1_cpu_apm())
            .field("read_ree2_cpu_apm", &self.read_ree2_cpu_apm())
            .field("write_tee_cpu_apm", &self.write_tee_cpu_apm())
            .field("write_ree0_cpu_apm", &self.write_ree0_cpu_apm())
            .field("write_ree1_cpu_apm", &self.write_ree1_cpu_apm())
            .field("write_ree2_cpu_apm", &self.write_ree2_cpu_apm())
            .field("cpu_apm_lock", &self.cpu_apm_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures cpu_apm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_cpu_apm(&mut self) -> READ_TEE_CPU_APM_W<'_, CPU_APM_CTRL_SPEC> {
        READ_TEE_CPU_APM_W::new(self, 0)
    }
    #[doc = "Bit 4 - Configures cpu_apm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_cpu_apm(&mut self) -> WRITE_TEE_CPU_APM_W<'_, CPU_APM_CTRL_SPEC> {
        WRITE_TEE_CPU_APM_W::new(self, 4)
    }
    #[doc = "Bit 8 - Set 1 to lock cpu_apm peri_apm configuration"]
    #[inline(always)]
    pub fn cpu_apm_lock(&mut self) -> CPU_APM_LOCK_W<'_, CPU_APM_CTRL_SPEC> {
        CPU_APM_LOCK_W::new(self, 8)
    }
}
#[doc = "cpu_apm read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_apm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_apm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_APM_CTRL_SPEC;
impl crate::RegisterSpec for CPU_APM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_apm_ctrl::R`](R) reader structure"]
impl crate::Readable for CPU_APM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_apm_ctrl::W`](W) writer structure"]
impl crate::Writable for CPU_APM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_APM_CTRL to value 0x11"]
impl crate::Resettable for CPU_APM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}

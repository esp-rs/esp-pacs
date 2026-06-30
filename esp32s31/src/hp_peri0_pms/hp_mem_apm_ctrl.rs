#[doc = "Register `HP_MEM_APM_CTRL` reader"]
pub type R = crate::R<HP_MEM_APM_CTRL_SPEC>;
#[doc = "Register `HP_MEM_APM_CTRL` writer"]
pub type W = crate::W<HP_MEM_APM_CTRL_SPEC>;
#[doc = "Field `READ_TEE_HP_MEM_APM` reader - Configures hp_mem_apm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_HP_MEM_APM_R = crate::BitReader;
#[doc = "Field `READ_TEE_HP_MEM_APM` writer - Configures hp_mem_apm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_HP_MEM_APM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_HP_MEM_APM` reader - Configures hp_mem_apm registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_HP_MEM_APM_R = crate::BitReader;
#[doc = "Field `READ_REE1_HP_MEM_APM` reader - Configures hp_mem_apm registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_HP_MEM_APM_R = crate::BitReader;
#[doc = "Field `READ_REE2_HP_MEM_APM` reader - Configures hp_mem_apm registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_HP_MEM_APM_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_HP_MEM_APM` reader - Configures hp_mem_apm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_HP_MEM_APM_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_HP_MEM_APM` writer - Configures hp_mem_apm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_HP_MEM_APM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_HP_MEM_APM` reader - Configures hp_mem_apm registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_HP_MEM_APM_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_HP_MEM_APM` reader - Configures hp_mem_apm registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_HP_MEM_APM_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_HP_MEM_APM` reader - Configures hp_mem_apm registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_HP_MEM_APM_R = crate::BitReader;
#[doc = "Field `HP_MEM_APM_LOCK` reader - Set 1 to lock hp_mem_apm peri_apm configuration"]
pub type HP_MEM_APM_LOCK_R = crate::BitReader;
#[doc = "Field `HP_MEM_APM_LOCK` writer - Set 1 to lock hp_mem_apm peri_apm configuration"]
pub type HP_MEM_APM_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures hp_mem_apm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_hp_mem_apm(&self) -> READ_TEE_HP_MEM_APM_R {
        READ_TEE_HP_MEM_APM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures hp_mem_apm registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_hp_mem_apm(&self) -> READ_REE0_HP_MEM_APM_R {
        READ_REE0_HP_MEM_APM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures hp_mem_apm registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_hp_mem_apm(&self) -> READ_REE1_HP_MEM_APM_R {
        READ_REE1_HP_MEM_APM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures hp_mem_apm registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_hp_mem_apm(&self) -> READ_REE2_HP_MEM_APM_R {
        READ_REE2_HP_MEM_APM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures hp_mem_apm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_hp_mem_apm(&self) -> WRITE_TEE_HP_MEM_APM_R {
        WRITE_TEE_HP_MEM_APM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures hp_mem_apm registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_hp_mem_apm(&self) -> WRITE_REE0_HP_MEM_APM_R {
        WRITE_REE0_HP_MEM_APM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures hp_mem_apm registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_hp_mem_apm(&self) -> WRITE_REE1_HP_MEM_APM_R {
        WRITE_REE1_HP_MEM_APM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures hp_mem_apm registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_hp_mem_apm(&self) -> WRITE_REE2_HP_MEM_APM_R {
        WRITE_REE2_HP_MEM_APM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to lock hp_mem_apm peri_apm configuration"]
    #[inline(always)]
    pub fn hp_mem_apm_lock(&self) -> HP_MEM_APM_LOCK_R {
        HP_MEM_APM_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MEM_APM_CTRL")
            .field("read_tee_hp_mem_apm", &self.read_tee_hp_mem_apm())
            .field("read_ree0_hp_mem_apm", &self.read_ree0_hp_mem_apm())
            .field("read_ree1_hp_mem_apm", &self.read_ree1_hp_mem_apm())
            .field("read_ree2_hp_mem_apm", &self.read_ree2_hp_mem_apm())
            .field("write_tee_hp_mem_apm", &self.write_tee_hp_mem_apm())
            .field("write_ree0_hp_mem_apm", &self.write_ree0_hp_mem_apm())
            .field("write_ree1_hp_mem_apm", &self.write_ree1_hp_mem_apm())
            .field("write_ree2_hp_mem_apm", &self.write_ree2_hp_mem_apm())
            .field("hp_mem_apm_lock", &self.hp_mem_apm_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures hp_mem_apm registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_hp_mem_apm(&mut self) -> READ_TEE_HP_MEM_APM_W<'_, HP_MEM_APM_CTRL_SPEC> {
        READ_TEE_HP_MEM_APM_W::new(self, 0)
    }
    #[doc = "Bit 4 - Configures hp_mem_apm registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_hp_mem_apm(&mut self) -> WRITE_TEE_HP_MEM_APM_W<'_, HP_MEM_APM_CTRL_SPEC> {
        WRITE_TEE_HP_MEM_APM_W::new(self, 4)
    }
    #[doc = "Bit 8 - Set 1 to lock hp_mem_apm peri_apm configuration"]
    #[inline(always)]
    pub fn hp_mem_apm_lock(&mut self) -> HP_MEM_APM_LOCK_W<'_, HP_MEM_APM_CTRL_SPEC> {
        HP_MEM_APM_LOCK_W::new(self, 8)
    }
}
#[doc = "hp_mem_apm read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mem_apm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mem_apm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MEM_APM_CTRL_SPEC;
impl crate::RegisterSpec for HP_MEM_APM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_mem_apm_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_MEM_APM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_mem_apm_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_MEM_APM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MEM_APM_CTRL to value 0x11"]
impl crate::Resettable for HP_MEM_APM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}

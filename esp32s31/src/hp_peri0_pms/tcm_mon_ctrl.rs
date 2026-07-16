#[doc = "Register `TCM_MON_CTRL` reader"]
pub type R = crate::R<TCM_MON_CTRL_SPEC>;
#[doc = "Register `TCM_MON_CTRL` writer"]
pub type W = crate::W<TCM_MON_CTRL_SPEC>;
#[doc = "Field `READ_TEE_TCM_MON` reader - Configures tcm_mon registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_TCM_MON_R = crate::BitReader;
#[doc = "Field `READ_TEE_TCM_MON` writer - Configures tcm_mon registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_TCM_MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_TCM_MON` reader - Configures tcm_mon registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_TCM_MON_R = crate::BitReader;
#[doc = "Field `READ_REE0_TCM_MON` writer - Configures tcm_mon registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_TCM_MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_TCM_MON` reader - Configures tcm_mon registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_TCM_MON_R = crate::BitReader;
#[doc = "Field `READ_REE1_TCM_MON` writer - Configures tcm_mon registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_TCM_MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_TCM_MON` reader - Configures tcm_mon registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_TCM_MON_R = crate::BitReader;
#[doc = "Field `READ_REE2_TCM_MON` writer - Configures tcm_mon registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_TCM_MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_TCM_MON` reader - Configures tcm_mon registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_TCM_MON_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_TCM_MON` writer - Configures tcm_mon registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_TCM_MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_TCM_MON` reader - Configures tcm_mon registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_TCM_MON_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_TCM_MON` writer - Configures tcm_mon registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_TCM_MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_TCM_MON` reader - Configures tcm_mon registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_TCM_MON_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_TCM_MON` writer - Configures tcm_mon registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_TCM_MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_TCM_MON` reader - Configures tcm_mon registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_TCM_MON_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_TCM_MON` writer - Configures tcm_mon registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_TCM_MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCM_MON_LOCK` reader - Set 1 to lock tcm_mon peri_apm configuration"]
pub type TCM_MON_LOCK_R = crate::BitReader;
#[doc = "Field `TCM_MON_LOCK` writer - Set 1 to lock tcm_mon peri_apm configuration"]
pub type TCM_MON_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures tcm_mon registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_tcm_mon(&self) -> READ_TEE_TCM_MON_R {
        READ_TEE_TCM_MON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures tcm_mon registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_tcm_mon(&self) -> READ_REE0_TCM_MON_R {
        READ_REE0_TCM_MON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures tcm_mon registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_tcm_mon(&self) -> READ_REE1_TCM_MON_R {
        READ_REE1_TCM_MON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures tcm_mon registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_tcm_mon(&self) -> READ_REE2_TCM_MON_R {
        READ_REE2_TCM_MON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures tcm_mon registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_tcm_mon(&self) -> WRITE_TEE_TCM_MON_R {
        WRITE_TEE_TCM_MON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures tcm_mon registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_tcm_mon(&self) -> WRITE_REE0_TCM_MON_R {
        WRITE_REE0_TCM_MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures tcm_mon registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_tcm_mon(&self) -> WRITE_REE1_TCM_MON_R {
        WRITE_REE1_TCM_MON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures tcm_mon registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_tcm_mon(&self) -> WRITE_REE2_TCM_MON_R {
        WRITE_REE2_TCM_MON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to lock tcm_mon peri_apm configuration"]
    #[inline(always)]
    pub fn tcm_mon_lock(&self) -> TCM_MON_LOCK_R {
        TCM_MON_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_MON_CTRL")
            .field("read_tee_tcm_mon", &self.read_tee_tcm_mon())
            .field("read_ree0_tcm_mon", &self.read_ree0_tcm_mon())
            .field("read_ree1_tcm_mon", &self.read_ree1_tcm_mon())
            .field("read_ree2_tcm_mon", &self.read_ree2_tcm_mon())
            .field("write_tee_tcm_mon", &self.write_tee_tcm_mon())
            .field("write_ree0_tcm_mon", &self.write_ree0_tcm_mon())
            .field("write_ree1_tcm_mon", &self.write_ree1_tcm_mon())
            .field("write_ree2_tcm_mon", &self.write_ree2_tcm_mon())
            .field("tcm_mon_lock", &self.tcm_mon_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures tcm_mon registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_tcm_mon(&mut self) -> READ_TEE_TCM_MON_W<'_, TCM_MON_CTRL_SPEC> {
        READ_TEE_TCM_MON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures tcm_mon registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_tcm_mon(&mut self) -> READ_REE0_TCM_MON_W<'_, TCM_MON_CTRL_SPEC> {
        READ_REE0_TCM_MON_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures tcm_mon registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_tcm_mon(&mut self) -> READ_REE1_TCM_MON_W<'_, TCM_MON_CTRL_SPEC> {
        READ_REE1_TCM_MON_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures tcm_mon registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_tcm_mon(&mut self) -> READ_REE2_TCM_MON_W<'_, TCM_MON_CTRL_SPEC> {
        READ_REE2_TCM_MON_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures tcm_mon registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_tcm_mon(&mut self) -> WRITE_TEE_TCM_MON_W<'_, TCM_MON_CTRL_SPEC> {
        WRITE_TEE_TCM_MON_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures tcm_mon registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_tcm_mon(&mut self) -> WRITE_REE0_TCM_MON_W<'_, TCM_MON_CTRL_SPEC> {
        WRITE_REE0_TCM_MON_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures tcm_mon registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_tcm_mon(&mut self) -> WRITE_REE1_TCM_MON_W<'_, TCM_MON_CTRL_SPEC> {
        WRITE_REE1_TCM_MON_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures tcm_mon registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_tcm_mon(&mut self) -> WRITE_REE2_TCM_MON_W<'_, TCM_MON_CTRL_SPEC> {
        WRITE_REE2_TCM_MON_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set 1 to lock tcm_mon peri_apm configuration"]
    #[inline(always)]
    pub fn tcm_mon_lock(&mut self) -> TCM_MON_LOCK_W<'_, TCM_MON_CTRL_SPEC> {
        TCM_MON_LOCK_W::new(self, 8)
    }
}
#[doc = "tcm_mon read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_mon_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_mon_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_MON_CTRL_SPEC;
impl crate::RegisterSpec for TCM_MON_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_mon_ctrl::R`](R) reader structure"]
impl crate::Readable for TCM_MON_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcm_mon_ctrl::W`](W) writer structure"]
impl crate::Writable for TCM_MON_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_MON_CTRL to value 0x11"]
impl crate::Resettable for TCM_MON_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}

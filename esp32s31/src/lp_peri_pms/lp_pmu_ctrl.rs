#[doc = "Register `LP_PMU_CTRL` reader"]
pub type R = crate::R<LP_PMU_CTRL_SPEC>;
#[doc = "Register `LP_PMU_CTRL` writer"]
pub type W = crate::W<LP_PMU_CTRL_SPEC>;
#[doc = "Field `READ_TEE_LP_PMU` reader - Configures lp_pmu registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_LP_PMU_R = crate::BitReader;
#[doc = "Field `READ_TEE_LP_PMU` writer - Configures lp_pmu registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_LP_PMU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_LP_PMU` reader - Configures lp_pmu registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_LP_PMU_R = crate::BitReader;
#[doc = "Field `READ_REE0_LP_PMU` writer - Configures lp_pmu registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_LP_PMU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_LP_PMU` reader - Configures lp_pmu registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_LP_PMU_R = crate::BitReader;
#[doc = "Field `READ_REE1_LP_PMU` writer - Configures lp_pmu registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_LP_PMU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_LP_PMU` reader - Configures lp_pmu registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_LP_PMU_R = crate::BitReader;
#[doc = "Field `READ_REE2_LP_PMU` writer - Configures lp_pmu registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_LP_PMU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_LP_PMU` reader - Configures lp_pmu registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_LP_PMU_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_LP_PMU` writer - Configures lp_pmu registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_LP_PMU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_LP_PMU` reader - Configures lp_pmu registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_LP_PMU_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_LP_PMU` writer - Configures lp_pmu registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_LP_PMU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_LP_PMU` reader - Configures lp_pmu registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_LP_PMU_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_LP_PMU` writer - Configures lp_pmu registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_LP_PMU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_LP_PMU` reader - Configures lp_pmu registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_LP_PMU_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_LP_PMU` writer - Configures lp_pmu registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_LP_PMU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PMU_LOCK` reader - Set 1 to lock lp_pmu peri_apm configuration"]
pub type LP_PMU_LOCK_R = crate::BitReader;
#[doc = "Field `LP_PMU_LOCK` writer - Set 1 to lock lp_pmu peri_apm configuration"]
pub type LP_PMU_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures lp_pmu registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_lp_pmu(&self) -> READ_TEE_LP_PMU_R {
        READ_TEE_LP_PMU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures lp_pmu registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_lp_pmu(&self) -> READ_REE0_LP_PMU_R {
        READ_REE0_LP_PMU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures lp_pmu registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_lp_pmu(&self) -> READ_REE1_LP_PMU_R {
        READ_REE1_LP_PMU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures lp_pmu registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_lp_pmu(&self) -> READ_REE2_LP_PMU_R {
        READ_REE2_LP_PMU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures lp_pmu registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_lp_pmu(&self) -> WRITE_TEE_LP_PMU_R {
        WRITE_TEE_LP_PMU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures lp_pmu registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_lp_pmu(&self) -> WRITE_REE0_LP_PMU_R {
        WRITE_REE0_LP_PMU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures lp_pmu registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_lp_pmu(&self) -> WRITE_REE1_LP_PMU_R {
        WRITE_REE1_LP_PMU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures lp_pmu registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_lp_pmu(&self) -> WRITE_REE2_LP_PMU_R {
        WRITE_REE2_LP_PMU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to lock lp_pmu peri_apm configuration"]
    #[inline(always)]
    pub fn lp_pmu_lock(&self) -> LP_PMU_LOCK_R {
        LP_PMU_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PMU_CTRL")
            .field("read_tee_lp_pmu", &self.read_tee_lp_pmu())
            .field("read_ree0_lp_pmu", &self.read_ree0_lp_pmu())
            .field("read_ree1_lp_pmu", &self.read_ree1_lp_pmu())
            .field("read_ree2_lp_pmu", &self.read_ree2_lp_pmu())
            .field("write_tee_lp_pmu", &self.write_tee_lp_pmu())
            .field("write_ree0_lp_pmu", &self.write_ree0_lp_pmu())
            .field("write_ree1_lp_pmu", &self.write_ree1_lp_pmu())
            .field("write_ree2_lp_pmu", &self.write_ree2_lp_pmu())
            .field("lp_pmu_lock", &self.lp_pmu_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures lp_pmu registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_lp_pmu(&mut self) -> READ_TEE_LP_PMU_W<'_, LP_PMU_CTRL_SPEC> {
        READ_TEE_LP_PMU_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures lp_pmu registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_lp_pmu(&mut self) -> READ_REE0_LP_PMU_W<'_, LP_PMU_CTRL_SPEC> {
        READ_REE0_LP_PMU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures lp_pmu registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_lp_pmu(&mut self) -> READ_REE1_LP_PMU_W<'_, LP_PMU_CTRL_SPEC> {
        READ_REE1_LP_PMU_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures lp_pmu registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_lp_pmu(&mut self) -> READ_REE2_LP_PMU_W<'_, LP_PMU_CTRL_SPEC> {
        READ_REE2_LP_PMU_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures lp_pmu registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_lp_pmu(&mut self) -> WRITE_TEE_LP_PMU_W<'_, LP_PMU_CTRL_SPEC> {
        WRITE_TEE_LP_PMU_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures lp_pmu registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_lp_pmu(&mut self) -> WRITE_REE0_LP_PMU_W<'_, LP_PMU_CTRL_SPEC> {
        WRITE_REE0_LP_PMU_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures lp_pmu registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_lp_pmu(&mut self) -> WRITE_REE1_LP_PMU_W<'_, LP_PMU_CTRL_SPEC> {
        WRITE_REE1_LP_PMU_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures lp_pmu registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_lp_pmu(&mut self) -> WRITE_REE2_LP_PMU_W<'_, LP_PMU_CTRL_SPEC> {
        WRITE_REE2_LP_PMU_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set 1 to lock lp_pmu peri_apm configuration"]
    #[inline(always)]
    pub fn lp_pmu_lock(&mut self) -> LP_PMU_LOCK_W<'_, LP_PMU_CTRL_SPEC> {
        LP_PMU_LOCK_W::new(self, 8)
    }
}
#[doc = "lp_pmu read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pmu_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pmu_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_PMU_CTRL_SPEC;
impl crate::RegisterSpec for LP_PMU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_pmu_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_PMU_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_pmu_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_PMU_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_PMU_CTRL to value 0x11"]
impl crate::Resettable for LP_PMU_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}

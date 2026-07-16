#[doc = "Register `LP_INTR_CTRL` reader"]
pub type R = crate::R<LP_INTR_CTRL_SPEC>;
#[doc = "Register `LP_INTR_CTRL` writer"]
pub type W = crate::W<LP_INTR_CTRL_SPEC>;
#[doc = "Field `READ_TEE_LP_INTR` reader - Configures lp_intr registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_LP_INTR_R = crate::BitReader;
#[doc = "Field `READ_TEE_LP_INTR` writer - Configures lp_intr registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_LP_INTR` reader - Configures lp_intr registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_LP_INTR_R = crate::BitReader;
#[doc = "Field `READ_REE0_LP_INTR` writer - Configures lp_intr registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_LP_INTR` reader - Configures lp_intr registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_LP_INTR_R = crate::BitReader;
#[doc = "Field `READ_REE1_LP_INTR` writer - Configures lp_intr registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_LP_INTR` reader - Configures lp_intr registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_LP_INTR_R = crate::BitReader;
#[doc = "Field `READ_REE2_LP_INTR` writer - Configures lp_intr registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_LP_INTR` reader - Configures lp_intr registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_LP_INTR_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_LP_INTR` writer - Configures lp_intr registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_LP_INTR` reader - Configures lp_intr registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_LP_INTR_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_LP_INTR` writer - Configures lp_intr registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_LP_INTR` reader - Configures lp_intr registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_LP_INTR_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_LP_INTR` writer - Configures lp_intr registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_LP_INTR` reader - Configures lp_intr registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_LP_INTR_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_LP_INTR` writer - Configures lp_intr registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_INTR_LOCK` reader - Set 1 to lock lp_intr peri_apm configuration"]
pub type LP_INTR_LOCK_R = crate::BitReader;
#[doc = "Field `LP_INTR_LOCK` writer - Set 1 to lock lp_intr peri_apm configuration"]
pub type LP_INTR_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures lp_intr registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_lp_intr(&self) -> READ_TEE_LP_INTR_R {
        READ_TEE_LP_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures lp_intr registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_lp_intr(&self) -> READ_REE0_LP_INTR_R {
        READ_REE0_LP_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures lp_intr registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_lp_intr(&self) -> READ_REE1_LP_INTR_R {
        READ_REE1_LP_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures lp_intr registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_lp_intr(&self) -> READ_REE2_LP_INTR_R {
        READ_REE2_LP_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures lp_intr registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_lp_intr(&self) -> WRITE_TEE_LP_INTR_R {
        WRITE_TEE_LP_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures lp_intr registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_lp_intr(&self) -> WRITE_REE0_LP_INTR_R {
        WRITE_REE0_LP_INTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures lp_intr registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_lp_intr(&self) -> WRITE_REE1_LP_INTR_R {
        WRITE_REE1_LP_INTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures lp_intr registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_lp_intr(&self) -> WRITE_REE2_LP_INTR_R {
        WRITE_REE2_LP_INTR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to lock lp_intr peri_apm configuration"]
    #[inline(always)]
    pub fn lp_intr_lock(&self) -> LP_INTR_LOCK_R {
        LP_INTR_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INTR_CTRL")
            .field("read_tee_lp_intr", &self.read_tee_lp_intr())
            .field("read_ree0_lp_intr", &self.read_ree0_lp_intr())
            .field("read_ree1_lp_intr", &self.read_ree1_lp_intr())
            .field("read_ree2_lp_intr", &self.read_ree2_lp_intr())
            .field("write_tee_lp_intr", &self.write_tee_lp_intr())
            .field("write_ree0_lp_intr", &self.write_ree0_lp_intr())
            .field("write_ree1_lp_intr", &self.write_ree1_lp_intr())
            .field("write_ree2_lp_intr", &self.write_ree2_lp_intr())
            .field("lp_intr_lock", &self.lp_intr_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures lp_intr registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_lp_intr(&mut self) -> READ_TEE_LP_INTR_W<'_, LP_INTR_CTRL_SPEC> {
        READ_TEE_LP_INTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures lp_intr registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_lp_intr(&mut self) -> READ_REE0_LP_INTR_W<'_, LP_INTR_CTRL_SPEC> {
        READ_REE0_LP_INTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures lp_intr registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_lp_intr(&mut self) -> READ_REE1_LP_INTR_W<'_, LP_INTR_CTRL_SPEC> {
        READ_REE1_LP_INTR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures lp_intr registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_lp_intr(&mut self) -> READ_REE2_LP_INTR_W<'_, LP_INTR_CTRL_SPEC> {
        READ_REE2_LP_INTR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures lp_intr registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_lp_intr(&mut self) -> WRITE_TEE_LP_INTR_W<'_, LP_INTR_CTRL_SPEC> {
        WRITE_TEE_LP_INTR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures lp_intr registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_lp_intr(&mut self) -> WRITE_REE0_LP_INTR_W<'_, LP_INTR_CTRL_SPEC> {
        WRITE_REE0_LP_INTR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures lp_intr registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_lp_intr(&mut self) -> WRITE_REE1_LP_INTR_W<'_, LP_INTR_CTRL_SPEC> {
        WRITE_REE1_LP_INTR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures lp_intr registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_lp_intr(&mut self) -> WRITE_REE2_LP_INTR_W<'_, LP_INTR_CTRL_SPEC> {
        WRITE_REE2_LP_INTR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set 1 to lock lp_intr peri_apm configuration"]
    #[inline(always)]
    pub fn lp_intr_lock(&mut self) -> LP_INTR_LOCK_W<'_, LP_INTR_CTRL_SPEC> {
        LP_INTR_LOCK_W::new(self, 8)
    }
}
#[doc = "lp_intr read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_intr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_intr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INTR_CTRL_SPEC;
impl crate::RegisterSpec for LP_INTR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_intr_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_INTR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_intr_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_INTR_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_INTR_CTRL to value 0x11"]
impl crate::Resettable for LP_INTR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}

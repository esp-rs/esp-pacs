#[doc = "Register `LP_ANA_PERI_CTRL` reader"]
pub type R = crate::R<LP_ANA_PERI_CTRL_SPEC>;
#[doc = "Register `LP_ANA_PERI_CTRL` writer"]
pub type W = crate::W<LP_ANA_PERI_CTRL_SPEC>;
#[doc = "Field `READ_TEE_LP_ANA_PERI` reader - Configures lp_ana_peri registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_LP_ANA_PERI_R = crate::BitReader;
#[doc = "Field `READ_TEE_LP_ANA_PERI` writer - Configures lp_ana_peri registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_LP_ANA_PERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_LP_ANA_PERI` reader - Configures lp_ana_peri registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_LP_ANA_PERI_R = crate::BitReader;
#[doc = "Field `READ_REE0_LP_ANA_PERI` writer - Configures lp_ana_peri registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_LP_ANA_PERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_LP_ANA_PERI` reader - Configures lp_ana_peri registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_LP_ANA_PERI_R = crate::BitReader;
#[doc = "Field `READ_REE1_LP_ANA_PERI` writer - Configures lp_ana_peri registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_LP_ANA_PERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_LP_ANA_PERI` reader - Configures lp_ana_peri registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_LP_ANA_PERI_R = crate::BitReader;
#[doc = "Field `READ_REE2_LP_ANA_PERI` writer - Configures lp_ana_peri registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_LP_ANA_PERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_LP_ANA_PERI` reader - Configures lp_ana_peri registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_LP_ANA_PERI_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_LP_ANA_PERI` writer - Configures lp_ana_peri registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_LP_ANA_PERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_LP_ANA_PERI` reader - Configures lp_ana_peri registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_LP_ANA_PERI_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_LP_ANA_PERI` writer - Configures lp_ana_peri registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_LP_ANA_PERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_LP_ANA_PERI` reader - Configures lp_ana_peri registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_LP_ANA_PERI_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_LP_ANA_PERI` writer - Configures lp_ana_peri registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_LP_ANA_PERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_LP_ANA_PERI` reader - Configures lp_ana_peri registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_LP_ANA_PERI_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_LP_ANA_PERI` writer - Configures lp_ana_peri registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_LP_ANA_PERI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures lp_ana_peri registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_lp_ana_peri(&self) -> READ_TEE_LP_ANA_PERI_R {
        READ_TEE_LP_ANA_PERI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures lp_ana_peri registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_lp_ana_peri(&self) -> READ_REE0_LP_ANA_PERI_R {
        READ_REE0_LP_ANA_PERI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures lp_ana_peri registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_lp_ana_peri(&self) -> READ_REE1_LP_ANA_PERI_R {
        READ_REE1_LP_ANA_PERI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures lp_ana_peri registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_lp_ana_peri(&self) -> READ_REE2_LP_ANA_PERI_R {
        READ_REE2_LP_ANA_PERI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures lp_ana_peri registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_lp_ana_peri(&self) -> WRITE_TEE_LP_ANA_PERI_R {
        WRITE_TEE_LP_ANA_PERI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures lp_ana_peri registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_lp_ana_peri(&self) -> WRITE_REE0_LP_ANA_PERI_R {
        WRITE_REE0_LP_ANA_PERI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures lp_ana_peri registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_lp_ana_peri(&self) -> WRITE_REE1_LP_ANA_PERI_R {
        WRITE_REE1_LP_ANA_PERI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures lp_ana_peri registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_lp_ana_peri(&self) -> WRITE_REE2_LP_ANA_PERI_R {
        WRITE_REE2_LP_ANA_PERI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_PERI_CTRL")
            .field("read_tee_lp_ana_peri", &self.read_tee_lp_ana_peri())
            .field("read_ree0_lp_ana_peri", &self.read_ree0_lp_ana_peri())
            .field("read_ree1_lp_ana_peri", &self.read_ree1_lp_ana_peri())
            .field("read_ree2_lp_ana_peri", &self.read_ree2_lp_ana_peri())
            .field("write_tee_lp_ana_peri", &self.write_tee_lp_ana_peri())
            .field("write_ree0_lp_ana_peri", &self.write_ree0_lp_ana_peri())
            .field("write_ree1_lp_ana_peri", &self.write_ree1_lp_ana_peri())
            .field("write_ree2_lp_ana_peri", &self.write_ree2_lp_ana_peri())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures lp_ana_peri registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_lp_ana_peri(&mut self) -> READ_TEE_LP_ANA_PERI_W<'_, LP_ANA_PERI_CTRL_SPEC> {
        READ_TEE_LP_ANA_PERI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures lp_ana_peri registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_lp_ana_peri(&mut self) -> READ_REE0_LP_ANA_PERI_W<'_, LP_ANA_PERI_CTRL_SPEC> {
        READ_REE0_LP_ANA_PERI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures lp_ana_peri registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_lp_ana_peri(&mut self) -> READ_REE1_LP_ANA_PERI_W<'_, LP_ANA_PERI_CTRL_SPEC> {
        READ_REE1_LP_ANA_PERI_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures lp_ana_peri registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_lp_ana_peri(&mut self) -> READ_REE2_LP_ANA_PERI_W<'_, LP_ANA_PERI_CTRL_SPEC> {
        READ_REE2_LP_ANA_PERI_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures lp_ana_peri registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_lp_ana_peri(&mut self) -> WRITE_TEE_LP_ANA_PERI_W<'_, LP_ANA_PERI_CTRL_SPEC> {
        WRITE_TEE_LP_ANA_PERI_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures lp_ana_peri registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_lp_ana_peri(
        &mut self,
    ) -> WRITE_REE0_LP_ANA_PERI_W<'_, LP_ANA_PERI_CTRL_SPEC> {
        WRITE_REE0_LP_ANA_PERI_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures lp_ana_peri registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_lp_ana_peri(
        &mut self,
    ) -> WRITE_REE1_LP_ANA_PERI_W<'_, LP_ANA_PERI_CTRL_SPEC> {
        WRITE_REE1_LP_ANA_PERI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures lp_ana_peri registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_lp_ana_peri(
        &mut self,
    ) -> WRITE_REE2_LP_ANA_PERI_W<'_, LP_ANA_PERI_CTRL_SPEC> {
        WRITE_REE2_LP_ANA_PERI_W::new(self, 7)
    }
}
#[doc = "lp_ana_peri read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_peri_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_peri_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_PERI_CTRL_SPEC;
impl crate::RegisterSpec for LP_ANA_PERI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_peri_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_ANA_PERI_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_peri_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_ANA_PERI_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_ANA_PERI_CTRL to value 0x11"]
impl crate::Resettable for LP_ANA_PERI_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}

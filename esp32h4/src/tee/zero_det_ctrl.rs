#[doc = "Register `ZERO_DET_CTRL` reader"]
pub type R = crate::R<ZERO_DET_CTRL_SPEC>;
#[doc = "Register `ZERO_DET_CTRL` writer"]
pub type W = crate::W<ZERO_DET_CTRL_SPEC>;
#[doc = "Field `READ_TEE_ZERO_DET` reader - Configures zero_det registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_ZERO_DET_R = crate::BitReader;
#[doc = "Field `READ_TEE_ZERO_DET` writer - Configures zero_det registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_ZERO_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_ZERO_DET` reader - Configures zero_det registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_ZERO_DET_R = crate::BitReader;
#[doc = "Field `READ_REE0_ZERO_DET` writer - Configures zero_det registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_ZERO_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_ZERO_DET` reader - Configures zero_det registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_ZERO_DET_R = crate::BitReader;
#[doc = "Field `READ_REE1_ZERO_DET` writer - Configures zero_det registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_ZERO_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_ZERO_DET` reader - Configures zero_det registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_ZERO_DET_R = crate::BitReader;
#[doc = "Field `READ_REE2_ZERO_DET` writer - Configures zero_det registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_ZERO_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_ZERO_DET` reader - Configures zero_det registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_ZERO_DET_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_ZERO_DET` writer - Configures zero_det registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_ZERO_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_ZERO_DET` reader - Configures zero_det registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_ZERO_DET_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_ZERO_DET` writer - Configures zero_det registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_ZERO_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_ZERO_DET` reader - Configures zero_det registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_ZERO_DET_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_ZERO_DET` writer - Configures zero_det registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_ZERO_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_ZERO_DET` reader - Configures zero_det registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_ZERO_DET_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_ZERO_DET` writer - Configures zero_det registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_ZERO_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures zero_det registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_zero_det(&self) -> READ_TEE_ZERO_DET_R {
        READ_TEE_ZERO_DET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures zero_det registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_zero_det(&self) -> READ_REE0_ZERO_DET_R {
        READ_REE0_ZERO_DET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures zero_det registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_zero_det(&self) -> READ_REE1_ZERO_DET_R {
        READ_REE1_ZERO_DET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures zero_det registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_zero_det(&self) -> READ_REE2_ZERO_DET_R {
        READ_REE2_ZERO_DET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures zero_det registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_zero_det(&self) -> WRITE_TEE_ZERO_DET_R {
        WRITE_TEE_ZERO_DET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures zero_det registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_zero_det(&self) -> WRITE_REE0_ZERO_DET_R {
        WRITE_REE0_ZERO_DET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures zero_det registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_zero_det(&self) -> WRITE_REE1_ZERO_DET_R {
        WRITE_REE1_ZERO_DET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures zero_det registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_zero_det(&self) -> WRITE_REE2_ZERO_DET_R {
        WRITE_REE2_ZERO_DET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZERO_DET_CTRL")
            .field("read_tee_zero_det", &self.read_tee_zero_det())
            .field("read_ree0_zero_det", &self.read_ree0_zero_det())
            .field("read_ree1_zero_det", &self.read_ree1_zero_det())
            .field("read_ree2_zero_det", &self.read_ree2_zero_det())
            .field("write_tee_zero_det", &self.write_tee_zero_det())
            .field("write_ree0_zero_det", &self.write_ree0_zero_det())
            .field("write_ree1_zero_det", &self.write_ree1_zero_det())
            .field("write_ree2_zero_det", &self.write_ree2_zero_det())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures zero_det registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_zero_det(&mut self) -> READ_TEE_ZERO_DET_W<'_, ZERO_DET_CTRL_SPEC> {
        READ_TEE_ZERO_DET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures zero_det registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_zero_det(&mut self) -> READ_REE0_ZERO_DET_W<'_, ZERO_DET_CTRL_SPEC> {
        READ_REE0_ZERO_DET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures zero_det registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_zero_det(&mut self) -> READ_REE1_ZERO_DET_W<'_, ZERO_DET_CTRL_SPEC> {
        READ_REE1_ZERO_DET_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures zero_det registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_zero_det(&mut self) -> READ_REE2_ZERO_DET_W<'_, ZERO_DET_CTRL_SPEC> {
        READ_REE2_ZERO_DET_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures zero_det registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_zero_det(&mut self) -> WRITE_TEE_ZERO_DET_W<'_, ZERO_DET_CTRL_SPEC> {
        WRITE_TEE_ZERO_DET_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures zero_det registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_zero_det(&mut self) -> WRITE_REE0_ZERO_DET_W<'_, ZERO_DET_CTRL_SPEC> {
        WRITE_REE0_ZERO_DET_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures zero_det registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_zero_det(&mut self) -> WRITE_REE1_ZERO_DET_W<'_, ZERO_DET_CTRL_SPEC> {
        WRITE_REE1_ZERO_DET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures zero_det registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_zero_det(&mut self) -> WRITE_REE2_ZERO_DET_W<'_, ZERO_DET_CTRL_SPEC> {
        WRITE_REE2_ZERO_DET_W::new(self, 7)
    }
}
#[doc = "zero_det read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_det_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zero_det_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZERO_DET_CTRL_SPEC;
impl crate::RegisterSpec for ZERO_DET_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zero_det_ctrl::R`](R) reader structure"]
impl crate::Readable for ZERO_DET_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`zero_det_ctrl::W`](W) writer structure"]
impl crate::Writable for ZERO_DET_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ZERO_DET_CTRL to value 0x11"]
impl crate::Resettable for ZERO_DET_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}

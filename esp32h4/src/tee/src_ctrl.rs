#[doc = "Register `SRC_CTRL` reader"]
pub type R = crate::R<SRC_CTRL_SPEC>;
#[doc = "Register `SRC_CTRL` writer"]
pub type W = crate::W<SRC_CTRL_SPEC>;
#[doc = "Field `READ_TEE_SRC` reader - Configures src registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_SRC_R = crate::BitReader;
#[doc = "Field `READ_TEE_SRC` writer - Configures src registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_SRC` reader - Configures src registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_SRC_R = crate::BitReader;
#[doc = "Field `READ_REE0_SRC` writer - Configures src registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_SRC` reader - Configures src registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_SRC_R = crate::BitReader;
#[doc = "Field `READ_REE1_SRC` writer - Configures src registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_SRC` reader - Configures src registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_SRC_R = crate::BitReader;
#[doc = "Field `READ_REE2_SRC` writer - Configures src registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_SRC` reader - Configures src registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_SRC_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_SRC` writer - Configures src registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_SRC` reader - Configures src registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_SRC_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_SRC` writer - Configures src registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_SRC` reader - Configures src registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_SRC_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_SRC` writer - Configures src registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_SRC` reader - Configures src registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_SRC_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_SRC` writer - Configures src registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures src registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_src(&self) -> READ_TEE_SRC_R {
        READ_TEE_SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures src registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_src(&self) -> READ_REE0_SRC_R {
        READ_REE0_SRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures src registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_src(&self) -> READ_REE1_SRC_R {
        READ_REE1_SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures src registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_src(&self) -> READ_REE2_SRC_R {
        READ_REE2_SRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures src registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_src(&self) -> WRITE_TEE_SRC_R {
        WRITE_TEE_SRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures src registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_src(&self) -> WRITE_REE0_SRC_R {
        WRITE_REE0_SRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures src registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_src(&self) -> WRITE_REE1_SRC_R {
        WRITE_REE1_SRC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures src registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_src(&self) -> WRITE_REE2_SRC_R {
        WRITE_REE2_SRC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRC_CTRL")
            .field("read_tee_src", &self.read_tee_src())
            .field("read_ree0_src", &self.read_ree0_src())
            .field("read_ree1_src", &self.read_ree1_src())
            .field("read_ree2_src", &self.read_ree2_src())
            .field("write_tee_src", &self.write_tee_src())
            .field("write_ree0_src", &self.write_ree0_src())
            .field("write_ree1_src", &self.write_ree1_src())
            .field("write_ree2_src", &self.write_ree2_src())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures src registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_src(&mut self) -> READ_TEE_SRC_W<'_, SRC_CTRL_SPEC> {
        READ_TEE_SRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures src registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_src(&mut self) -> READ_REE0_SRC_W<'_, SRC_CTRL_SPEC> {
        READ_REE0_SRC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures src registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_src(&mut self) -> READ_REE1_SRC_W<'_, SRC_CTRL_SPEC> {
        READ_REE1_SRC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures src registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_src(&mut self) -> READ_REE2_SRC_W<'_, SRC_CTRL_SPEC> {
        READ_REE2_SRC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures src registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_src(&mut self) -> WRITE_TEE_SRC_W<'_, SRC_CTRL_SPEC> {
        WRITE_TEE_SRC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures src registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_src(&mut self) -> WRITE_REE0_SRC_W<'_, SRC_CTRL_SPEC> {
        WRITE_REE0_SRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures src registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_src(&mut self) -> WRITE_REE1_SRC_W<'_, SRC_CTRL_SPEC> {
        WRITE_REE1_SRC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures src registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_src(&mut self) -> WRITE_REE2_SRC_W<'_, SRC_CTRL_SPEC> {
        WRITE_REE2_SRC_W::new(self, 7)
    }
}
#[doc = "src read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`src_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRC_CTRL_SPEC;
impl crate::RegisterSpec for SRC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_ctrl::R`](R) reader structure"]
impl crate::Readable for SRC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`src_ctrl::W`](W) writer structure"]
impl crate::Writable for SRC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_CTRL to value 0x11"]
impl crate::Resettable for SRC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}

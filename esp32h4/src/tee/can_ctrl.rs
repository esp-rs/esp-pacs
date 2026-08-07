#[doc = "Register `CAN_CTRL` reader"]
pub type R = crate::R<CAN_CTRL_SPEC>;
#[doc = "Register `CAN_CTRL` writer"]
pub type W = crate::W<CAN_CTRL_SPEC>;
#[doc = "Field `READ_TEE_CAN` reader - Configures can registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_CAN_R = crate::BitReader;
#[doc = "Field `READ_TEE_CAN` writer - Configures can registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_CAN` reader - Configures can registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_CAN_R = crate::BitReader;
#[doc = "Field `READ_REE0_CAN` writer - Configures can registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_CAN` reader - Configures can registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_CAN_R = crate::BitReader;
#[doc = "Field `READ_REE1_CAN` writer - Configures can registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_CAN` reader - Configures can registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_CAN_R = crate::BitReader;
#[doc = "Field `READ_REE2_CAN` writer - Configures can registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_CAN` reader - Configures can registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_CAN_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_CAN` writer - Configures can registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_CAN` reader - Configures can registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_CAN_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_CAN` writer - Configures can registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_CAN` reader - Configures can registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_CAN_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_CAN` writer - Configures can registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_CAN` reader - Configures can registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_CAN_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_CAN` writer - Configures can registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures can registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_can(&self) -> READ_TEE_CAN_R {
        READ_TEE_CAN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures can registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_can(&self) -> READ_REE0_CAN_R {
        READ_REE0_CAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures can registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_can(&self) -> READ_REE1_CAN_R {
        READ_REE1_CAN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures can registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_can(&self) -> READ_REE2_CAN_R {
        READ_REE2_CAN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures can registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_can(&self) -> WRITE_TEE_CAN_R {
        WRITE_TEE_CAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures can registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_can(&self) -> WRITE_REE0_CAN_R {
        WRITE_REE0_CAN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures can registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_can(&self) -> WRITE_REE1_CAN_R {
        WRITE_REE1_CAN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures can registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_can(&self) -> WRITE_REE2_CAN_R {
        WRITE_REE2_CAN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_CTRL")
            .field("read_tee_can", &self.read_tee_can())
            .field("read_ree0_can", &self.read_ree0_can())
            .field("read_ree1_can", &self.read_ree1_can())
            .field("read_ree2_can", &self.read_ree2_can())
            .field("write_tee_can", &self.write_tee_can())
            .field("write_ree0_can", &self.write_ree0_can())
            .field("write_ree1_can", &self.write_ree1_can())
            .field("write_ree2_can", &self.write_ree2_can())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures can registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_can(&mut self) -> READ_TEE_CAN_W<'_, CAN_CTRL_SPEC> {
        READ_TEE_CAN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures can registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_can(&mut self) -> READ_REE0_CAN_W<'_, CAN_CTRL_SPEC> {
        READ_REE0_CAN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures can registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_can(&mut self) -> READ_REE1_CAN_W<'_, CAN_CTRL_SPEC> {
        READ_REE1_CAN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures can registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_can(&mut self) -> READ_REE2_CAN_W<'_, CAN_CTRL_SPEC> {
        READ_REE2_CAN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures can registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_can(&mut self) -> WRITE_TEE_CAN_W<'_, CAN_CTRL_SPEC> {
        WRITE_TEE_CAN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures can registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_can(&mut self) -> WRITE_REE0_CAN_W<'_, CAN_CTRL_SPEC> {
        WRITE_REE0_CAN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures can registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_can(&mut self) -> WRITE_REE1_CAN_W<'_, CAN_CTRL_SPEC> {
        WRITE_REE1_CAN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures can registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_can(&mut self) -> WRITE_REE2_CAN_W<'_, CAN_CTRL_SPEC> {
        WRITE_REE2_CAN_W::new(self, 7)
    }
}
#[doc = "can read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_CTRL_SPEC;
impl crate::RegisterSpec for CAN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ctrl::R`](R) reader structure"]
impl crate::Readable for CAN_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_ctrl::W`](W) writer structure"]
impl crate::Writable for CAN_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_CTRL to value 0x11"]
impl crate::Resettable for CAN_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}

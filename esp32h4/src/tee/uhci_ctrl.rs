#[doc = "Register `UHCI_CTRL` reader"]
pub type R = crate::R<UHCI_CTRL_SPEC>;
#[doc = "Register `UHCI_CTRL` writer"]
pub type W = crate::W<UHCI_CTRL_SPEC>;
#[doc = "Field `READ_TEE_UHCI` reader - Configures uhci registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_UHCI_R = crate::BitReader;
#[doc = "Field `READ_TEE_UHCI` writer - Configures uhci registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_TEE_UHCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE0_UHCI` reader - Configures uhci registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_UHCI_R = crate::BitReader;
#[doc = "Field `READ_REE0_UHCI` writer - Configures uhci registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE0_UHCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE1_UHCI` reader - Configures uhci registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_UHCI_R = crate::BitReader;
#[doc = "Field `READ_REE1_UHCI` writer - Configures uhci registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE1_UHCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REE2_UHCI` reader - Configures uhci registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_UHCI_R = crate::BitReader;
#[doc = "Field `READ_REE2_UHCI` writer - Configures uhci registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
pub type READ_REE2_UHCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TEE_UHCI` reader - Configures uhci registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_UHCI_R = crate::BitReader;
#[doc = "Field `WRITE_TEE_UHCI` writer - Configures uhci registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_TEE_UHCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE0_UHCI` reader - Configures uhci registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_UHCI_R = crate::BitReader;
#[doc = "Field `WRITE_REE0_UHCI` writer - Configures uhci registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE0_UHCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE1_UHCI` reader - Configures uhci registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_UHCI_R = crate::BitReader;
#[doc = "Field `WRITE_REE1_UHCI` writer - Configures uhci registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE1_UHCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_REE2_UHCI` reader - Configures uhci registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_UHCI_R = crate::BitReader;
#[doc = "Field `WRITE_REE2_UHCI` writer - Configures uhci registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
pub type WRITE_REE2_UHCI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures uhci registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_uhci(&self) -> READ_TEE_UHCI_R {
        READ_TEE_UHCI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures uhci registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_uhci(&self) -> READ_REE0_UHCI_R {
        READ_REE0_UHCI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures uhci registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_uhci(&self) -> READ_REE1_UHCI_R {
        READ_REE1_UHCI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures uhci registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_uhci(&self) -> READ_REE2_UHCI_R {
        READ_REE2_UHCI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures uhci registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_uhci(&self) -> WRITE_TEE_UHCI_R {
        WRITE_TEE_UHCI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures uhci registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_uhci(&self) -> WRITE_REE0_UHCI_R {
        WRITE_REE0_UHCI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures uhci registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_uhci(&self) -> WRITE_REE1_UHCI_R {
        WRITE_REE1_UHCI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures uhci registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_uhci(&self) -> WRITE_REE2_UHCI_R {
        WRITE_REE2_UHCI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHCI_CTRL")
            .field("read_tee_uhci", &self.read_tee_uhci())
            .field("read_ree0_uhci", &self.read_ree0_uhci())
            .field("read_ree1_uhci", &self.read_ree1_uhci())
            .field("read_ree2_uhci", &self.read_ree2_uhci())
            .field("write_tee_uhci", &self.write_tee_uhci())
            .field("write_ree0_uhci", &self.write_ree0_uhci())
            .field("write_ree1_uhci", &self.write_ree1_uhci())
            .field("write_ree2_uhci", &self.write_ree2_uhci())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures uhci registers read permission in tee mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_tee_uhci(&mut self) -> READ_TEE_UHCI_W<'_, UHCI_CTRL_SPEC> {
        READ_TEE_UHCI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures uhci registers read permission in ree0 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree0_uhci(&mut self) -> READ_REE0_UHCI_W<'_, UHCI_CTRL_SPEC> {
        READ_REE0_UHCI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures uhci registers read permission in ree1 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree1_uhci(&mut self) -> READ_REE1_UHCI_W<'_, UHCI_CTRL_SPEC> {
        READ_REE1_UHCI_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures uhci registers read permission in ree2 mode.\\\\ 0: can not be read \\\\ 1: can be read \\\\"]
    #[inline(always)]
    pub fn read_ree2_uhci(&mut self) -> READ_REE2_UHCI_W<'_, UHCI_CTRL_SPEC> {
        READ_REE2_UHCI_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures uhci registers write permission in tee mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_tee_uhci(&mut self) -> WRITE_TEE_UHCI_W<'_, UHCI_CTRL_SPEC> {
        WRITE_TEE_UHCI_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures uhci registers write permission in ree0 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree0_uhci(&mut self) -> WRITE_REE0_UHCI_W<'_, UHCI_CTRL_SPEC> {
        WRITE_REE0_UHCI_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures uhci registers write permission in ree1 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree1_uhci(&mut self) -> WRITE_REE1_UHCI_W<'_, UHCI_CTRL_SPEC> {
        WRITE_REE1_UHCI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures uhci registers write permission in ree2 mode.\\\\ 0: can not be write \\\\ 1: can be write \\\\"]
    #[inline(always)]
    pub fn write_ree2_uhci(&mut self) -> WRITE_REE2_UHCI_W<'_, UHCI_CTRL_SPEC> {
        WRITE_REE2_UHCI_W::new(self, 7)
    }
}
#[doc = "uhci read/write control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHCI_CTRL_SPEC;
impl crate::RegisterSpec for UHCI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci_ctrl::R`](R) reader structure"]
impl crate::Readable for UHCI_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhci_ctrl::W`](W) writer structure"]
impl crate::Writable for UHCI_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCI_CTRL to value 0x11"]
impl crate::Resettable for UHCI_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x11;
}

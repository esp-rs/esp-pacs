#[doc = "Register `COMD4` reader"]
pub type R = crate::R<COMD4_SPEC>;
#[doc = "Register `COMD4` writer"]
pub type W = crate::W<COMD4_SPEC>;
#[doc = "Field `COMMAND4` reader - Configures command 4. See details in I2C_CMD0_REG\\[13:0\\]."]
pub type COMMAND4_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND4` writer - Configures command 4. See details in I2C_CMD0_REG\\[13:0\\]."]
pub type COMMAND4_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND4_DONE` reader - Represents whether command 4 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
pub type COMMAND4_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND4_DONE` writer - Represents whether command 4 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
pub type COMMAND4_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Configures command 4. See details in I2C_CMD0_REG\\[13:0\\]."]
    #[inline(always)]
    pub fn command4(&self) -> COMMAND4_R {
        COMMAND4_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Represents whether command 4 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
    #[inline(always)]
    pub fn command4_done(&self) -> COMMAND4_DONE_R {
        COMMAND4_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD4")
            .field("command4", &self.command4())
            .field("command4_done", &self.command4_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Configures command 4. See details in I2C_CMD0_REG\\[13:0\\]."]
    #[inline(always)]
    pub fn command4(&mut self) -> COMMAND4_W<COMD4_SPEC> {
        COMMAND4_W::new(self, 0)
    }
    #[doc = "Bit 31 - Represents whether command 4 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
    #[inline(always)]
    pub fn command4_done(&mut self) -> COMMAND4_DONE_W<COMD4_SPEC> {
        COMMAND4_DONE_W::new(self, 31)
    }
}
#[doc = "I2C command register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`comd4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comd4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD4_SPEC;
impl crate::RegisterSpec for COMD4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd4::R`](R) reader structure"]
impl crate::Readable for COMD4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd4::W`](W) writer structure"]
impl crate::Writable for COMD4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMD4 to value 0"]
impl crate::Resettable for COMD4_SPEC {}

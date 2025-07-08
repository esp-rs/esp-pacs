#[doc = "Register `COMD7` reader"]
pub type R = crate::R<COMD7_SPEC>;
#[doc = "Register `COMD7` writer"]
pub type W = crate::W<COMD7_SPEC>;
#[doc = "Field `COMMAND7` reader - Configures command 7. See details in I2C_CMD0_REG\\[13:0\\]."]
pub type COMMAND7_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND7` writer - Configures command 7. See details in I2C_CMD0_REG\\[13:0\\]."]
pub type COMMAND7_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND7_DONE` reader - Represents whether command 7 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
pub type COMMAND7_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND7_DONE` writer - Represents whether command 7 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
pub type COMMAND7_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Configures command 7. See details in I2C_CMD0_REG\\[13:0\\]."]
    #[inline(always)]
    pub fn command7(&self) -> COMMAND7_R {
        COMMAND7_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Represents whether command 7 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
    #[inline(always)]
    pub fn command7_done(&self) -> COMMAND7_DONE_R {
        COMMAND7_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD7")
            .field("command7", &self.command7())
            .field("command7_done", &self.command7_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Configures command 7. See details in I2C_CMD0_REG\\[13:0\\]."]
    #[inline(always)]
    pub fn command7(&mut self) -> COMMAND7_W<COMD7_SPEC> {
        COMMAND7_W::new(self, 0)
    }
    #[doc = "Bit 31 - Represents whether command 7 is done in I2C Master mode.\\\\ 0: Not done \\\\ 1: Done \\\\"]
    #[inline(always)]
    pub fn command7_done(&mut self) -> COMMAND7_DONE_W<COMD7_SPEC> {
        COMMAND7_DONE_W::new(self, 31)
    }
}
#[doc = "I2C command register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`comd7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comd7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD7_SPEC;
impl crate::RegisterSpec for COMD7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd7::R`](R) reader structure"]
impl crate::Readable for COMD7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd7::W`](W) writer structure"]
impl crate::Writable for COMD7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMD7 to value 0"]
impl crate::Resettable for COMD7_SPEC {}

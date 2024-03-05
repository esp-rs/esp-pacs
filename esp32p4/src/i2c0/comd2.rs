#[doc = "Register `COMD2` reader"]
pub type R = crate::R<COMD2_SPEC>;
#[doc = "Register `COMD2` writer"]
pub type W = crate::W<COMD2_SPEC>;
#[doc = "Field `COMMAND2` reader - Configures command 2. See details in I2C_CMD0_REG\\[13:0\\]."]
pub type COMMAND2_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND2` writer - Configures command 2. See details in I2C_CMD0_REG\\[13:0\\]."]
pub type COMMAND2_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND2_DONE` reader - Represents whether command 2 is done in I2C Master mode. 0: Not done 1: Done"]
pub type COMMAND2_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND2_DONE` writer - Represents whether command 2 is done in I2C Master mode. 0: Not done 1: Done"]
pub type COMMAND2_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Configures command 2. See details in I2C_CMD0_REG\\[13:0\\]."]
    #[inline(always)]
    pub fn command2(&self) -> COMMAND2_R {
        COMMAND2_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Represents whether command 2 is done in I2C Master mode. 0: Not done 1: Done"]
    #[inline(always)]
    pub fn command2_done(&self) -> COMMAND2_DONE_R {
        COMMAND2_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD2")
            .field("command2", &format_args!("{}", self.command2().bits()))
            .field(
                "command2_done",
                &format_args!("{}", self.command2_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMD2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Configures command 2. See details in I2C_CMD0_REG\\[13:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn command2(&mut self) -> COMMAND2_W<COMD2_SPEC> {
        COMMAND2_W::new(self, 0)
    }
    #[doc = "Bit 31 - Represents whether command 2 is done in I2C Master mode. 0: Not done 1: Done"]
    #[inline(always)]
    #[must_use]
    pub fn command2_done(&mut self) -> COMMAND2_DONE_W<COMD2_SPEC> {
        COMMAND2_DONE_W::new(self, 31)
    }
}
#[doc = "I2C command register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comd2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comd2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD2_SPEC;
impl crate::RegisterSpec for COMD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd2::R`](R) reader structure"]
impl crate::Readable for COMD2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd2::W`](W) writer structure"]
impl crate::Writable for COMD2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMD2 to value 0"]
impl crate::Resettable for COMD2_SPEC {
    const RESET_VALUE: u32 = 0;
}

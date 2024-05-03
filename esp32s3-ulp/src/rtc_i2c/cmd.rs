#[doc = "Register `CMD%s` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD%s` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `COMMAND` reader - command0"]
pub type COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND` writer - command0"]
pub type COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND_DONE` reader - command0_done"]
pub type COMMAND_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command0"]
    #[inline(always)]
    pub fn command(&self) -> COMMAND_R {
        COMMAND_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command0_done"]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("command", &self.command().bits())
            .field("command_done", &self.command_done().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command0"]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> COMMAND_W<CMD_SPEC> {
        COMMAND_W::new(self, 0)
    }
}
#[doc = "I2C command%s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD%s to value 0x0903"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0x0903;
}

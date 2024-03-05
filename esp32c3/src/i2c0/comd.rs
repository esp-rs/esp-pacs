#[doc = "Register `COMD%s` reader"]
pub type R = crate::R<COMD_SPEC>;
#[doc = "Register `COMD%s` writer"]
pub type W = crate::W<COMD_SPEC>;
#[doc = "Field `COMMAND` reader - reg_command"]
pub type COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND` writer - reg_command"]
pub type COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND_DONE` reader - reg_command_done"]
pub type COMMAND_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND_DONE` writer - reg_command_done"]
pub type COMMAND_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - reg_command"]
    #[inline(always)]
    pub fn command(&self) -> COMMAND_R {
        COMMAND_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - reg_command_done"]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD")
            .field("command", &format_args!("{}", self.command().bits()))
            .field(
                "command_done",
                &format_args!("{}", self.command_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - reg_command"]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> COMMAND_W<COMD_SPEC> {
        COMMAND_W::new(self, 0)
    }
    #[doc = "Bit 31 - reg_command_done"]
    #[inline(always)]
    #[must_use]
    pub fn command_done(&mut self) -> COMMAND_DONE_W<COMD_SPEC> {
        COMMAND_DONE_W::new(self, 31)
    }
}
#[doc = "I2C_COMD%s_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMD_SPEC;
impl crate::RegisterSpec for COMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comd::R`](R) reader structure"]
impl crate::Readable for COMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comd::W`](W) writer structure"]
impl crate::Writable for COMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMD%s to value 0"]
impl crate::Resettable for COMD_SPEC {
    const RESET_VALUE: u32 = 0;
}

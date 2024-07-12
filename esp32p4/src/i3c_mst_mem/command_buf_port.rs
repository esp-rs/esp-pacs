#[doc = "Register `COMMAND_BUF_PORT` reader"]
pub type R = crate::R<COMMAND_BUF_PORT_SPEC>;
#[doc = "Register `COMMAND_BUF_PORT` writer"]
pub type W = crate::W<COMMAND_BUF_PORT_SPEC>;
#[doc = "Field `REG_COMMAND` reader - Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus."]
pub type REG_COMMAND_R = crate::FieldReader<u32>;
#[doc = "Field `REG_COMMAND` writer - Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus."]
pub type REG_COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus."]
    #[inline(always)]
    pub fn reg_command(&self) -> REG_COMMAND_R {
        REG_COMMAND_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMAND_BUF_PORT")
            .field("reg_command", &self.reg_command())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains a Command Descriptor structure that depends on the requested transfer type. Command Descriptor structure is used to schedule the transfers to devices on I3C bus."]
    #[inline(always)]
    #[must_use]
    pub fn reg_command(&mut self) -> REG_COMMAND_W<COMMAND_BUF_PORT_SPEC> {
        REG_COMMAND_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`command_buf_port::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command_buf_port::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMMAND_BUF_PORT_SPEC;
impl crate::RegisterSpec for COMMAND_BUF_PORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command_buf_port::R`](R) reader structure"]
impl crate::Readable for COMMAND_BUF_PORT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`command_buf_port::W`](W) writer structure"]
impl crate::Writable for COMMAND_BUF_PORT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMMAND_BUF_PORT to value 0"]
impl crate::Resettable for COMMAND_BUF_PORT_SPEC {
    const RESET_VALUE: u32 = 0;
}

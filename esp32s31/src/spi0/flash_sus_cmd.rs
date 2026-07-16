#[doc = "Register `FLASH_SUS_CMD` reader"]
pub type R = crate::R<FLASH_SUS_CMD_SPEC>;
#[doc = "Register `FLASH_SUS_CMD` writer"]
pub type W = crate::W<FLASH_SUS_CMD_SPEC>;
#[doc = "Field `FLASH_PES_COMMAND` reader - "]
pub type FLASH_PES_COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_PES_COMMAND` writer - "]
pub type FLASH_PES_COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WAIT_PESR_COMMAND` reader - "]
pub type WAIT_PESR_COMMAND_R = crate::FieldReader<u16>;
#[doc = "Field `WAIT_PESR_COMMAND` writer - "]
pub type WAIT_PESR_COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn flash_pes_command(&self) -> FLASH_PES_COMMAND_R {
        FLASH_PES_COMMAND_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wait_pesr_command(&self) -> WAIT_PESR_COMMAND_R {
        WAIT_PESR_COMMAND_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_SUS_CMD")
            .field("flash_pes_command", &self.flash_pes_command())
            .field("wait_pesr_command", &self.wait_pesr_command())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn flash_pes_command(&mut self) -> FLASH_PES_COMMAND_W<'_, FLASH_SUS_CMD_SPEC> {
        FLASH_PES_COMMAND_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wait_pesr_command(&mut self) -> WAIT_PESR_COMMAND_W<'_, FLASH_SUS_CMD_SPEC> {
        WAIT_PESR_COMMAND_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SUS_CMD_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_sus_cmd::R`](R) reader structure"]
impl crate::Readable for FLASH_SUS_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_sus_cmd::W`](W) writer structure"]
impl crate::Writable for FLASH_SUS_CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_SUS_CMD to value 0"]
impl crate::Resettable for FLASH_SUS_CMD_SPEC {}

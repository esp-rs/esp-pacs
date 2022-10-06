#[doc = "Register `FLASH_SUS_CMD` reader"]
pub struct R(crate::R<FLASH_SUS_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SUS_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SUS_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SUS_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_SUS_CMD` writer"]
pub struct W(crate::W<FLASH_SUS_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SUS_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FLASH_SUS_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SUS_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_PER_COMMAND` reader - Program/Erase resume command."]
pub type FLASH_PER_COMMAND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_PER_COMMAND` writer - Program/Erase resume command."]
pub type FLASH_PER_COMMAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SUS_CMD_SPEC, u8, u8, 8, O>;
#[doc = "Field `FLASH_PES_COMMAND` reader - Program/Erase suspend command."]
pub type FLASH_PES_COMMAND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_PES_COMMAND` writer - Program/Erase suspend command."]
pub type FLASH_PES_COMMAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SUS_CMD_SPEC, u8, u8, 8, O>;
#[doc = "Field `WAIT_PESR_COMMAND` reader - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
pub type WAIT_PESR_COMMAND_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WAIT_PESR_COMMAND` writer - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
pub type WAIT_PESR_COMMAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SUS_CMD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - Program/Erase resume command."]
    #[inline(always)]
    pub fn flash_per_command(&self) -> FLASH_PER_COMMAND_R {
        FLASH_PER_COMMAND_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Program/Erase suspend command."]
    #[inline(always)]
    pub fn flash_pes_command(&self) -> FLASH_PES_COMMAND_R {
        FLASH_PES_COMMAND_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
    #[inline(always)]
    pub fn wait_pesr_command(&self) -> WAIT_PESR_COMMAND_R {
        WAIT_PESR_COMMAND_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Program/Erase resume command."]
    #[inline(always)]
    pub fn flash_per_command(&mut self) -> FLASH_PER_COMMAND_W<0> {
        FLASH_PER_COMMAND_W::new(self)
    }
    #[doc = "Bits 8:15 - Program/Erase suspend command."]
    #[inline(always)]
    pub fn flash_pes_command(&mut self) -> FLASH_PES_COMMAND_W<8> {
        FLASH_PES_COMMAND_W::new(self)
    }
    #[doc = "Bits 16:31 - Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
    #[inline(always)]
    pub fn wait_pesr_command(&mut self) -> WAIT_PESR_COMMAND_W<16> {
        WAIT_PESR_COMMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 flash suspend command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_sus_cmd](index.html) module"]
pub struct FLASH_SUS_CMD_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_sus_cmd::R](R) reader structure"]
impl crate::Readable for FLASH_SUS_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_sus_cmd::W](W) writer structure"]
impl crate::Writable for FLASH_SUS_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_SUS_CMD to value 0x0005_757a"]
impl crate::Resettable for FLASH_SUS_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_757a
    }
}

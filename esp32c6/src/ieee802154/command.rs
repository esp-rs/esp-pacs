#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<COMMAND_SPEC>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<COMMAND_SPEC>;
#[doc = "Field `OPCODE` reader - "]
pub type OPCODE_R = crate::FieldReader;
#[doc = "Field `OPCODE` writer - "]
pub type OPCODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMAND")
            .field("opcode", &self.opcode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OPCODE_W<COMMAND_SPEC> {
        OPCODE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMMAND_SPEC;
impl crate::RegisterSpec for COMMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for COMMAND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for COMMAND_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for COMMAND_SPEC {
    const RESET_VALUE: u32 = 0;
}

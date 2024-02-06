#[doc = "Register `CMD2` reader"]
pub type R = crate::R<CMD2_SPEC>;
#[doc = "Register `CMD2` writer"]
pub type W = crate::W<CMD2_SPEC>;
#[doc = "Field `COMMAND2` reader - Content of command 2. For more information, please refer to the register I2C_COMD2_REG in Chapter I²C Controller."]
pub type COMMAND2_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND2` writer - Content of command 2. For more information, please refer to the register I2C_COMD2_REG in Chapter I²C Controller."]
pub type COMMAND2_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND2_DONE` reader - When command 2 is done, this bit changes to 1."]
pub type COMMAND2_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - Content of command 2. For more information, please refer to the register I2C_COMD2_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command2(&self) -> COMMAND2_R {
        COMMAND2_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 2 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command2_done(&self) -> COMMAND2_DONE_R {
        COMMAND2_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD2")
            .field("command2", &format_args!("{}", self.command2().bits()))
            .field(
                "command2_done",
                &format_args!("{}", self.command2_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 2. For more information, please refer to the register I2C_COMD2_REG in Chapter I²C Controller."]
    #[inline(always)]
    #[must_use]
    pub fn command2(&mut self) -> COMMAND2_W<CMD2_SPEC> {
        COMMAND2_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC I2C Command 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD2_SPEC;
impl crate::RegisterSpec for CMD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd2::R`](R) reader structure"]
impl crate::Readable for CMD2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd2::W`](W) writer structure"]
impl crate::Writable for CMD2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD2 to value 0x0902"]
impl crate::Resettable for CMD2_SPEC {
    const RESET_VALUE: u32 = 0x0902;
}

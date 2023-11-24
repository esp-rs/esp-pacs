#[doc = "Register `CMD4` reader"]
pub type R = crate::R<CMD4_SPEC>;
#[doc = "Register `CMD4` writer"]
pub type W = crate::W<CMD4_SPEC>;
#[doc = "Field `COMMAND4` reader - Content of command 4. For more information, please refer to the register I2C_COMD4_REG in Chapter I²C Controller."]
pub type COMMAND4_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND4` writer - Content of command 4. For more information, please refer to the register I2C_COMD4_REG in Chapter I²C Controller."]
pub type COMMAND4_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND4_DONE` reader - When command 4 is done, this bit changes to 1."]
pub type COMMAND4_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - Content of command 4. For more information, please refer to the register I2C_COMD4_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command4(&self) -> COMMAND4_R {
        COMMAND4_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 4 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command4_done(&self) -> COMMAND4_DONE_R {
        COMMAND4_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD4")
            .field("command4", &format_args!("{}", self.command4().bits()))
            .field(
                "command4_done",
                &format_args!("{}", self.command4_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 4. For more information, please refer to the register I2C_COMD4_REG in Chapter I²C Controller."]
    #[inline(always)]
    #[must_use]
    pub fn command4(&mut self) -> COMMAND4_W<CMD4_SPEC> {
        COMMAND4_W::new(self, 0)
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
#[doc = "RTC I2C Command 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD4_SPEC;
impl crate::RegisterSpec for CMD4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd4::R`](R) reader structure"]
impl crate::Readable for CMD4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd4::W`](W) writer structure"]
impl crate::Writable for CMD4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD4 to value 0x0901"]
impl crate::Resettable for CMD4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0901;
}

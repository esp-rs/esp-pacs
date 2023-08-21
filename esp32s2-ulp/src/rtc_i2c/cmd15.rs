#[doc = "Register `CMD15` reader"]
pub type R = crate::R<CMD15_SPEC>;
#[doc = "Register `CMD15` writer"]
pub type W = crate::W<CMD15_SPEC>;
#[doc = "Field `COMMAND15` reader - Content of command 15. For more information, please refer to the register I2C_COMD15_REG in Chapter I²C Controller."]
pub type COMMAND15_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND15` writer - Content of command 15. For more information, please refer to the register I2C_COMD15_REG in Chapter I²C Controller."]
pub type COMMAND15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `COMMAND15_DONE` reader - When command 15 is done, this bit changes to 1."]
pub type COMMAND15_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - Content of command 15. For more information, please refer to the register I2C_COMD15_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command15(&self) -> COMMAND15_R {
        COMMAND15_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 15 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command15_done(&self) -> COMMAND15_DONE_R {
        COMMAND15_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD15")
            .field("command15", &format_args!("{}", self.command15().bits()))
            .field(
                "command15_done",
                &format_args!("{}", self.command15_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD15_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 15. For more information, please refer to the register I2C_COMD15_REG in Chapter I²C Controller."]
    #[inline(always)]
    #[must_use]
    pub fn command15(&mut self) -> COMMAND15_W<CMD15_SPEC, 0> {
        COMMAND15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC I2C Command 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD15_SPEC;
impl crate::RegisterSpec for CMD15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd15::R`](R) reader structure"]
impl crate::Readable for CMD15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd15::W`](W) writer structure"]
impl crate::Writable for CMD15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD15 to value 0"]
impl crate::Resettable for CMD15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

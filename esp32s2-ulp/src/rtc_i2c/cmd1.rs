#[doc = "Register `CMD1` reader"]
pub type R = crate::R<CMD1_SPEC>;
#[doc = "Register `CMD1` writer"]
pub type W = crate::W<CMD1_SPEC>;
#[doc = "Field `COMMAND1` reader - Content of command 1. For more information, please refer to the register I2C_COMD1_REG in Chapter I²C Controller."]
pub type COMMAND1_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND1` writer - Content of command 1. For more information, please refer to the register I2C_COMD1_REG in Chapter I²C Controller."]
pub type COMMAND1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `COMMAND1_DONE` reader - When command 1 is done, this bit changes to 1."]
pub type COMMAND1_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - Content of command 1. For more information, please refer to the register I2C_COMD1_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command1(&self) -> COMMAND1_R {
        COMMAND1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 1 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command1_done(&self) -> COMMAND1_DONE_R {
        COMMAND1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD1")
            .field("command1", &format_args!("{}", self.command1().bits()))
            .field(
                "command1_done",
                &format_args!("{}", self.command1_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 1. For more information, please refer to the register I2C_COMD1_REG in Chapter I²C Controller."]
    #[inline(always)]
    #[must_use]
    pub fn command1(&mut self) -> COMMAND1_W<CMD1_SPEC, 0> {
        COMMAND1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC I2C Command 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD1_SPEC;
impl crate::RegisterSpec for CMD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd1::R`](R) reader structure"]
impl crate::Readable for CMD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd1::W`](W) writer structure"]
impl crate::Writable for CMD1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD1 to value 0x1901"]
impl crate::Resettable for CMD1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1901;
}

#[doc = "Register `CMD6` reader"]
pub type R = crate::R<CMD6_SPEC>;
#[doc = "Register `CMD6` writer"]
pub type W = crate::W<CMD6_SPEC>;
#[doc = "Field `COMMAND6` reader - Content of command 6. For more information, please refer to the register I2C_COMD6_REG in Chapter I²C Controller."]
pub type COMMAND6_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND6` writer - Content of command 6. For more information, please refer to the register I2C_COMD6_REG in Chapter I²C Controller."]
pub type COMMAND6_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND6_DONE` reader - When command 6 is done, this bit changes to 1."]
pub type COMMAND6_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - Content of command 6. For more information, please refer to the register I2C_COMD6_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command6(&self) -> COMMAND6_R {
        COMMAND6_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 6 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command6_done(&self) -> COMMAND6_DONE_R {
        COMMAND6_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD6")
            .field("command6", &format_args!("{}", self.command6().bits()))
            .field(
                "command6_done",
                &format_args!("{}", self.command6_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 6. For more information, please refer to the register I2C_COMD6_REG in Chapter I²C Controller."]
    #[inline(always)]
    #[must_use]
    pub fn command6(&mut self) -> COMMAND6_W<CMD6_SPEC> {
        COMMAND6_W::new(self, 0)
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
#[doc = "RTC I2C Command 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD6_SPEC;
impl crate::RegisterSpec for CMD6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd6::R`](R) reader structure"]
impl crate::Readable for CMD6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd6::W`](W) writer structure"]
impl crate::Writable for CMD6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD6 to value 0x1901"]
impl crate::Resettable for CMD6_SPEC {
    const RESET_VALUE: u32 = 0x1901;
}

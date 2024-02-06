#[doc = "Register `CMD0` reader"]
pub type R = crate::R<CMD0_SPEC>;
#[doc = "Register `CMD0` writer"]
pub type W = crate::W<CMD0_SPEC>;
#[doc = "Field `COMMAND0` reader - Content of command 0. For more information, please refer to the register I2C_COMD0_REG in Chapter I²C Controller"]
pub type COMMAND0_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND0` writer - Content of command 0. For more information, please refer to the register I2C_COMD0_REG in Chapter I²C Controller"]
pub type COMMAND0_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND0_DONE` reader - When command 0 is done, this bit changes to 1."]
pub type COMMAND0_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - Content of command 0. For more information, please refer to the register I2C_COMD0_REG in Chapter I²C Controller"]
    #[inline(always)]
    pub fn command0(&self) -> COMMAND0_R {
        COMMAND0_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 0 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command0_done(&self) -> COMMAND0_DONE_R {
        COMMAND0_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD0")
            .field("command0", &format_args!("{}", self.command0().bits()))
            .field(
                "command0_done",
                &format_args!("{}", self.command0_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 0. For more information, please refer to the register I2C_COMD0_REG in Chapter I²C Controller"]
    #[inline(always)]
    #[must_use]
    pub fn command0(&mut self) -> COMMAND0_W<CMD0_SPEC> {
        COMMAND0_W::new(self, 0)
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
#[doc = "RTC I2C Command 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD0_SPEC;
impl crate::RegisterSpec for CMD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd0::R`](R) reader structure"]
impl crate::Readable for CMD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd0::W`](W) writer structure"]
impl crate::Writable for CMD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD0 to value 0x0903"]
impl crate::Resettable for CMD0_SPEC {
    const RESET_VALUE: u32 = 0x0903;
}

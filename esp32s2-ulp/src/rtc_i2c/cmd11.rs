#[doc = "Register `CMD11` reader"]
pub type R = crate::R<CMD11_SPEC>;
#[doc = "Register `CMD11` writer"]
pub type W = crate::W<CMD11_SPEC>;
#[doc = "Field `COMMAND11` reader - Content of command 11. For more information, please refer to the register I2C_COMD11_REG in Chapter I²C Controller."]
pub type COMMAND11_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND11` writer - Content of command 11. For more information, please refer to the register I2C_COMD11_REG in Chapter I²C Controller."]
pub type COMMAND11_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND11_DONE` reader - When command 11 is done, this bit changes to 1."]
pub type COMMAND11_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - Content of command 11. For more information, please refer to the register I2C_COMD11_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command11(&self) -> COMMAND11_R {
        COMMAND11_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 11 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command11_done(&self) -> COMMAND11_DONE_R {
        COMMAND11_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD11")
            .field("command11", &format_args!("{}", self.command11().bits()))
            .field(
                "command11_done",
                &format_args!("{}", self.command11_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 11. For more information, please refer to the register I2C_COMD11_REG in Chapter I²C Controller."]
    #[inline(always)]
    #[must_use]
    pub fn command11(&mut self) -> COMMAND11_W<CMD11_SPEC> {
        COMMAND11_W::new(self, 0)
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
#[doc = "RTC I2C Command 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD11_SPEC;
impl crate::RegisterSpec for CMD11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd11::R`](R) reader structure"]
impl crate::Readable for CMD11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd11::W`](W) writer structure"]
impl crate::Writable for CMD11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD11 to value 0x0901"]
impl crate::Resettable for CMD11_SPEC {
    const RESET_VALUE: u32 = 0x0901;
}

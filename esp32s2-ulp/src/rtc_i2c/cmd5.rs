#[doc = "Register `CMD5` reader"]
pub type R = crate::R<CMD5_SPEC>;
#[doc = "Register `CMD5` writer"]
pub type W = crate::W<CMD5_SPEC>;
#[doc = "Field `COMMAND5` reader - Content of command 5. For more information, please refer to the register I2C_COMD5_REG in Chapter I²C Controller."]
pub type COMMAND5_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND5` writer - Content of command 5. For more information, please refer to the register I2C_COMD5_REG in Chapter I²C Controller."]
pub type COMMAND5_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND5_DONE` reader - When command 5 is done, this bit changes to 1."]
pub type COMMAND5_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - Content of command 5. For more information, please refer to the register I2C_COMD5_REG in Chapter I²C Controller."]
    #[inline(always)]
    pub fn command5(&self) -> COMMAND5_R {
        COMMAND5_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 5 is done, this bit changes to 1."]
    #[inline(always)]
    pub fn command5_done(&self) -> COMMAND5_DONE_R {
        COMMAND5_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD5")
            .field("command5", &format_args!("{}", self.command5().bits()))
            .field(
                "command5_done",
                &format_args!("{}", self.command5_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Content of command 5. For more information, please refer to the register I2C_COMD5_REG in Chapter I²C Controller."]
    #[inline(always)]
    #[must_use]
    pub fn command5(&mut self) -> COMMAND5_W<CMD5_SPEC> {
        COMMAND5_W::new(self, 0)
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
#[doc = "RTC I2C Command 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD5_SPEC;
impl crate::RegisterSpec for CMD5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd5::R`](R) reader structure"]
impl crate::Readable for CMD5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd5::W`](W) writer structure"]
impl crate::Writable for CMD5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD5 to value 0x1701"]
impl crate::Resettable for CMD5_SPEC {
    const RESET_VALUE: Self::Ux = 0x1701;
}

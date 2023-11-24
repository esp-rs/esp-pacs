#[doc = "Register `CMD10` reader"]
pub type R = crate::R<CMD10_SPEC>;
#[doc = "Register `CMD10` writer"]
pub type W = crate::W<CMD10_SPEC>;
#[doc = "Field `COMMAND10` reader - command10"]
pub type COMMAND10_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND10` writer - command10"]
pub type COMMAND10_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND10_DONE` reader - command10_done"]
pub type COMMAND10_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command10"]
    #[inline(always)]
    pub fn command10(&self) -> COMMAND10_R {
        COMMAND10_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command10_done"]
    #[inline(always)]
    pub fn command10_done(&self) -> COMMAND10_DONE_R {
        COMMAND10_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD10")
            .field("command10", &format_args!("{}", self.command10().bits()))
            .field(
                "command10_done",
                &format_args!("{}", self.command10_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command10"]
    #[inline(always)]
    #[must_use]
    pub fn command10(&mut self) -> COMMAND10_W<CMD10_SPEC> {
        COMMAND10_W::new(self, 0)
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
#[doc = "i2c commond10 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD10_SPEC;
impl crate::RegisterSpec for CMD10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd10::R`](R) reader structure"]
impl crate::Readable for CMD10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd10::W`](W) writer structure"]
impl crate::Writable for CMD10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD10 to value 0x0101"]
impl crate::Resettable for CMD10_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}

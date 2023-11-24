#[doc = "Register `CMD12` reader"]
pub type R = crate::R<CMD12_SPEC>;
#[doc = "Register `CMD12` writer"]
pub type W = crate::W<CMD12_SPEC>;
#[doc = "Field `COMMAND12` reader - command12"]
pub type COMMAND12_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND12` writer - command12"]
pub type COMMAND12_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND12_DONE` reader - command12_done"]
pub type COMMAND12_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command12"]
    #[inline(always)]
    pub fn command12(&self) -> COMMAND12_R {
        COMMAND12_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command12_done"]
    #[inline(always)]
    pub fn command12_done(&self) -> COMMAND12_DONE_R {
        COMMAND12_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD12")
            .field("command12", &format_args!("{}", self.command12().bits()))
            .field(
                "command12_done",
                &format_args!("{}", self.command12_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command12"]
    #[inline(always)]
    #[must_use]
    pub fn command12(&mut self) -> COMMAND12_W<CMD12_SPEC> {
        COMMAND12_W::new(self, 0)
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
#[doc = "i2c commond12 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD12_SPEC;
impl crate::RegisterSpec for CMD12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd12::R`](R) reader structure"]
impl crate::Readable for CMD12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd12::W`](W) writer structure"]
impl crate::Writable for CMD12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD12 to value 0x1701"]
impl crate::Resettable for CMD12_SPEC {
    const RESET_VALUE: Self::Ux = 0x1701;
}

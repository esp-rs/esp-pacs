#[doc = "Register `CMD8` reader"]
pub type R = crate::R<CMD8_SPEC>;
#[doc = "Register `CMD8` writer"]
pub type W = crate::W<CMD8_SPEC>;
#[doc = "Field `COMMAND8` reader - command8"]
pub type COMMAND8_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND8` writer - command8"]
pub type COMMAND8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `COMMAND8_DONE` reader - command8_done"]
pub type COMMAND8_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command8"]
    #[inline(always)]
    pub fn command8(&self) -> COMMAND8_R {
        COMMAND8_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command8_done"]
    #[inline(always)]
    pub fn command8_done(&self) -> COMMAND8_DONE_R {
        COMMAND8_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD8")
            .field("command8", &format_args!("{}", self.command8().bits()))
            .field(
                "command8_done",
                &format_args!("{}", self.command8_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command8"]
    #[inline(always)]
    #[must_use]
    pub fn command8(&mut self) -> COMMAND8_W<CMD8_SPEC, 0> {
        COMMAND8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "i2c commond8 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD8_SPEC;
impl crate::RegisterSpec for CMD8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd8::R`](R) reader structure"]
impl crate::Readable for CMD8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd8::W`](W) writer structure"]
impl crate::Writable for CMD8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD8 to value 0x1901"]
impl crate::Resettable for CMD8_SPEC {
    const RESET_VALUE: Self::Ux = 0x1901;
}

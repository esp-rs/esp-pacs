#[doc = "Register `CMD13` reader"]
pub type R = crate::R<CMD13_SPEC>;
#[doc = "Register `CMD13` writer"]
pub type W = crate::W<CMD13_SPEC>;
#[doc = "Field `COMMAND13` reader - command13"]
pub type COMMAND13_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND13` writer - command13"]
pub type COMMAND13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `COMMAND13_DONE` reader - command13_done"]
pub type COMMAND13_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command13"]
    #[inline(always)]
    pub fn command13(&self) -> COMMAND13_R {
        COMMAND13_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command13_done"]
    #[inline(always)]
    pub fn command13_done(&self) -> COMMAND13_DONE_R {
        COMMAND13_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD13")
            .field("command13", &format_args!("{}", self.command13().bits()))
            .field(
                "command13_done",
                &format_args!("{}", self.command13_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command13"]
    #[inline(always)]
    #[must_use]
    pub fn command13(&mut self) -> COMMAND13_W<CMD13_SPEC, 0> {
        COMMAND13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "i2c commond13 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD13_SPEC;
impl crate::RegisterSpec for CMD13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd13::R`](R) reader structure"]
impl crate::Readable for CMD13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd13::W`](W) writer structure"]
impl crate::Writable for CMD13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD13 to value 0x1901"]
impl crate::Resettable for CMD13_SPEC {
    const RESET_VALUE: Self::Ux = 0x1901;
}

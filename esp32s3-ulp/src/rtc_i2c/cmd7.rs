#[doc = "Register `CMD7` reader"]
pub type R = crate::R<CMD7_SPEC>;
#[doc = "Register `CMD7` writer"]
pub type W = crate::W<CMD7_SPEC>;
#[doc = "Field `COMMAND7` reader - command7"]
pub type COMMAND7_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND7` writer - command7"]
pub type COMMAND7_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND7_DONE` reader - command7_done"]
pub type COMMAND7_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command7"]
    #[inline(always)]
    pub fn command7(&self) -> COMMAND7_R {
        COMMAND7_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command7_done"]
    #[inline(always)]
    pub fn command7_done(&self) -> COMMAND7_DONE_R {
        COMMAND7_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD7")
            .field("command7", &format_args!("{}", self.command7().bits()))
            .field(
                "command7_done",
                &format_args!("{}", self.command7_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command7"]
    #[inline(always)]
    #[must_use]
    pub fn command7(&mut self) -> COMMAND7_W<CMD7_SPEC> {
        COMMAND7_W::new(self, 0)
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
#[doc = "i2c commond7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD7_SPEC;
impl crate::RegisterSpec for CMD7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd7::R`](R) reader structure"]
impl crate::Readable for CMD7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd7::W`](W) writer structure"]
impl crate::Writable for CMD7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD7 to value 0x0904"]
impl crate::Resettable for CMD7_SPEC {
    const RESET_VALUE: u32 = 0x0904;
}

#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `VAL` reader - Command content"]
pub type VAL_R = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Command content"]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DONE` reader - Bit is set by HW when command is done"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - Bit is set by HW when command is done"]
pub type DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Command content"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Bit is set by HW when command is done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("val", &self.val())
            .field("done", &self.done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Command content"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<CMD_SPEC> {
        VAL_W::new(self, 0)
    }
    #[doc = "Bit 31 - Bit is set by HW when command is done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<CMD_SPEC> {
        DONE_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}

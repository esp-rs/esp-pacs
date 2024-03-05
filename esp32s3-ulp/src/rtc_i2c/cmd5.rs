#[doc = "Register `CMD5` reader"]
pub type R = crate::R<CMD5_SPEC>;
#[doc = "Register `CMD5` writer"]
pub type W = crate::W<CMD5_SPEC>;
#[doc = "Field `COMMAND5` reader - command5"]
pub type COMMAND5_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND5` writer - command5"]
pub type COMMAND5_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND5_DONE` reader - command5_done"]
pub type COMMAND5_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command5"]
    #[inline(always)]
    pub fn command5(&self) -> COMMAND5_R {
        COMMAND5_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command5_done"]
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
    #[doc = "Bits 0:13 - command5"]
    #[inline(always)]
    #[must_use]
    pub fn command5(&mut self) -> COMMAND5_W<CMD5_SPEC> {
        COMMAND5_W::new(self, 0)
    }
}
#[doc = "i2c commond5_register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD5_SPEC;
impl crate::RegisterSpec for CMD5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd5::R`](R) reader structure"]
impl crate::Readable for CMD5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd5::W`](W) writer structure"]
impl crate::Writable for CMD5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD5 to value 0x1701"]
impl crate::Resettable for CMD5_SPEC {
    const RESET_VALUE: u32 = 0x1701;
}

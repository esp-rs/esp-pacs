#[doc = "Register `BISTIN_SEL` reader"]
pub type R = crate::R<BISTIN_SEL_SPEC>;
#[doc = "Register `BISTIN_SEL` writer"]
pub type W = crate::W<BISTIN_SEL_SPEC>;
#[doc = "Field `BISTIN_SEL` reader - High speed sdio pad bist in pad sel 0:pad39, 1: pad40..."]
pub type BISTIN_SEL_R = crate::FieldReader;
#[doc = "Field `BISTIN_SEL` writer - High speed sdio pad bist in pad sel 0:pad39, 1: pad40..."]
pub type BISTIN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - High speed sdio pad bist in pad sel 0:pad39, 1: pad40..."]
    #[inline(always)]
    pub fn bistin_sel(&self) -> BISTIN_SEL_R {
        BISTIN_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BISTIN_SEL")
            .field("bistin_sel", &self.bistin_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - High speed sdio pad bist in pad sel 0:pad39, 1: pad40..."]
    #[inline(always)]
    pub fn bistin_sel(&mut self) -> BISTIN_SEL_W<'_, BISTIN_SEL_SPEC> {
        BISTIN_SEL_W::new(self, 0)
    }
}
#[doc = "High speed sdio pad bist in pad sel\n\nYou can [`read`](crate::Reg::read) this register and get [`bistin_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bistin_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BISTIN_SEL_SPEC;
impl crate::RegisterSpec for BISTIN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bistin_sel::R`](R) reader structure"]
impl crate::Readable for BISTIN_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bistin_sel::W`](W) writer structure"]
impl crate::Writable for BISTIN_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BISTIN_SEL to value 0x0f"]
impl crate::Resettable for BISTIN_SEL_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}

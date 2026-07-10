#[doc = "Register `ENTRY%s` reader"]
pub type R = crate::R<ENTRY_SPEC>;
#[doc = "Register `ENTRY%s` writer"]
pub type W = crate::W<ENTRY_SPEC>;
#[doc = "Field `PADDR` reader - Physical page number"]
pub type PADDR_R = crate::FieldReader;
#[doc = "Field `PADDR` writer - Physical page number"]
pub type PADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INVALID` reader - Set to mark this entry invalid"]
pub type INVALID_R = crate::BitReader;
#[doc = "Field `INVALID` writer - Set to mark this entry invalid"]
pub type INVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Physical page number"]
    #[inline(always)]
    pub fn paddr(&self) -> PADDR_R {
        PADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Set to mark this entry invalid"]
    #[inline(always)]
    pub fn invalid(&self) -> INVALID_R {
        INVALID_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENTRY")
            .field("paddr", &self.paddr())
            .field("invalid", &self.invalid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Physical page number"]
    #[inline(always)]
    pub fn paddr(&mut self) -> PADDR_W<'_, ENTRY_SPEC> {
        PADDR_W::new(self, 0)
    }
    #[doc = "Bit 8 - Set to mark this entry invalid"]
    #[inline(always)]
    pub fn invalid(&mut self) -> INVALID_W<'_, ENTRY_SPEC> {
        INVALID_W::new(self, 8)
    }
}
#[doc = "Flash MMU table entry\n\nYou can [`read`](crate::Reg::read) this register and get [`entry::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`entry::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENTRY_SPEC;
impl crate::RegisterSpec for ENTRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry::R`](R) reader structure"]
impl crate::Readable for ENTRY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`entry::W`](W) writer structure"]
impl crate::Writable for ENTRY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENTRY%s to value 0"]
impl crate::Resettable for ENTRY_SPEC {}

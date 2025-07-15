#[doc = "Register `ENTRY_CHECK` reader"]
pub type R = crate::R<ENTRY_CHECK_SPEC>;
#[doc = "Register `ENTRY_CHECK` writer"]
pub type W = crate::W<ENTRY_CHECK_SPEC>;
#[doc = "Field `ENTRY_CHECK` reader - This filed is used to enable entry address check"]
pub type ENTRY_CHECK_R = crate::FieldReader<u16>;
#[doc = "Field `ENTRY_CHECK` writer - This filed is used to enable entry address check"]
pub type ENTRY_CHECK_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 1:13 - This filed is used to enable entry address check"]
    #[inline(always)]
    pub fn entry_check(&self) -> ENTRY_CHECK_R {
        ENTRY_CHECK_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENTRY_CHECK")
            .field("entry_check", &self.entry_check())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:13 - This filed is used to enable entry address check"]
    #[inline(always)]
    pub fn entry_check(&mut self) -> ENTRY_CHECK_W<ENTRY_CHECK_SPEC> {
        ENTRY_CHECK_W::new(self, 1)
    }
}
#[doc = "Core_0 Entry check configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`entry_check::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`entry_check::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENTRY_CHECK_SPEC;
impl crate::RegisterSpec for ENTRY_CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry_check::R`](R) reader structure"]
impl crate::Readable for ENTRY_CHECK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`entry_check::W`](W) writer structure"]
impl crate::Writable for ENTRY_CHECK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENTRY_CHECK to value 0x02"]
impl crate::Resettable for ENTRY_CHECK_SPEC {
    const RESET_VALUE: u32 = 0x02;
}

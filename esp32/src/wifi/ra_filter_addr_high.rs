#[doc = "Register `RA_FILTER_ADDR_HIGH` reader"]
pub type R = crate::R<RA_FILTER_ADDR_HIGH_SPEC>;
#[doc = "Register `RA_FILTER_ADDR_HIGH` writer"]
pub type W = crate::W<RA_FILTER_ADDR_HIGH_SPEC>;
#[doc = "Field `ADDR` reader - "]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - "]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RA_FILTER_ADDR_HIGH")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<RA_FILTER_ADDR_HIGH_SPEC> {
        ADDR_W::new(self, 0)
    }
}
#[doc = "last 2 bytes of RA MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`ra_filter_addr_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra_filter_addr_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RA_FILTER_ADDR_HIGH_SPEC;
impl crate::RegisterSpec for RA_FILTER_ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ra_filter_addr_high::R`](R) reader structure"]
impl crate::Readable for RA_FILTER_ADDR_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ra_filter_addr_high::W`](W) writer structure"]
impl crate::Writable for RA_FILTER_ADDR_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RA_FILTER_ADDR_HIGH to value 0"]
impl crate::Resettable for RA_FILTER_ADDR_HIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}

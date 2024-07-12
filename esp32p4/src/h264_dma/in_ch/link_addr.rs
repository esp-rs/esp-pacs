#[doc = "Register `LINK_ADDR` reader"]
pub type R = crate::R<LINK_ADDR_SPEC>;
#[doc = "Register `LINK_ADDR` writer"]
pub type W = crate::W<LINK_ADDR_SPEC>;
#[doc = "Field `INLINK_ADDR` reader - This register stores the first inlink descriptor's address."]
pub type INLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR` writer - This register stores the first inlink descriptor's address."]
pub type INLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr(&self) -> INLINK_ADDR_R {
        INLINK_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LINK_ADDR")
            .field("inlink_addr", &self.inlink_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the first inlink descriptor's address."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_addr(&mut self) -> INLINK_ADDR_W<LINK_ADDR_SPEC> {
        INLINK_ADDR_W::new(self, 0)
    }
}
#[doc = "RX CHx in_link dscr addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`link_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINK_ADDR_SPEC;
impl crate::RegisterSpec for LINK_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_addr::R`](R) reader structure"]
impl crate::Readable for LINK_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`link_addr::W`](W) writer structure"]
impl crate::Writable for LINK_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINK_ADDR to value 0"]
impl crate::Resettable for LINK_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}

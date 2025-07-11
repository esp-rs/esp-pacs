#[doc = "Register `IN_LINK2` reader"]
pub type R = crate::R<IN_LINK2_SPEC>;
#[doc = "Register `IN_LINK2` writer"]
pub type W = crate::W<IN_LINK2_SPEC>;
#[doc = "Field `INLINK_ADDR` reader - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type INLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR` writer - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type INLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr(&self) -> INLINK_ADDR_R {
        INLINK_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK2")
            .field("inlink_addr", &self.inlink_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr(&mut self) -> INLINK_ADDR_W<IN_LINK2_SPEC> {
        INLINK_ADDR_W::new(self, 0)
    }
}
#[doc = "Link descriptor configure and control register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_LINK2_SPEC;
impl crate::RegisterSpec for IN_LINK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link2::R`](R) reader structure"]
impl crate::Readable for IN_LINK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_link2::W`](W) writer structure"]
impl crate::Writable for IN_LINK2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_LINK2 to value 0"]
impl crate::Resettable for IN_LINK2_SPEC {}

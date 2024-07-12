#[doc = "Register `OUT_LINK2` reader"]
pub type R = crate::R<OUT_LINK2_SPEC>;
#[doc = "Register `OUT_LINK2` writer"]
pub type W = crate::W<OUT_LINK2_SPEC>;
#[doc = "Field `OUTLINK_ADDR` reader - This register stores the 32 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR` writer - This register stores the 32 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the 32 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK2")
            .field("outlink_addr", &self.outlink_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the 32 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W<OUT_LINK2_SPEC> {
        OUTLINK_ADDR_W::new(self, 0)
    }
}
#[doc = "Link descriptor configure and control register of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK2_SPEC;
impl crate::RegisterSpec for OUT_LINK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link2::R`](R) reader structure"]
impl crate::Readable for OUT_LINK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link2::W`](W) writer structure"]
impl crate::Writable for OUT_LINK2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_LINK2 to value 0"]
impl crate::Resettable for OUT_LINK2_SPEC {
    const RESET_VALUE: u32 = 0;
}

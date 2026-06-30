#[doc = "Register `REGISTER2_HASHTABLEHIGHREGISTER` reader"]
pub type R = crate::R<REGISTER2_HASHTABLEHIGHREGISTER_SPEC>;
#[doc = "Register `REGISTER2_HASHTABLEHIGHREGISTER` writer"]
pub type W = crate::W<REGISTER2_HASHTABLEHIGHREGISTER_SPEC>;
#[doc = "Field `HTH` reader - Hash Table High This field contains the upper 32 bits of the Hash table"]
pub type HTH_R = crate::FieldReader<u32>;
#[doc = "Field `HTH` writer - Hash Table High This field contains the upper 32 bits of the Hash table"]
pub type HTH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Table High This field contains the upper 32 bits of the Hash table"]
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER2_HASHTABLEHIGHREGISTER")
            .field("hth", &self.hth())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table High This field contains the upper 32 bits of the Hash table"]
    #[inline(always)]
    pub fn hth(&mut self) -> HTH_W<'_, REGISTER2_HASHTABLEHIGHREGISTER_SPEC> {
        HTH_W::new(self, 0)
    }
}
#[doc = "Contains the higher 32 bits of the Multicast Hash table This register is present only when you select the 64bit Hash filter function in coreConsultant _See Table 79_\n\nYou can [`read`](crate::Reg::read) this register and get [`register2_hashtablehighregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register2_hashtablehighregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER2_HASHTABLEHIGHREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER2_HASHTABLEHIGHREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register2_hashtablehighregister::R`](R) reader structure"]
impl crate::Readable for REGISTER2_HASHTABLEHIGHREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register2_hashtablehighregister::W`](W) writer structure"]
impl crate::Writable for REGISTER2_HASHTABLEHIGHREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER2_HASHTABLEHIGHREGISTER to value 0"]
impl crate::Resettable for REGISTER2_HASHTABLEHIGHREGISTER_SPEC {}

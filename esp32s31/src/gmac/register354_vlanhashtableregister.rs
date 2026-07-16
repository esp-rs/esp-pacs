#[doc = "Register `REGISTER354_VLANHASHTABLEREGISTER` reader"]
pub type R = crate::R<REGISTER354_VLANHASHTABLEREGISTER_SPEC>;
#[doc = "Register `REGISTER354_VLANHASHTABLEREGISTER` writer"]
pub type W = crate::W<REGISTER354_VLANHASHTABLEREGISTER_SPEC>;
#[doc = "Field `VLHT` reader - VLAN Hash Table This field contains the 16bit VLAN Hash Table"]
pub type VLHT_R = crate::FieldReader<u16>;
#[doc = "Field `VLHT` writer - VLAN Hash Table This field contains the 16bit VLAN Hash Table"]
pub type VLHT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VLAN Hash Table This field contains the 16bit VLAN Hash Table"]
    #[inline(always)]
    pub fn vlht(&self) -> VLHT_R {
        VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER354_VLANHASHTABLEREGISTER")
            .field("vlht", &self.vlht())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Hash Table This field contains the 16bit VLAN Hash Table"]
    #[inline(always)]
    pub fn vlht(&mut self) -> VLHT_W<'_, REGISTER354_VLANHASHTABLEREGISTER_SPEC> {
        VLHT_W::new(self, 0)
    }
}
#[doc = "This register contains the VLAN hash table\n\nYou can [`read`](crate::Reg::read) this register and get [`register354_vlanhashtableregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register354_vlanhashtableregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER354_VLANHASHTABLEREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER354_VLANHASHTABLEREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register354_vlanhashtableregister::R`](R) reader structure"]
impl crate::Readable for REGISTER354_VLANHASHTABLEREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register354_vlanhashtableregister::W`](W) writer structure"]
impl crate::Writable for REGISTER354_VLANHASHTABLEREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER354_VLANHASHTABLEREGISTER to value 0"]
impl crate::Resettable for REGISTER354_VLANHASHTABLEREGISTER_SPEC {}

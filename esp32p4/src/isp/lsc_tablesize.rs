#[doc = "Register `LSC_TABLESIZE` reader"]
pub type R = crate::R<LSC_TABLESIZE_SPEC>;
#[doc = "Register `LSC_TABLESIZE` writer"]
pub type W = crate::W<LSC_TABLESIZE_SPEC>;
#[doc = "Field `LSC_XTABLESIZE` reader - this field configures lsc table size in x-direction"]
pub type LSC_XTABLESIZE_R = crate::FieldReader;
#[doc = "Field `LSC_XTABLESIZE` writer - this field configures lsc table size in x-direction"]
pub type LSC_XTABLESIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this field configures lsc table size in x-direction"]
    #[inline(always)]
    pub fn lsc_xtablesize(&self) -> LSC_XTABLESIZE_R {
        LSC_XTABLESIZE_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSC_TABLESIZE")
            .field("lsc_xtablesize", &self.lsc_xtablesize())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this field configures lsc table size in x-direction"]
    #[inline(always)]
    pub fn lsc_xtablesize(&mut self) -> LSC_XTABLESIZE_W<LSC_TABLESIZE_SPEC> {
        LSC_XTABLESIZE_W::new(self, 0)
    }
}
#[doc = "LSC point in x-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`lsc_tablesize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsc_tablesize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSC_TABLESIZE_SPEC;
impl crate::RegisterSpec for LSC_TABLESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_tablesize::R`](R) reader structure"]
impl crate::Readable for LSC_TABLESIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lsc_tablesize::W`](W) writer structure"]
impl crate::Writable for LSC_TABLESIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_TABLESIZE to value 0x1f"]
impl crate::Resettable for LSC_TABLESIZE_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}

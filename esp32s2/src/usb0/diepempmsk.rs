#[doc = "Register `DIEPEMPMSK` reader"]
pub type R = crate::R<DIEPEMPMSK_SPEC>;
#[doc = "Register `DIEPEMPMSK` writer"]
pub type W = crate::W<DIEPEMPMSK_SPEC>;
#[doc = "Field `D_INEPTXFEMPMSK` reader - "]
pub type D_INEPTXFEMPMSK_R = crate::FieldReader<u16>;
#[doc = "Field `D_INEPTXFEMPMSK` writer - "]
pub type D_INEPTXFEMPMSK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn d_ineptxfempmsk(&self) -> D_INEPTXFEMPMSK_R {
        D_INEPTXFEMPMSK_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPEMPMSK")
            .field("d_ineptxfempmsk", &self.d_ineptxfempmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn d_ineptxfempmsk(&mut self) -> D_INEPTXFEMPMSK_W<DIEPEMPMSK_SPEC> {
        D_INEPTXFEMPMSK_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPEMPMSK_SPEC;
impl crate::RegisterSpec for DIEPEMPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepempmsk::R`](R) reader structure"]
impl crate::Readable for DIEPEMPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepempmsk::W`](W) writer structure"]
impl crate::Writable for DIEPEMPMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPEMPMSK to value 0"]
impl crate::Resettable for DIEPEMPMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}

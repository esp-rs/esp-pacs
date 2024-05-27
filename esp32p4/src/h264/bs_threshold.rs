#[doc = "Register `BS_THRESHOLD` reader"]
pub type R = crate::R<BS_THRESHOLD_SPEC>;
#[doc = "Register `BS_THRESHOLD` writer"]
pub type W = crate::W<BS_THRESHOLD_SPEC>;
#[doc = "Field `BS_BUFFER_THRESHOLD` reader - Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb."]
pub type BS_BUFFER_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `BS_BUFFER_THRESHOLD` writer - Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb."]
pub type BS_BUFFER_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb."]
    #[inline(always)]
    pub fn bs_buffer_threshold(&self) -> BS_BUFFER_THRESHOLD_R {
        BS_BUFFER_THRESHOLD_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BS_THRESHOLD")
            .field("bs_buffer_threshold", &self.bs_buffer_threshold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb."]
    #[inline(always)]
    #[must_use]
    pub fn bs_buffer_threshold(&mut self) -> BS_BUFFER_THRESHOLD_W<BS_THRESHOLD_SPEC> {
        BS_BUFFER_THRESHOLD_W::new(self, 0)
    }
}
#[doc = "Bitstream buffer overflow threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bs_threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bs_threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BS_THRESHOLD_SPEC;
impl crate::RegisterSpec for BS_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_threshold::R`](R) reader structure"]
impl crate::Readable for BS_THRESHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bs_threshold::W`](W) writer structure"]
impl crate::Writable for BS_THRESHOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BS_THRESHOLD to value 0x30"]
impl crate::Resettable for BS_THRESHOLD_SPEC {
    const RESET_VALUE: u32 = 0x30;
}

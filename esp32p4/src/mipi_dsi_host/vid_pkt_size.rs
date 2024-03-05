#[doc = "Register `VID_PKT_SIZE` reader"]
pub type R = crate::R<VID_PKT_SIZE_SPEC>;
#[doc = "Register `VID_PKT_SIZE` writer"]
pub type W = crate::W<VID_PKT_SIZE_SPEC>;
#[doc = "Field `VID_PKT_SIZE` reader - NA"]
pub type VID_PKT_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `VID_PKT_SIZE` writer - NA"]
pub type VID_PKT_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - NA"]
    #[inline(always)]
    pub fn vid_pkt_size(&self) -> VID_PKT_SIZE_R {
        VID_PKT_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_PKT_SIZE")
            .field(
                "vid_pkt_size",
                &format_args!("{}", self.vid_pkt_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VID_PKT_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vid_pkt_size(&mut self) -> VID_PKT_SIZE_W<VID_PKT_SIZE_SPEC> {
        VID_PKT_SIZE_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_pkt_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_pkt_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_PKT_SIZE_SPEC;
impl crate::RegisterSpec for VID_PKT_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_pkt_size::R`](R) reader structure"]
impl crate::Readable for VID_PKT_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vid_pkt_size::W`](W) writer structure"]
impl crate::Writable for VID_PKT_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_PKT_SIZE to value 0"]
impl crate::Resettable for VID_PKT_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DIEPTSIZ4` reader"]
pub type R = crate::R<DIEPTSIZ4_SPEC>;
#[doc = "Register `DIEPTSIZ4` writer"]
pub type W = crate::W<DIEPTSIZ4_SPEC>;
#[doc = "Field `D_XFERSIZE4` reader - "]
pub type D_XFERSIZE4_R = crate::FieldReader;
#[doc = "Field `D_XFERSIZE4` writer - "]
pub type D_XFERSIZE4_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `D_PKTCNT4` reader - "]
pub type D_PKTCNT4_R = crate::FieldReader;
#[doc = "Field `D_PKTCNT4` writer - "]
pub type D_PKTCNT4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize4(&self) -> D_XFERSIZE4_R {
        D_XFERSIZE4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt4(&self) -> D_PKTCNT4_R {
        D_PKTCNT4_R::new(((self.bits >> 19) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ4")
            .field(
                "d_xfersize4",
                &format_args!("{}", self.d_xfersize4().bits()),
            )
            .field("d_pktcnt4", &format_args!("{}", self.d_pktcnt4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTSIZ4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfersize4(&mut self) -> D_XFERSIZE4_W<DIEPTSIZ4_SPEC> {
        D_XFERSIZE4_W::new(self, 0)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktcnt4(&mut self) -> D_PKTCNT4_W<DIEPTSIZ4_SPEC> {
        D_PKTCNT4_W::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ4_SPEC;
impl crate::RegisterSpec for DIEPTSIZ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz4::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz4::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ4 to value 0"]
impl crate::Resettable for DIEPTSIZ4_SPEC {
    const RESET_VALUE: u32 = 0;
}

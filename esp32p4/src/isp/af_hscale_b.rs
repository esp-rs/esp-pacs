#[doc = "Register `AF_HSCALE_B` reader"]
pub type R = crate::R<AF_HSCALE_B_SPEC>;
#[doc = "Register `AF_HSCALE_B` writer"]
pub type W = crate::W<AF_HSCALE_B_SPEC>;
#[doc = "Field `AF_RPOINT_B` reader - this field configures left coordinate of focus window b, must >= 2"]
pub type AF_RPOINT_B_R = crate::FieldReader<u16>;
#[doc = "Field `AF_RPOINT_B` writer - this field configures left coordinate of focus window b, must >= 2"]
pub type AF_RPOINT_B_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AF_LPOINT_B` reader - this field configures top coordinate of focus window b, must >= 2"]
pub type AF_LPOINT_B_R = crate::FieldReader<u16>;
#[doc = "Field `AF_LPOINT_B` writer - this field configures top coordinate of focus window b, must >= 2"]
pub type AF_LPOINT_B_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures left coordinate of focus window b, must >= 2"]
    #[inline(always)]
    pub fn af_rpoint_b(&self) -> AF_RPOINT_B_R {
        AF_RPOINT_B_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures top coordinate of focus window b, must >= 2"]
    #[inline(always)]
    pub fn af_lpoint_b(&self) -> AF_LPOINT_B_R {
        AF_LPOINT_B_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_HSCALE_B")
            .field("af_rpoint_b", &self.af_rpoint_b())
            .field("af_lpoint_b", &self.af_lpoint_b())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures left coordinate of focus window b, must >= 2"]
    #[inline(always)]
    #[must_use]
    pub fn af_rpoint_b(&mut self) -> AF_RPOINT_B_W<AF_HSCALE_B_SPEC> {
        AF_RPOINT_B_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures top coordinate of focus window b, must >= 2"]
    #[inline(always)]
    #[must_use]
    pub fn af_lpoint_b(&mut self) -> AF_LPOINT_B_W<AF_HSCALE_B_SPEC> {
        AF_LPOINT_B_W::new(self, 16)
    }
}
#[doc = "h-scale of af window b register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_hscale_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_hscale_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_HSCALE_B_SPEC;
impl crate::RegisterSpec for AF_HSCALE_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_hscale_b::R`](R) reader structure"]
impl crate::Readable for AF_HSCALE_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af_hscale_b::W`](W) writer structure"]
impl crate::Writable for AF_HSCALE_B_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF_HSCALE_B to value 0x0001_0080"]
impl crate::Resettable for AF_HSCALE_B_SPEC {
    const RESET_VALUE: u32 = 0x0001_0080;
}

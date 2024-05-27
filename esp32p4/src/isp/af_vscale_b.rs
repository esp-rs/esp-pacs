#[doc = "Register `AF_VSCALE_B` reader"]
pub type R = crate::R<AF_VSCALE_B_SPEC>;
#[doc = "Register `AF_VSCALE_B` writer"]
pub type W = crate::W<AF_VSCALE_B_SPEC>;
#[doc = "Field `AF_BPOINT_B` reader - this field configures right coordinate of focus window b, must &lt;= hnum-2"]
pub type AF_BPOINT_B_R = crate::FieldReader<u16>;
#[doc = "Field `AF_BPOINT_B` writer - this field configures right coordinate of focus window b, must &lt;= hnum-2"]
pub type AF_BPOINT_B_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AF_TPOINT_B` reader - this field configures bottom coordinate of focus window b, must &lt;= hnum-2"]
pub type AF_TPOINT_B_R = crate::FieldReader<u16>;
#[doc = "Field `AF_TPOINT_B` writer - this field configures bottom coordinate of focus window b, must &lt;= hnum-2"]
pub type AF_TPOINT_B_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures right coordinate of focus window b, must &lt;= hnum-2"]
    #[inline(always)]
    pub fn af_bpoint_b(&self) -> AF_BPOINT_B_R {
        AF_BPOINT_B_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures bottom coordinate of focus window b, must &lt;= hnum-2"]
    #[inline(always)]
    pub fn af_tpoint_b(&self) -> AF_TPOINT_B_R {
        AF_TPOINT_B_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_VSCALE_B")
            .field("af_bpoint_b", &self.af_bpoint_b())
            .field("af_tpoint_b", &self.af_tpoint_b())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures right coordinate of focus window b, must &lt;= hnum-2"]
    #[inline(always)]
    #[must_use]
    pub fn af_bpoint_b(&mut self) -> AF_BPOINT_B_W<AF_VSCALE_B_SPEC> {
        AF_BPOINT_B_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures bottom coordinate of focus window b, must &lt;= hnum-2"]
    #[inline(always)]
    #[must_use]
    pub fn af_tpoint_b(&mut self) -> AF_TPOINT_B_W<AF_VSCALE_B_SPEC> {
        AF_TPOINT_B_W::new(self, 16)
    }
}
#[doc = "v-scale of af window b register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_vscale_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_vscale_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_VSCALE_B_SPEC;
impl crate::RegisterSpec for AF_VSCALE_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_vscale_b::R`](R) reader structure"]
impl crate::Readable for AF_VSCALE_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af_vscale_b::W`](W) writer structure"]
impl crate::Writable for AF_VSCALE_B_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF_VSCALE_B to value 0x0001_0080"]
impl crate::Resettable for AF_VSCALE_B_SPEC {
    const RESET_VALUE: u32 = 0x0001_0080;
}

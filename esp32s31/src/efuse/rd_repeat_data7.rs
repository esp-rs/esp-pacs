#[doc = "Register `RD_REPEAT_DATA7` reader"]
pub type R = crate::R<RD_REPEAT_DATA7_SPEC>;
#[doc = "Register `RD_REPEAT_DATA7` writer"]
pub type W = crate::W<RD_REPEAT_DATA7_SPEC>;
#[doc = "Field `REPEAT7_RSVD` reader - Reserved."]
pub type REPEAT7_RSVD_R = crate::FieldReader<u16>;
#[doc = "Field `RD_RESERVE_0_272` reader - "]
pub type RD_RESERVE_0_272_R = crate::FieldReader<u16>;
#[doc = "Field `RD_RESERVE_0_272` writer - "]
pub type RD_RESERVE_0_272_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved."]
    #[inline(always)]
    pub fn repeat7_rsvd(&self) -> REPEAT7_RSVD_R {
        REPEAT7_RSVD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rd_reserve_0_272(&self) -> RD_RESERVE_0_272_R {
        RD_RESERVE_0_272_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA7")
            .field("repeat7_rsvd", &self.repeat7_rsvd())
            .field("rd_reserve_0_272", &self.rd_reserve_0_272())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rd_reserve_0_272(&mut self) -> RD_RESERVE_0_272_W<'_, RD_REPEAT_DATA7_SPEC> {
        RD_RESERVE_0_272_W::new(self, 16)
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_repeat_data7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA7_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data7::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_repeat_data7::W`](W) writer structure"]
impl crate::Writable for RD_REPEAT_DATA7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA7 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA7_SPEC {}

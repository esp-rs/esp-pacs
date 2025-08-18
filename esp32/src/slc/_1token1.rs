#[doc = "Register `_1TOKEN1` reader"]
pub type R = crate::R<_1TOKEN1_SPEC>;
#[doc = "Register `_1TOKEN1` writer"]
pub type W = crate::W<_1TOKEN1_SPEC>;
#[doc = "Field `SLC1_TOKEN1_WDATA` writer - "]
pub type SLC1_TOKEN1_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SLC1_TOKEN1_WR` writer - "]
pub type SLC1_TOKEN1_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TOKEN1_INC` writer - "]
pub type SLC1_TOKEN1_INC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TOKEN1_INC_MORE` writer - "]
pub type SLC1_TOKEN1_INC_MORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TOKEN1` reader - "]
pub type SLC1_TOKEN1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc1_token1(&self) -> SLC1_TOKEN1_R {
        SLC1_TOKEN1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1TOKEN1")
            .field("slc1_token1", &self.slc1_token1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc1_token1_wdata(&mut self) -> SLC1_TOKEN1_WDATA_W<'_, _1TOKEN1_SPEC> {
        SLC1_TOKEN1_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc1_token1_wr(&mut self) -> SLC1_TOKEN1_WR_W<'_, _1TOKEN1_SPEC> {
        SLC1_TOKEN1_WR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc1_token1_inc(&mut self) -> SLC1_TOKEN1_INC_W<'_, _1TOKEN1_SPEC> {
        SLC1_TOKEN1_INC_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_token1_inc_more(&mut self) -> SLC1_TOKEN1_INC_MORE_W<'_, _1TOKEN1_SPEC> {
        SLC1_TOKEN1_INC_MORE_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`_1token1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1token1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1TOKEN1_SPEC;
impl crate::RegisterSpec for _1TOKEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1token1::R`](R) reader structure"]
impl crate::Readable for _1TOKEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_1token1::W`](W) writer structure"]
impl crate::Writable for _1TOKEN1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _1TOKEN1 to value 0"]
impl crate::Resettable for _1TOKEN1_SPEC {}

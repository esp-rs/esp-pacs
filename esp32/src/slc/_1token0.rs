#[doc = "Register `_1TOKEN0` reader"]
pub type R = crate::R<_1TOKEN0_SPEC>;
#[doc = "Register `_1TOKEN0` writer"]
pub type W = crate::W<_1TOKEN0_SPEC>;
#[doc = "Field `SLC1_TOKEN0_WDATA` writer - "]
pub type SLC1_TOKEN0_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SLC1_TOKEN0_WR` writer - "]
pub type SLC1_TOKEN0_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TOKEN0_INC` writer - "]
pub type SLC1_TOKEN0_INC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TOKEN0_INC_MORE` writer - "]
pub type SLC1_TOKEN0_INC_MORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TOKEN0` reader - "]
pub type SLC1_TOKEN0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc1_token0(&self) -> SLC1_TOKEN0_R {
        SLC1_TOKEN0_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1TOKEN0")
            .field("slc1_token0", &self.slc1_token0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc1_token0_wdata(&mut self) -> SLC1_TOKEN0_WDATA_W<_1TOKEN0_SPEC> {
        SLC1_TOKEN0_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc1_token0_wr(&mut self) -> SLC1_TOKEN0_WR_W<_1TOKEN0_SPEC> {
        SLC1_TOKEN0_WR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc1_token0_inc(&mut self) -> SLC1_TOKEN0_INC_W<_1TOKEN0_SPEC> {
        SLC1_TOKEN0_INC_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_token0_inc_more(&mut self) -> SLC1_TOKEN0_INC_MORE_W<_1TOKEN0_SPEC> {
        SLC1_TOKEN0_INC_MORE_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`_1token0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1token0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1TOKEN0_SPEC;
impl crate::RegisterSpec for _1TOKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1token0::R`](R) reader structure"]
impl crate::Readable for _1TOKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_1token0::W`](W) writer structure"]
impl crate::Writable for _1TOKEN0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _1TOKEN0 to value 0"]
impl crate::Resettable for _1TOKEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}

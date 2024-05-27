///Register `_0TOKEN1` reader
pub type R = crate::R<_0TOKEN1_SPEC>;
///Register `_0TOKEN1` writer
pub type W = crate::W<_0TOKEN1_SPEC>;
///Field `SLC0_TOKEN1_WDATA` writer -
pub type SLC0_TOKEN1_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `SLC0_TOKEN1_WR` writer -
pub type SLC0_TOKEN1_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC0_TOKEN1_INC` writer -
pub type SLC0_TOKEN1_INC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC0_TOKEN1_INC_MORE` writer -
pub type SLC0_TOKEN1_INC_MORE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC0_TOKEN1` reader -
pub type SLC0_TOKEN1_R = crate::FieldReader<u16>;
impl R {
    ///Bits 16:27
    #[inline(always)]
    pub fn slc0_token1(&self) -> SLC0_TOKEN1_R {
        SLC0_TOKEN1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0TOKEN1")
            .field("slc0_token1", &self.slc0_token1())
            .finish()
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    #[must_use]
    pub fn slc0_token1_wdata(&mut self) -> SLC0_TOKEN1_WDATA_W<_0TOKEN1_SPEC> {
        SLC0_TOKEN1_WDATA_W::new(self, 0)
    }
    ///Bit 12
    #[inline(always)]
    #[must_use]
    pub fn slc0_token1_wr(&mut self) -> SLC0_TOKEN1_WR_W<_0TOKEN1_SPEC> {
        SLC0_TOKEN1_WR_W::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    #[must_use]
    pub fn slc0_token1_inc(&mut self) -> SLC0_TOKEN1_INC_W<_0TOKEN1_SPEC> {
        SLC0_TOKEN1_INC_W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    #[must_use]
    pub fn slc0_token1_inc_more(&mut self) -> SLC0_TOKEN1_INC_MORE_W<_0TOKEN1_SPEC> {
        SLC0_TOKEN1_INC_MORE_W::new(self, 14)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`_0token1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0token1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct _0TOKEN1_SPEC;
impl crate::RegisterSpec for _0TOKEN1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`_0token1::R`](R) reader structure
impl crate::Readable for _0TOKEN1_SPEC {}
///`write(|w| ..)` method takes [`_0token1::W`](W) writer structure
impl crate::Writable for _0TOKEN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets _0TOKEN1 to value 0
impl crate::Resettable for _0TOKEN1_SPEC {
    const RESET_VALUE: u32 = 0;
}

///Register `TO_CNT_CFG` reader
pub type R = crate::R<TO_CNT_CFG_SPEC>;
///Register `TO_CNT_CFG` writer
pub type W = crate::W<TO_CNT_CFG_SPEC>;
///Field `LPRX_TO_CNT` reader - NA
pub type LPRX_TO_CNT_R = crate::FieldReader<u16>;
///Field `LPRX_TO_CNT` writer - NA
pub type LPRX_TO_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `HSTX_TO_CNT` reader - NA
pub type HSTX_TO_CNT_R = crate::FieldReader<u16>;
///Field `HSTX_TO_CNT` writer - NA
pub type HSTX_TO_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - NA
    #[inline(always)]
    pub fn lprx_to_cnt(&self) -> LPRX_TO_CNT_R {
        LPRX_TO_CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - NA
    #[inline(always)]
    pub fn hstx_to_cnt(&self) -> HSTX_TO_CNT_R {
        HSTX_TO_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TO_CNT_CFG")
            .field("lprx_to_cnt", &self.lprx_to_cnt())
            .field("hstx_to_cnt", &self.hstx_to_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - NA
    #[inline(always)]
    #[must_use]
    pub fn lprx_to_cnt(&mut self) -> LPRX_TO_CNT_W<TO_CNT_CFG_SPEC> {
        LPRX_TO_CNT_W::new(self, 0)
    }
    ///Bits 16:31 - NA
    #[inline(always)]
    #[must_use]
    pub fn hstx_to_cnt(&mut self) -> HSTX_TO_CNT_W<TO_CNT_CFG_SPEC> {
        HSTX_TO_CNT_W::new(self, 16)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`to_cnt_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to_cnt_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TO_CNT_CFG_SPEC;
impl crate::RegisterSpec for TO_CNT_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`to_cnt_cfg::R`](R) reader structure
impl crate::Readable for TO_CNT_CFG_SPEC {}
///`write(|w| ..)` method takes [`to_cnt_cfg::W`](W) writer structure
impl crate::Writable for TO_CNT_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TO_CNT_CFG to value 0
impl crate::Resettable for TO_CNT_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}

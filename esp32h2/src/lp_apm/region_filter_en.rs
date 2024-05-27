///Register `REGION_FILTER_EN` reader
pub type R = crate::R<REGION_FILTER_EN_SPEC>;
///Register `REGION_FILTER_EN` writer
pub type W = crate::W<REGION_FILTER_EN_SPEC>;
///Field `REGION_FILTER_EN` reader - Region filter enable
pub type REGION_FILTER_EN_R = crate::FieldReader;
///Field `REGION_FILTER_EN` writer - Region filter enable
pub type REGION_FILTER_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Region filter enable
    #[inline(always)]
    pub fn region_filter_en(&self) -> REGION_FILTER_EN_R {
        REGION_FILTER_EN_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_FILTER_EN")
            .field("region_filter_en", &self.region_filter_en())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Region filter enable
    #[inline(always)]
    #[must_use]
    pub fn region_filter_en(&mut self) -> REGION_FILTER_EN_W<REGION_FILTER_EN_SPEC> {
        REGION_FILTER_EN_W::new(self, 0)
    }
}
/**Region filter enable register

You can [`read`](crate::generic::Reg::read) this register and get [`region_filter_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region_filter_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REGION_FILTER_EN_SPEC;
impl crate::RegisterSpec for REGION_FILTER_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`region_filter_en::R`](R) reader structure
impl crate::Readable for REGION_FILTER_EN_SPEC {}
///`write(|w| ..)` method takes [`region_filter_en::W`](W) writer structure
impl crate::Writable for REGION_FILTER_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REGION_FILTER_EN to value 0x01
impl crate::Resettable for REGION_FILTER_EN_SPEC {
    const RESET_VALUE: u32 = 0x01;
}

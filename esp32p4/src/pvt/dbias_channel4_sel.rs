///Register `DBIAS_CHANNEL4_SEL` reader
pub type R = crate::R<DBIAS_CHANNEL4_SEL_SPEC>;
///Register `DBIAS_CHANNEL4_SEL` writer
pub type W = crate::W<DBIAS_CHANNEL4_SEL_SPEC>;
///Field `DBIAS_CHANNEL4_CFG` reader - needs field desc
pub type DBIAS_CHANNEL4_CFG_R = crate::FieldReader<u32>;
///Field `DBIAS_CHANNEL4_CFG` writer - needs field desc
pub type DBIAS_CHANNEL4_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    ///Bits 0:16 - needs field desc
    #[inline(always)]
    pub fn dbias_channel4_cfg(&self) -> DBIAS_CHANNEL4_CFG_R {
        DBIAS_CHANNEL4_CFG_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBIAS_CHANNEL4_SEL")
            .field("dbias_channel4_cfg", &self.dbias_channel4_cfg())
            .finish()
    }
}
impl W {
    ///Bits 0:16 - needs field desc
    #[inline(always)]
    #[must_use]
    pub fn dbias_channel4_cfg(&mut self) -> DBIAS_CHANNEL4_CFG_W<DBIAS_CHANNEL4_SEL_SPEC> {
        DBIAS_CHANNEL4_CFG_W::new(self, 0)
    }
}
/**needs desc

You can [`read`](crate::generic::Reg::read) this register and get [`dbias_channel4_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_channel4_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DBIAS_CHANNEL4_SEL_SPEC;
impl crate::RegisterSpec for DBIAS_CHANNEL4_SEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dbias_channel4_sel::R`](R) reader structure
impl crate::Readable for DBIAS_CHANNEL4_SEL_SPEC {}
///`write(|w| ..)` method takes [`dbias_channel4_sel::W`](W) writer structure
impl crate::Writable for DBIAS_CHANNEL4_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBIAS_CHANNEL4_SEL to value 0
impl crate::Resettable for DBIAS_CHANNEL4_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}

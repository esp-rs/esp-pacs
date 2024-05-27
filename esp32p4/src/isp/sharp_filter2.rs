///Register `SHARP_FILTER2` reader
pub type R = crate::R<SHARP_FILTER2_SPEC>;
///Register `SHARP_FILTER2` writer
pub type W = crate::W<SHARP_FILTER2_SPEC>;
///Field `SHARP_FILTER_COE20` reader - this field configures usm filter coefficient
pub type SHARP_FILTER_COE20_R = crate::FieldReader;
///Field `SHARP_FILTER_COE20` writer - this field configures usm filter coefficient
pub type SHARP_FILTER_COE20_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SHARP_FILTER_COE21` reader - this field configures usm filter coefficient
pub type SHARP_FILTER_COE21_R = crate::FieldReader;
///Field `SHARP_FILTER_COE21` writer - this field configures usm filter coefficient
pub type SHARP_FILTER_COE21_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SHARP_FILTER_COE22` reader - this field configures usm filter coefficient
pub type SHARP_FILTER_COE22_R = crate::FieldReader;
///Field `SHARP_FILTER_COE22` writer - this field configures usm filter coefficient
pub type SHARP_FILTER_COE22_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - this field configures usm filter coefficient
    #[inline(always)]
    pub fn sharp_filter_coe20(&self) -> SHARP_FILTER_COE20_R {
        SHARP_FILTER_COE20_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - this field configures usm filter coefficient
    #[inline(always)]
    pub fn sharp_filter_coe21(&self) -> SHARP_FILTER_COE21_R {
        SHARP_FILTER_COE21_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - this field configures usm filter coefficient
    #[inline(always)]
    pub fn sharp_filter_coe22(&self) -> SHARP_FILTER_COE22_R {
        SHARP_FILTER_COE22_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHARP_FILTER2")
            .field("sharp_filter_coe20", &self.sharp_filter_coe20())
            .field("sharp_filter_coe21", &self.sharp_filter_coe21())
            .field("sharp_filter_coe22", &self.sharp_filter_coe22())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - this field configures usm filter coefficient
    #[inline(always)]
    #[must_use]
    pub fn sharp_filter_coe20(&mut self) -> SHARP_FILTER_COE20_W<SHARP_FILTER2_SPEC> {
        SHARP_FILTER_COE20_W::new(self, 0)
    }
    ///Bits 5:9 - this field configures usm filter coefficient
    #[inline(always)]
    #[must_use]
    pub fn sharp_filter_coe21(&mut self) -> SHARP_FILTER_COE21_W<SHARP_FILTER2_SPEC> {
        SHARP_FILTER_COE21_W::new(self, 5)
    }
    ///Bits 10:14 - this field configures usm filter coefficient
    #[inline(always)]
    #[must_use]
    pub fn sharp_filter_coe22(&mut self) -> SHARP_FILTER_COE22_W<SHARP_FILTER2_SPEC> {
        SHARP_FILTER_COE22_W::new(self, 10)
    }
}
/**sharp usm config register 2

You can [`read`](crate::generic::Reg::read) this register and get [`sharp_filter2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sharp_filter2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SHARP_FILTER2_SPEC;
impl crate::RegisterSpec for SHARP_FILTER2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sharp_filter2::R`](R) reader structure
impl crate::Readable for SHARP_FILTER2_SPEC {}
///`write(|w| ..)` method takes [`sharp_filter2::W`](W) writer structure
impl crate::Writable for SHARP_FILTER2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SHARP_FILTER2 to value 0x0441
impl crate::Resettable for SHARP_FILTER2_SPEC {
    const RESET_VALUE: u32 = 0x0441;
}

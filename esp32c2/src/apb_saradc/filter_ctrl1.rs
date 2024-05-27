///Register `FILTER_CTRL1` reader
pub type R = crate::R<FILTER_CTRL1_SPEC>;
///Register `FILTER_CTRL1` writer
pub type W = crate::W<FILTER_CTRL1_SPEC>;
///Field `FILTER_FACTOR1` reader - Need add description
pub type FILTER_FACTOR1_R = crate::FieldReader;
///Field `FILTER_FACTOR1` writer - Need add description
pub type FILTER_FACTOR1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FILTER_FACTOR0` reader - Need add description
pub type FILTER_FACTOR0_R = crate::FieldReader;
///Field `FILTER_FACTOR0` writer - Need add description
pub type FILTER_FACTOR0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 26:28 - Need add description
    #[inline(always)]
    pub fn filter_factor1(&self) -> FILTER_FACTOR1_R {
        FILTER_FACTOR1_R::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bits 29:31 - Need add description
    #[inline(always)]
    pub fn filter_factor0(&self) -> FILTER_FACTOR0_R {
        FILTER_FACTOR0_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CTRL1")
            .field("filter_factor1", &self.filter_factor1())
            .field("filter_factor0", &self.filter_factor0())
            .finish()
    }
}
impl W {
    ///Bits 26:28 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn filter_factor1(&mut self) -> FILTER_FACTOR1_W<FILTER_CTRL1_SPEC> {
        FILTER_FACTOR1_W::new(self, 26)
    }
    ///Bits 29:31 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn filter_factor0(&mut self) -> FILTER_FACTOR0_W<FILTER_CTRL1_SPEC> {
        FILTER_FACTOR0_W::new(self, 29)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`filter_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FILTER_CTRL1_SPEC;
impl crate::RegisterSpec for FILTER_CTRL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`filter_ctrl1::R`](R) reader structure
impl crate::Readable for FILTER_CTRL1_SPEC {}
///`write(|w| ..)` method takes [`filter_ctrl1::W`](W) writer structure
impl crate::Writable for FILTER_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FILTER_CTRL1 to value 0
impl crate::Resettable for FILTER_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}

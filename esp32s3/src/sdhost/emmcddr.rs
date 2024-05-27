///Register `EMMCDDR` reader
pub type R = crate::R<EMMCDDR_SPEC>;
///Register `EMMCDDR` writer
pub type W = crate::W<EMMCDDR_SPEC>;
///Field `HALFSTARTBIT` reader - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle.
pub type HALFSTARTBIT_R = crate::FieldReader;
///Field `HALFSTARTBIT` writer - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle.
pub type HALFSTARTBIT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HS400_MODE` reader - Set 1 to enable HS400 mode.
pub type HS400_MODE_R = crate::BitReader;
///Field `HS400_MODE` writer - Set 1 to enable HS400 mode.
pub type HS400_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle.
    #[inline(always)]
    pub fn halfstartbit(&self) -> HALFSTARTBIT_R {
        HALFSTARTBIT_R::new((self.bits & 3) as u8)
    }
    ///Bit 31 - Set 1 to enable HS400 mode.
    #[inline(always)]
    pub fn hs400_mode(&self) -> HS400_MODE_R {
        HS400_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMMCDDR")
            .field("halfstartbit", &self.halfstartbit())
            .field("hs400_mode", &self.hs400_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle.
    #[inline(always)]
    #[must_use]
    pub fn halfstartbit(&mut self) -> HALFSTARTBIT_W<EMMCDDR_SPEC> {
        HALFSTARTBIT_W::new(self, 0)
    }
    ///Bit 31 - Set 1 to enable HS400 mode.
    #[inline(always)]
    #[must_use]
    pub fn hs400_mode(&mut self) -> HS400_MODE_W<EMMCDDR_SPEC> {
        HS400_MODE_W::new(self, 31)
    }
}
/**eMMC DDR register

You can [`read`](crate::generic::Reg::read) this register and get [`emmcddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EMMCDDR_SPEC;
impl crate::RegisterSpec for EMMCDDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`emmcddr::R`](R) reader structure
impl crate::Readable for EMMCDDR_SPEC {}
///`write(|w| ..)` method takes [`emmcddr::W`](W) writer structure
impl crate::Writable for EMMCDDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EMMCDDR to value 0
impl crate::Resettable for EMMCDDR_SPEC {
    const RESET_VALUE: u32 = 0;
}

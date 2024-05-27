///Register `AWB_MODE` reader
pub type R = crate::R<AWB_MODE_SPEC>;
///Register `AWB_MODE` writer
pub type W = crate::W<AWB_MODE_SPEC>;
///Field `AWB_MODE` reader - this field configures awb algo sel. 00: none sellected. 01: sel algo0. 10: sel algo1. 11: sel both algo0 and algo1
pub type AWB_MODE_R = crate::FieldReader;
///Field `AWB_MODE` writer - this field configures awb algo sel. 00: none sellected. 01: sel algo0. 10: sel algo1. 11: sel both algo0 and algo1
pub type AWB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AWB_SAMPLE` reader - this bit configures awb sample location, 0:before ccm, 1:after ccm
pub type AWB_SAMPLE_R = crate::BitReader;
///Field `AWB_SAMPLE` writer - this bit configures awb sample location, 0:before ccm, 1:after ccm
pub type AWB_SAMPLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - this field configures awb algo sel. 00: none sellected. 01: sel algo0. 10: sel algo1. 11: sel both algo0 and algo1
    #[inline(always)]
    pub fn awb_mode(&self) -> AWB_MODE_R {
        AWB_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - this bit configures awb sample location, 0:before ccm, 1:after ccm
    #[inline(always)]
    pub fn awb_sample(&self) -> AWB_SAMPLE_R {
        AWB_SAMPLE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB_MODE")
            .field("awb_mode", &self.awb_mode())
            .field("awb_sample", &self.awb_sample())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - this field configures awb algo sel. 00: none sellected. 01: sel algo0. 10: sel algo1. 11: sel both algo0 and algo1
    #[inline(always)]
    #[must_use]
    pub fn awb_mode(&mut self) -> AWB_MODE_W<AWB_MODE_SPEC> {
        AWB_MODE_W::new(self, 0)
    }
    ///Bit 4 - this bit configures awb sample location, 0:before ccm, 1:after ccm
    #[inline(always)]
    #[must_use]
    pub fn awb_sample(&mut self) -> AWB_SAMPLE_W<AWB_MODE_SPEC> {
        AWB_SAMPLE_W::new(self, 4)
    }
}
/**awb mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`awb_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AWB_MODE_SPEC;
impl crate::RegisterSpec for AWB_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`awb_mode::R`](R) reader structure
impl crate::Readable for AWB_MODE_SPEC {}
///`write(|w| ..)` method takes [`awb_mode::W`](W) writer structure
impl crate::Writable for AWB_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AWB_MODE to value 0x03
impl crate::Resettable for AWB_MODE_SPEC {
    const RESET_VALUE: u32 = 0x03;
}

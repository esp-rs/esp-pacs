#[doc = "Register `REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER` reader"]
pub type R = crate::R<REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER_SPEC>;
#[doc = "Register `REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER` writer"]
pub type W = crate::W<REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER_SPEC>;
#[doc = "Field `TSSS` reader - Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 046 ns When Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_, each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF"]
pub type TSSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSSS` writer - Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 046 ns When Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_, each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF"]
pub type TSSS_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `ADDSUB` reader - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register When this bit is reset, the time value is added with the contents of the update register"]
pub type ADDSUB_R = crate::BitReader;
#[doc = "Field `ADDSUB` writer - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register When this bit is reset, the time value is added with the contents of the update register"]
pub type ADDSUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 046 ns When Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_, each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register When this bit is reset, the time value is added with the contents of the update register"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER")
            .field("tsss", &self.tsss())
            .field("addsub", &self.addsub())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 046 ns When Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_, each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF"]
    #[inline(always)]
    pub fn tsss(&mut self) -> TSSS_W<'_, REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER_SPEC> {
        TSSS_W::new(self, 0)
    }
    #[doc = "Bit 31 - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register When this bit is reset, the time value is added with the contents of the update register"]
    #[inline(always)]
    pub fn addsub(&mut self) -> ADDSUB_W<'_, REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER_SPEC> {
        ADDSUB_W::new(self, 31)
    }
}
#[doc = "Contains 32 bits of the nanoseconds field to be written to, added to, or subtracted from the System Time value This register is only present when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register453_systemtimenanosecondsupdateregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register453_systemtimenanosecondsupdateregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register453_systemtimenanosecondsupdateregister::R`](R) reader structure"]
impl crate::Readable for REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register453_systemtimenanosecondsupdateregister::W`](W) writer structure"]
impl crate::Writable for REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER to value 0"]
impl crate::Resettable for REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER_SPEC {}

///Register `TO` reader
pub type R = crate::R<TO_SPEC>;
///Register `TO` writer
pub type W = crate::W<TO_SPEC>;
///Field `TIME_OUT_VALUE` reader - This register is used to configure the timeout for receiving a data bit in APB clock cycles.
pub type TIME_OUT_VALUE_R = crate::FieldReader;
///Field `TIME_OUT_VALUE` writer - This register is used to configure the timeout for receiving a data bit in APB clock cycles.
pub type TIME_OUT_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TIME_OUT_EN` reader - This is the enable bit for time out control.
pub type TIME_OUT_EN_R = crate::BitReader;
///Field `TIME_OUT_EN` writer - This is the enable bit for time out control.
pub type TIME_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - This register is used to configure the timeout for receiving a data bit in APB clock cycles.
    #[inline(always)]
    pub fn time_out_value(&self) -> TIME_OUT_VALUE_R {
        TIME_OUT_VALUE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - This is the enable bit for time out control.
    #[inline(always)]
    pub fn time_out_en(&self) -> TIME_OUT_EN_R {
        TIME_OUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TO")
            .field("time_out_value", &self.time_out_value())
            .field("time_out_en", &self.time_out_en())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - This register is used to configure the timeout for receiving a data bit in APB clock cycles.
    #[inline(always)]
    #[must_use]
    pub fn time_out_value(&mut self) -> TIME_OUT_VALUE_W<TO_SPEC> {
        TIME_OUT_VALUE_W::new(self, 0)
    }
    ///Bit 5 - This is the enable bit for time out control.
    #[inline(always)]
    #[must_use]
    pub fn time_out_en(&mut self) -> TIME_OUT_EN_W<TO_SPEC> {
        TIME_OUT_EN_W::new(self, 5)
    }
}
/**Setting time out control for receiving data.

You can [`read`](crate::generic::Reg::read) this register and get [`to::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TO_SPEC;
impl crate::RegisterSpec for TO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`to::R`](R) reader structure
impl crate::Readable for TO_SPEC {}
///`write(|w| ..)` method takes [`to::W`](W) writer structure
impl crate::Writable for TO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TO to value 0x10
impl crate::Resettable for TO_SPEC {
    const RESET_VALUE: u32 = 0x10;
}

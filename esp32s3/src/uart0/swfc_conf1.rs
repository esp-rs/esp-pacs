///Register `SWFC_CONF1` reader
pub type R = crate::R<SWFC_CONF1_SPEC>;
///Register `SWFC_CONF1` writer
pub type W = crate::W<SWFC_CONF1_SPEC>;
///Field `XON_THRESHOLD` reader - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1, it will send a Xon char.
pub type XON_THRESHOLD_R = crate::FieldReader<u16>;
///Field `XON_THRESHOLD` writer - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1, it will send a Xon char.
pub type XON_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `XON_CHAR` reader - This register stores the Xon flow control char.
pub type XON_CHAR_R = crate::FieldReader;
///Field `XON_CHAR` writer - This register stores the Xon flow control char.
pub type XON_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:9 - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1, it will send a Xon char.
    #[inline(always)]
    pub fn xon_threshold(&self) -> XON_THRESHOLD_R {
        XON_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:17 - This register stores the Xon flow control char.
    #[inline(always)]
    pub fn xon_char(&self) -> XON_CHAR_R {
        XON_CHAR_R::new(((self.bits >> 10) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWFC_CONF1")
            .field("xon_threshold", &self.xon_threshold())
            .field("xon_char", &self.xon_char())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1, it will send a Xon char.
    #[inline(always)]
    #[must_use]
    pub fn xon_threshold(&mut self) -> XON_THRESHOLD_W<SWFC_CONF1_SPEC> {
        XON_THRESHOLD_W::new(self, 0)
    }
    ///Bits 10:17 - This register stores the Xon flow control char.
    #[inline(always)]
    #[must_use]
    pub fn xon_char(&mut self) -> XON_CHAR_W<SWFC_CONF1_SPEC> {
        XON_CHAR_W::new(self, 10)
    }
}
/**Software flow-control character configuration

You can [`read`](crate::generic::Reg::read) this register and get [`swfc_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swfc_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SWFC_CONF1_SPEC;
impl crate::RegisterSpec for SWFC_CONF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`swfc_conf1::R`](R) reader structure
impl crate::Readable for SWFC_CONF1_SPEC {}
///`write(|w| ..)` method takes [`swfc_conf1::W`](W) writer structure
impl crate::Writable for SWFC_CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SWFC_CONF1 to value 0x4400
impl crate::Resettable for SWFC_CONF1_SPEC {
    const RESET_VALUE: u32 = 0x4400;
}

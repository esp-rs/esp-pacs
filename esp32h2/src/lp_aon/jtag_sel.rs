///Register `JTAG_SEL` reader
pub type R = crate::R<JTAG_SEL_SPEC>;
///Register `JTAG_SEL` writer
pub type W = crate::W<JTAG_SEL_SPEC>;
///Field `SOFT` reader - If strapping_sel_jtag feature is disabled by efuse, and if neither pad_jtag or usb_jtag is disabled by efuse, this field determines which one jtag between usb_jtag and pad_jtag will be used. 1'b1(default): usb_jtag, 1'b0: pad_jtag.
pub type SOFT_R = crate::BitReader;
///Field `SOFT` writer - If strapping_sel_jtag feature is disabled by efuse, and if neither pad_jtag or usb_jtag is disabled by efuse, this field determines which one jtag between usb_jtag and pad_jtag will be used. 1'b1(default): usb_jtag, 1'b0: pad_jtag.
pub type SOFT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - If strapping_sel_jtag feature is disabled by efuse, and if neither pad_jtag or usb_jtag is disabled by efuse, this field determines which one jtag between usb_jtag and pad_jtag will be used. 1'b1(default): usb_jtag, 1'b0: pad_jtag.
    #[inline(always)]
    pub fn soft(&self) -> SOFT_R {
        SOFT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAG_SEL")
            .field("soft", &self.soft())
            .finish()
    }
}
impl W {
    ///Bit 31 - If strapping_sel_jtag feature is disabled by efuse, and if neither pad_jtag or usb_jtag is disabled by efuse, this field determines which one jtag between usb_jtag and pad_jtag will be used. 1'b1(default): usb_jtag, 1'b0: pad_jtag.
    #[inline(always)]
    #[must_use]
    pub fn soft(&mut self) -> SOFT_W<JTAG_SEL_SPEC> {
        SOFT_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`jtag_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JTAG_SEL_SPEC;
impl crate::RegisterSpec for JTAG_SEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jtag_sel::R`](R) reader structure
impl crate::Readable for JTAG_SEL_SPEC {}
///`write(|w| ..)` method takes [`jtag_sel::W`](W) writer structure
impl crate::Writable for JTAG_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JTAG_SEL to value 0x8000_0000
impl crate::Resettable for JTAG_SEL_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}

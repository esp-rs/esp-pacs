///Register `FUNC%s_IN_SEL_CFG` reader
pub type R = crate::R<FUNC_IN_SEL_CFG_SPEC>;
///Register `FUNC%s_IN_SEL_CFG` writer
pub type W = crate::W<FUNC_IN_SEL_CFG_SPEC>;
///Field `IN_SEL` reader - set this value: s=0-56: connect GPIO\[s\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level.
pub type IN_SEL_R = crate::FieldReader;
///Field `IN_SEL` writer - set this value: s=0-56: connect GPIO\[s\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level.
pub type IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `IN_INV_SEL` reader - set this bit to invert input signal. 1:invert. 0:not invert.
pub type IN_INV_SEL_R = crate::BitReader;
///Field `IN_INV_SEL` writer - set this bit to invert input signal. 1:invert. 0:not invert.
pub type IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL` reader - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO.
pub type SEL_R = crate::BitReader;
///Field `SEL` writer - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO.
pub type SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - set this value: s=0-56: connect GPIO\[s\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level.
    #[inline(always)]
    pub fn in_sel(&self) -> IN_SEL_R {
        IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert.
    #[inline(always)]
    pub fn in_inv_sel(&self) -> IN_INV_SEL_R {
        IN_INV_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO.
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_IN_SEL_CFG")
            .field("in_sel", &self.in_sel())
            .field("in_inv_sel", &self.in_inv_sel())
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - set this value: s=0-56: connect GPIO\[s\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level.
    #[inline(always)]
    #[must_use]
    pub fn in_sel(&mut self) -> IN_SEL_W<FUNC_IN_SEL_CFG_SPEC> {
        IN_SEL_W::new(self, 0)
    }
    ///Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert.
    #[inline(always)]
    #[must_use]
    pub fn in_inv_sel(&mut self) -> IN_INV_SEL_W<FUNC_IN_SEL_CFG_SPEC> {
        IN_INV_SEL_W::new(self, 6)
    }
    ///Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO.
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<FUNC_IN_SEL_CFG_SPEC> {
        SEL_W::new(self, 7)
    }
}
/**GPIO input function configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`func_in_sel_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FUNC_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`func_in_sel_cfg::R`](R) reader structure
impl crate::Readable for FUNC_IN_SEL_CFG_SPEC {}
///`write(|w| ..)` method takes [`func_in_sel_cfg::W`](W) writer structure
impl crate::Writable for FUNC_IN_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FUNC%s_IN_SEL_CFG to value 0
impl crate::Resettable for FUNC_IN_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}

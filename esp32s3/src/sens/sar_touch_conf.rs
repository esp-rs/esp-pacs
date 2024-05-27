///Register `SAR_TOUCH_CONF` reader
pub type R = crate::R<SAR_TOUCH_CONF_SPEC>;
///Register `SAR_TOUCH_CONF` writer
pub type W = crate::W<SAR_TOUCH_CONF_SPEC>;
///Field `SAR_TOUCH_OUTEN` reader - touch controller output enable
pub type SAR_TOUCH_OUTEN_R = crate::FieldReader<u16>;
///Field `SAR_TOUCH_OUTEN` writer - touch controller output enable
pub type SAR_TOUCH_OUTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `SAR_TOUCH_STATUS_CLR` writer - clear all touch active status
pub type SAR_TOUCH_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_TOUCH_DATA_SEL` reader - 3: smooth data 2: baseline 1,0: raw_data
pub type SAR_TOUCH_DATA_SEL_R = crate::FieldReader;
///Field `SAR_TOUCH_DATA_SEL` writer - 3: smooth data 2: baseline 1,0: raw_data
pub type SAR_TOUCH_DATA_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SAR_TOUCH_DENOISE_END` reader - touch_denoise_done
pub type SAR_TOUCH_DENOISE_END_R = crate::BitReader;
///Field `SAR_TOUCH_UNIT_END` reader - touch_unit_done
pub type SAR_TOUCH_UNIT_END_R = crate::BitReader;
///Field `SAR_TOUCH_APPROACH_PAD2` reader - indicate which pad is approach pad2
pub type SAR_TOUCH_APPROACH_PAD2_R = crate::FieldReader;
///Field `SAR_TOUCH_APPROACH_PAD2` writer - indicate which pad is approach pad2
pub type SAR_TOUCH_APPROACH_PAD2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SAR_TOUCH_APPROACH_PAD1` reader - indicate which pad is approach pad1
pub type SAR_TOUCH_APPROACH_PAD1_R = crate::FieldReader;
///Field `SAR_TOUCH_APPROACH_PAD1` writer - indicate which pad is approach pad1
pub type SAR_TOUCH_APPROACH_PAD1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SAR_TOUCH_APPROACH_PAD0` reader - indicate which pad is approach pad0
pub type SAR_TOUCH_APPROACH_PAD0_R = crate::FieldReader;
///Field `SAR_TOUCH_APPROACH_PAD0` writer - indicate which pad is approach pad0
pub type SAR_TOUCH_APPROACH_PAD0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:14 - touch controller output enable
    #[inline(always)]
    pub fn sar_touch_outen(&self) -> SAR_TOUCH_OUTEN_R {
        SAR_TOUCH_OUTEN_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 16:17 - 3: smooth data 2: baseline 1,0: raw_data
    #[inline(always)]
    pub fn sar_touch_data_sel(&self) -> SAR_TOUCH_DATA_SEL_R {
        SAR_TOUCH_DATA_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - touch_denoise_done
    #[inline(always)]
    pub fn sar_touch_denoise_end(&self) -> SAR_TOUCH_DENOISE_END_R {
        SAR_TOUCH_DENOISE_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - touch_unit_done
    #[inline(always)]
    pub fn sar_touch_unit_end(&self) -> SAR_TOUCH_UNIT_END_R {
        SAR_TOUCH_UNIT_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:23 - indicate which pad is approach pad2
    #[inline(always)]
    pub fn sar_touch_approach_pad2(&self) -> SAR_TOUCH_APPROACH_PAD2_R {
        SAR_TOUCH_APPROACH_PAD2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - indicate which pad is approach pad1
    #[inline(always)]
    pub fn sar_touch_approach_pad1(&self) -> SAR_TOUCH_APPROACH_PAD1_R {
        SAR_TOUCH_APPROACH_PAD1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - indicate which pad is approach pad0
    #[inline(always)]
    pub fn sar_touch_approach_pad0(&self) -> SAR_TOUCH_APPROACH_PAD0_R {
        SAR_TOUCH_APPROACH_PAD0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CONF")
            .field("sar_touch_outen", &self.sar_touch_outen())
            .field("sar_touch_data_sel", &self.sar_touch_data_sel())
            .field("sar_touch_denoise_end", &self.sar_touch_denoise_end())
            .field("sar_touch_unit_end", &self.sar_touch_unit_end())
            .field("sar_touch_approach_pad2", &self.sar_touch_approach_pad2())
            .field("sar_touch_approach_pad1", &self.sar_touch_approach_pad1())
            .field("sar_touch_approach_pad0", &self.sar_touch_approach_pad0())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - touch controller output enable
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_outen(&mut self) -> SAR_TOUCH_OUTEN_W<SAR_TOUCH_CONF_SPEC> {
        SAR_TOUCH_OUTEN_W::new(self, 0)
    }
    ///Bit 15 - clear all touch active status
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_status_clr(&mut self) -> SAR_TOUCH_STATUS_CLR_W<SAR_TOUCH_CONF_SPEC> {
        SAR_TOUCH_STATUS_CLR_W::new(self, 15)
    }
    ///Bits 16:17 - 3: smooth data 2: baseline 1,0: raw_data
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_data_sel(&mut self) -> SAR_TOUCH_DATA_SEL_W<SAR_TOUCH_CONF_SPEC> {
        SAR_TOUCH_DATA_SEL_W::new(self, 16)
    }
    ///Bits 20:23 - indicate which pad is approach pad2
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_approach_pad2(&mut self) -> SAR_TOUCH_APPROACH_PAD2_W<SAR_TOUCH_CONF_SPEC> {
        SAR_TOUCH_APPROACH_PAD2_W::new(self, 20)
    }
    ///Bits 24:27 - indicate which pad is approach pad1
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_approach_pad1(&mut self) -> SAR_TOUCH_APPROACH_PAD1_W<SAR_TOUCH_CONF_SPEC> {
        SAR_TOUCH_APPROACH_PAD1_W::new(self, 24)
    }
    ///Bits 28:31 - indicate which pad is approach pad0
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_approach_pad0(&mut self) -> SAR_TOUCH_APPROACH_PAD0_W<SAR_TOUCH_CONF_SPEC> {
        SAR_TOUCH_APPROACH_PAD0_W::new(self, 28)
    }
}
/**configure touch controller

You can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TOUCH_CONF_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_touch_conf::R`](R) reader structure
impl crate::Readable for SAR_TOUCH_CONF_SPEC {}
///`write(|w| ..)` method takes [`sar_touch_conf::W`](W) writer structure
impl crate::Writable for SAR_TOUCH_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_TOUCH_CONF to value 0xfff0_7fff
impl crate::Resettable for SAR_TOUCH_CONF_SPEC {
    const RESET_VALUE: u32 = 0xfff0_7fff;
}

#[doc = "Register `SAR2_PATT_TAB3` reader"]
pub type R = crate::R<SAR2_PATT_TAB3_SPEC>;
#[doc = "Register `SAR2_PATT_TAB3` writer"]
pub type W = crate::W<SAR2_PATT_TAB3_SPEC>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ITEM11_ATTEN {
    #[doc = "0: No input attenuation, ADC can measure up to approx."]
    Db0   = 0,
    #[doc = "1: The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    Db2_5 = 1,
    #[doc = "2: The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    Db6   = 2,
    #[doc = "3: The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    Db12  = 3,
}
impl From<ITEM11_ATTEN> for u8 {
    #[inline(always)]
    fn from(variant: ITEM11_ATTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ITEM11_ATTEN {
    type Ux = u8;
}
impl crate::IsEnum for ITEM11_ATTEN {}
#[doc = "Field `ITEM11_ATTEN` reader - "]
pub type ITEM11_ATTEN_R = crate::FieldReader<ITEM11_ATTEN>;
impl ITEM11_ATTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITEM11_ATTEN {
        match self.bits {
            0 => ITEM11_ATTEN::Db0,
            1 => ITEM11_ATTEN::Db2_5,
            2 => ITEM11_ATTEN::Db6,
            3 => ITEM11_ATTEN::Db12,
            _ => unreachable!(),
        }
    }
    #[doc = "No input attenuation, ADC can measure up to approx."]
    #[inline(always)]
    pub fn is_db0(&self) -> bool {
        *self == ITEM11_ATTEN::Db0
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    #[inline(always)]
    pub fn is_db2_5(&self) -> bool {
        *self == ITEM11_ATTEN::Db2_5
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    #[inline(always)]
    pub fn is_db6(&self) -> bool {
        *self == ITEM11_ATTEN::Db6
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    #[inline(always)]
    pub fn is_db12(&self) -> bool {
        *self == ITEM11_ATTEN::Db12
    }
}
#[doc = "Field `ITEM11_ATTEN` writer - "]
pub type ITEM11_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ITEM11_ATTEN, crate::Safe>;
impl<'a, REG> ITEM11_ATTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No input attenuation, ADC can measure up to approx."]
    #[inline(always)]
    pub fn db0(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM11_ATTEN::Db0)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    #[inline(always)]
    pub fn db2_5(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM11_ATTEN::Db2_5)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    #[inline(always)]
    pub fn db6(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM11_ATTEN::Db6)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    #[inline(always)]
    pub fn db12(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM11_ATTEN::Db12)
    }
}
#[doc = "Field `ITEM11_CHANNEL` reader - "]
pub type ITEM11_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM11_CHANNEL` writer - "]
pub type ITEM11_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ITEM10_ATTEN` reader - "]
pub use ITEM11_ATTEN_R as ITEM10_ATTEN_R;
#[doc = "Field `ITEM10_ATTEN` writer - "]
pub use ITEM11_ATTEN_W as ITEM10_ATTEN_W;
#[doc = "Field `ITEM10_CHANNEL` reader - "]
pub type ITEM10_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM10_CHANNEL` writer - "]
pub type ITEM10_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ITEM9_ATTEN` reader - "]
pub use ITEM11_ATTEN_R as ITEM9_ATTEN_R;
#[doc = "Field `ITEM9_ATTEN` writer - "]
pub use ITEM11_ATTEN_W as ITEM9_ATTEN_W;
#[doc = "Field `ITEM9_CHANNEL` reader - "]
pub type ITEM9_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM9_CHANNEL` writer - "]
pub type ITEM9_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ITEM8_ATTEN` reader - "]
pub use ITEM11_ATTEN_R as ITEM8_ATTEN_R;
#[doc = "Field `ITEM8_ATTEN` writer - "]
pub use ITEM11_ATTEN_W as ITEM8_ATTEN_W;
#[doc = "Field `ITEM8_CHANNEL` reader - "]
pub type ITEM8_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM8_CHANNEL` writer - "]
pub type ITEM8_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn item11_atten(&self) -> ITEM11_ATTEN_R {
        ITEM11_ATTEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn item11_channel(&self) -> ITEM11_CHANNEL_R {
        ITEM11_CHANNEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn item10_atten(&self) -> ITEM10_ATTEN_R {
        ITEM10_ATTEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn item10_channel(&self) -> ITEM10_CHANNEL_R {
        ITEM10_CHANNEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn item9_atten(&self) -> ITEM9_ATTEN_R {
        ITEM9_ATTEN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn item9_channel(&self) -> ITEM9_CHANNEL_R {
        ITEM9_CHANNEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn item8_atten(&self) -> ITEM8_ATTEN_R {
        ITEM8_ATTEN_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn item8_channel(&self) -> ITEM8_CHANNEL_R {
        ITEM8_CHANNEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2_PATT_TAB3")
            .field("item11_atten", &self.item11_atten())
            .field("item8_atten", &self.item8_atten())
            .field("item8_channel", &self.item8_channel())
            .field("item9_atten", &self.item9_atten())
            .field("item9_channel", &self.item9_channel())
            .field("item10_atten", &self.item10_atten())
            .field("item10_channel", &self.item10_channel())
            .field("item11_channel", &self.item11_channel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn item11_atten(&mut self) -> ITEM11_ATTEN_W<'_, SAR2_PATT_TAB3_SPEC> {
        ITEM11_ATTEN_W::new(self, 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn item11_channel(&mut self) -> ITEM11_CHANNEL_W<'_, SAR2_PATT_TAB3_SPEC> {
        ITEM11_CHANNEL_W::new(self, 2)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn item10_atten(&mut self) -> ITEM10_ATTEN_W<'_, SAR2_PATT_TAB3_SPEC> {
        ITEM10_ATTEN_W::new(self, 6)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn item10_channel(&mut self) -> ITEM10_CHANNEL_W<'_, SAR2_PATT_TAB3_SPEC> {
        ITEM10_CHANNEL_W::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn item9_atten(&mut self) -> ITEM9_ATTEN_W<'_, SAR2_PATT_TAB3_SPEC> {
        ITEM9_ATTEN_W::new(self, 12)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn item9_channel(&mut self) -> ITEM9_CHANNEL_W<'_, SAR2_PATT_TAB3_SPEC> {
        ITEM9_CHANNEL_W::new(self, 14)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn item8_atten(&mut self) -> ITEM8_ATTEN_W<'_, SAR2_PATT_TAB3_SPEC> {
        ITEM8_ATTEN_W::new(self, 18)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn item8_channel(&mut self) -> ITEM8_CHANNEL_W<'_, SAR2_PATT_TAB3_SPEC> {
        ITEM8_CHANNEL_W::new(self, 20)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR2_PATT_TAB3_SPEC;
impl crate::RegisterSpec for SAR2_PATT_TAB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2_patt_tab3::R`](R) reader structure"]
impl crate::Readable for SAR2_PATT_TAB3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar2_patt_tab3::W`](W) writer structure"]
impl crate::Writable for SAR2_PATT_TAB3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR2_PATT_TAB3 to value 0"]
impl crate::Resettable for SAR2_PATT_TAB3_SPEC {}

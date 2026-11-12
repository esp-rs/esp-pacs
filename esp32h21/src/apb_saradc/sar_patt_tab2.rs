#[doc = "Register `SAR_PATT_TAB2` reader"]
pub type R = crate::R<SAR_PATT_TAB2_SPEC>;
#[doc = "Register `SAR_PATT_TAB2` writer"]
pub type W = crate::W<SAR_PATT_TAB2_SPEC>;
#[doc = "\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ITEM7_ATTEN {
    #[doc = "0: No input attenuation, ADC can measure up to approx."]
    Db0   = 0,
    #[doc = "1: The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    Db2_5 = 1,
    #[doc = "2: The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    Db6   = 2,
    #[doc = "3: The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    Db12  = 3,
}
impl From<ITEM7_ATTEN> for u8 {
    #[inline(always)]
    fn from(variant: ITEM7_ATTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ITEM7_ATTEN {
    type Ux = u8;
}
impl crate::IsEnum for ITEM7_ATTEN {}
#[doc = "Field `ITEM7_ATTEN` reader - "]
pub type ITEM7_ATTEN_R = crate::FieldReader<ITEM7_ATTEN>;
impl ITEM7_ATTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITEM7_ATTEN {
        match self.bits {
            0 => ITEM7_ATTEN::Db0,
            1 => ITEM7_ATTEN::Db2_5,
            2 => ITEM7_ATTEN::Db6,
            3 => ITEM7_ATTEN::Db12,
            _ => unreachable!(),
        }
    }
    #[doc = "No input attenuation, ADC can measure up to approx."]
    #[inline(always)]
    pub fn is_db0(&self) -> bool {
        *self == ITEM7_ATTEN::Db0
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    #[inline(always)]
    pub fn is_db2_5(&self) -> bool {
        *self == ITEM7_ATTEN::Db2_5
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    #[inline(always)]
    pub fn is_db6(&self) -> bool {
        *self == ITEM7_ATTEN::Db6
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    #[inline(always)]
    pub fn is_db12(&self) -> bool {
        *self == ITEM7_ATTEN::Db12
    }
}
#[doc = "Field `ITEM7_ATTEN` writer - "]
pub type ITEM7_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ITEM7_ATTEN, crate::Safe>;
impl<'a, REG> ITEM7_ATTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No input attenuation, ADC can measure up to approx."]
    #[inline(always)]
    pub fn db0(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM7_ATTEN::Db0)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    #[inline(always)]
    pub fn db2_5(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM7_ATTEN::Db2_5)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    #[inline(always)]
    pub fn db6(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM7_ATTEN::Db6)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    #[inline(always)]
    pub fn db12(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM7_ATTEN::Db12)
    }
}
#[doc = "Field `ITEM7_CHANNEL` reader - "]
pub type ITEM7_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM7_CHANNEL` writer - "]
pub type ITEM7_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITEM7_UNIT {
    #[doc = "0: SAR ADC 1"]
    Adc1 = 0,
    #[doc = "1: SAR ADC 2"]
    Adc2 = 1,
}
impl From<ITEM7_UNIT> for bool {
    #[inline(always)]
    fn from(variant: ITEM7_UNIT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITEM7_UNIT` reader - "]
pub type ITEM7_UNIT_R = crate::BitReader<ITEM7_UNIT>;
impl ITEM7_UNIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITEM7_UNIT {
        match self.bits {
            false => ITEM7_UNIT::Adc1,
            true => ITEM7_UNIT::Adc2,
        }
    }
    #[doc = "SAR ADC 1"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == ITEM7_UNIT::Adc1
    }
    #[doc = "SAR ADC 2"]
    #[inline(always)]
    pub fn is_adc2(&self) -> bool {
        *self == ITEM7_UNIT::Adc2
    }
}
#[doc = "Field `ITEM7_UNIT` writer - "]
pub type ITEM7_UNIT_W<'a, REG> = crate::BitWriter<'a, REG, ITEM7_UNIT>;
impl<'a, REG> ITEM7_UNIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAR ADC 1"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM7_UNIT::Adc1)
    }
    #[doc = "SAR ADC 2"]
    #[inline(always)]
    pub fn adc2(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM7_UNIT::Adc2)
    }
}
#[doc = "Field `ITEM6_ATTEN` reader - "]
pub use ITEM7_ATTEN_R as ITEM6_ATTEN_R;
#[doc = "Field `ITEM6_ATTEN` writer - "]
pub use ITEM7_ATTEN_W as ITEM6_ATTEN_W;
#[doc = "Field `ITEM6_CHANNEL` reader - "]
pub type ITEM6_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM6_CHANNEL` writer - "]
pub type ITEM6_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITEM5_ATTEN` reader - "]
pub use ITEM7_ATTEN_R as ITEM5_ATTEN_R;
#[doc = "Field `ITEM5_ATTEN` writer - "]
pub use ITEM7_ATTEN_W as ITEM5_ATTEN_W;
#[doc = "Field `ITEM6_UNIT` reader - "]
pub use ITEM7_UNIT_R as ITEM6_UNIT_R;
#[doc = "Field `ITEM6_UNIT` writer - "]
pub use ITEM7_UNIT_W as ITEM6_UNIT_W;
#[doc = "Field `ITEM5_CHANNEL` reader - "]
pub type ITEM5_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM5_CHANNEL` writer - "]
pub type ITEM5_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITEM4_ATTEN` reader - "]
pub use ITEM7_ATTEN_R as ITEM4_ATTEN_R;
#[doc = "Field `ITEM4_ATTEN` writer - "]
pub use ITEM7_ATTEN_W as ITEM4_ATTEN_W;
#[doc = "Field `ITEM5_UNIT` reader - "]
pub use ITEM7_UNIT_R as ITEM5_UNIT_R;
#[doc = "Field `ITEM5_UNIT` writer - "]
pub use ITEM7_UNIT_W as ITEM5_UNIT_W;
#[doc = "Field `ITEM4_CHANNEL` reader - "]
pub type ITEM4_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM4_CHANNEL` writer - "]
pub type ITEM4_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITEM4_UNIT` reader - "]
pub use ITEM7_UNIT_R as ITEM4_UNIT_R;
#[doc = "Field `ITEM4_UNIT` writer - "]
pub use ITEM7_UNIT_W as ITEM4_UNIT_W;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn item7_atten(&self) -> ITEM7_ATTEN_R {
        ITEM7_ATTEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn item7_channel(&self) -> ITEM7_CHANNEL_R {
        ITEM7_CHANNEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn item7_unit(&self) -> ITEM7_UNIT_R {
        ITEM7_UNIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn item6_atten(&self) -> ITEM6_ATTEN_R {
        ITEM6_ATTEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn item6_channel(&self) -> ITEM6_CHANNEL_R {
        ITEM6_CHANNEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn item6_unit(&self) -> ITEM6_UNIT_R {
        ITEM6_UNIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn item5_atten(&self) -> ITEM5_ATTEN_R {
        ITEM5_ATTEN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn item5_channel(&self) -> ITEM5_CHANNEL_R {
        ITEM5_CHANNEL_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn item5_unit(&self) -> ITEM5_UNIT_R {
        ITEM5_UNIT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn item4_atten(&self) -> ITEM4_ATTEN_R {
        ITEM4_ATTEN_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn item4_channel(&self) -> ITEM4_CHANNEL_R {
        ITEM4_CHANNEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn item4_unit(&self) -> ITEM4_UNIT_R {
        ITEM4_UNIT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_PATT_TAB2")
            .field("item7_atten", &self.item7_atten())
            .field("item4_atten", &self.item4_atten())
            .field("item4_channel", &self.item4_channel())
            .field("item7_unit", &self.item7_unit())
            .field("item4_unit", &self.item4_unit())
            .field("item5_atten", &self.item5_atten())
            .field("item5_channel", &self.item5_channel())
            .field("item5_unit", &self.item5_unit())
            .field("item6_atten", &self.item6_atten())
            .field("item6_channel", &self.item6_channel())
            .field("item6_unit", &self.item6_unit())
            .field("item7_channel", &self.item7_channel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn item7_atten(&mut self) -> ITEM7_ATTEN_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM7_ATTEN_W::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn item7_channel(&mut self) -> ITEM7_CHANNEL_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM7_CHANNEL_W::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn item7_unit(&mut self) -> ITEM7_UNIT_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM7_UNIT_W::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn item6_atten(&mut self) -> ITEM6_ATTEN_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM6_ATTEN_W::new(self, 6)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn item6_channel(&mut self) -> ITEM6_CHANNEL_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM6_CHANNEL_W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn item6_unit(&mut self) -> ITEM6_UNIT_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM6_UNIT_W::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn item5_atten(&mut self) -> ITEM5_ATTEN_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM5_ATTEN_W::new(self, 12)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn item5_channel(&mut self) -> ITEM5_CHANNEL_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM5_CHANNEL_W::new(self, 14)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn item5_unit(&mut self) -> ITEM5_UNIT_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM5_UNIT_W::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn item4_atten(&mut self) -> ITEM4_ATTEN_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM4_ATTEN_W::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn item4_channel(&mut self) -> ITEM4_CHANNEL_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM4_CHANNEL_W::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn item4_unit(&mut self) -> ITEM4_UNIT_W<'_, SAR_PATT_TAB2_SPEC> {
        ITEM4_UNIT_W::new(self, 23)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_patt_tab2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_patt_tab2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_PATT_TAB2_SPEC;
impl crate::RegisterSpec for SAR_PATT_TAB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_patt_tab2::R`](R) reader structure"]
impl crate::Readable for SAR_PATT_TAB2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_patt_tab2::W`](W) writer structure"]
impl crate::Writable for SAR_PATT_TAB2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_PATT_TAB2 to value 0x00ff_ffff"]
impl crate::Resettable for SAR_PATT_TAB2_SPEC {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}

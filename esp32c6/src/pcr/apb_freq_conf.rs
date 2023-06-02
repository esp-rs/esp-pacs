#[doc = "Register `APB_FREQ_CONF` reader"]
pub struct R(crate::R<APB_FREQ_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_FREQ_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_FREQ_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_FREQ_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_FREQ_CONF` writer"]
pub struct W(crate::W<APB_FREQ_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_FREQ_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APB_FREQ_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_FREQ_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_DECREASE_DIV_NUM` reader - If this field's value is grater than PCR_APB_DIV_NUM, the clk_apb will be automatically down to clk_apb_decrease only when no access is on apb-bus, and will recover to the previous frequency when a new access appears on apb-bus. Set as one within (0,1,3) to set clk_apb_decrease as div1/div2/div4(default) of clk_ahb. Note that enable this function will reduce performance. Users can set this field as zero to disable the auto-decrease-apb-freq function. By default, this function is disable."]
pub type APB_DECREASE_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `APB_DECREASE_DIV_NUM` writer - If this field's value is grater than PCR_APB_DIV_NUM, the clk_apb will be automatically down to clk_apb_decrease only when no access is on apb-bus, and will recover to the previous frequency when a new access appears on apb-bus. Set as one within (0,1,3) to set clk_apb_decrease as div1/div2/div4(default) of clk_ahb. Note that enable this function will reduce performance. Users can set this field as zero to disable the auto-decrease-apb-freq function. By default, this function is disable."]
pub type APB_DECREASE_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, APB_FREQ_CONF_SPEC, 8, O>;
#[doc = "Field `APB_DIV_NUM` reader - Set as one within (0,1,3) to generate clk_apb drived by clk_ahb. The clk_apb is div1(default)/div2/div4 of clk_ahb."]
pub type APB_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `APB_DIV_NUM` writer - Set as one within (0,1,3) to generate clk_apb drived by clk_ahb. The clk_apb is div1(default)/div2/div4 of clk_ahb."]
pub type APB_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, APB_FREQ_CONF_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - If this field's value is grater than PCR_APB_DIV_NUM, the clk_apb will be automatically down to clk_apb_decrease only when no access is on apb-bus, and will recover to the previous frequency when a new access appears on apb-bus. Set as one within (0,1,3) to set clk_apb_decrease as div1/div2/div4(default) of clk_ahb. Note that enable this function will reduce performance. Users can set this field as zero to disable the auto-decrease-apb-freq function. By default, this function is disable."]
    #[inline(always)]
    pub fn apb_decrease_div_num(&self) -> APB_DECREASE_DIV_NUM_R {
        APB_DECREASE_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Set as one within (0,1,3) to generate clk_apb drived by clk_ahb. The clk_apb is div1(default)/div2/div4 of clk_ahb."]
    #[inline(always)]
    pub fn apb_div_num(&self) -> APB_DIV_NUM_R {
        APB_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_FREQ_CONF")
            .field(
                "apb_decrease_div_num",
                &format_args!("{}", self.apb_decrease_div_num().bits()),
            )
            .field(
                "apb_div_num",
                &format_args!("{}", self.apb_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_FREQ_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - If this field's value is grater than PCR_APB_DIV_NUM, the clk_apb will be automatically down to clk_apb_decrease only when no access is on apb-bus, and will recover to the previous frequency when a new access appears on apb-bus. Set as one within (0,1,3) to set clk_apb_decrease as div1/div2/div4(default) of clk_ahb. Note that enable this function will reduce performance. Users can set this field as zero to disable the auto-decrease-apb-freq function. By default, this function is disable."]
    #[inline(always)]
    #[must_use]
    pub fn apb_decrease_div_num(&mut self) -> APB_DECREASE_DIV_NUM_W<0> {
        APB_DECREASE_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 8:15 - Set as one within (0,1,3) to generate clk_apb drived by clk_ahb. The clk_apb is div1(default)/div2/div4 of clk_ahb."]
    #[inline(always)]
    #[must_use]
    pub fn apb_div_num(&mut self) -> APB_DIV_NUM_W<8> {
        APB_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_FREQ configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_freq_conf](index.html) module"]
pub struct APB_FREQ_CONF_SPEC;
impl crate::RegisterSpec for APB_FREQ_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_freq_conf::R](R) reader structure"]
impl crate::Readable for APB_FREQ_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_freq_conf::W](W) writer structure"]
impl crate::Writable for APB_FREQ_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_FREQ_CONF to value 0"]
impl crate::Resettable for APB_FREQ_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

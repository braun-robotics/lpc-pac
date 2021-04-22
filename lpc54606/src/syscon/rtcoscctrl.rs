#[doc = "Register `RTCOSCCTRL` reader"]
pub struct R(crate::R<RTCOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTCOSCCTRL_SPEC>> for R {
    fn from(reader: crate::R<RTCOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCOSCCTRL` writer"]
pub struct W(crate::W<RTCOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCOSCCTRL_SPEC>;
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
impl core::convert::From<crate::W<RTCOSCCTRL_SPEC>> for W {
    fn from(writer: crate::W<RTCOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - RTC 32 kHz clock enable."]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - RTC 32 kHz clock enable."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC 32 kHz clock enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC 32 kHz clock enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC oscillator 32 kHz output control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcoscctrl](index.html) module"]
pub struct RTCOSCCTRL_SPEC;
impl crate::RegisterSpec for RTCOSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcoscctrl::R](R) reader structure"]
impl crate::Readable for RTCOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcoscctrl::W](W) writer structure"]
impl crate::Writable for RTCOSCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCOSCCTRL to value 0x01"]
impl crate::Resettable for RTCOSCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
